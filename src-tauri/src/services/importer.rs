use crate::adapters;
use crate::database::get_connection_from_pool;
use crate::error::auth::AuthError;
use crate::event::{Event, EventBus, EventData, EventType};
use crate::models::candle::Candle;
use crate::models::dataset::AvailableData;
use crate::models::import_task::{ImportStatus, ImportTask};
use chrono::{DateTime, Duration, Utc};
use log::{error, info, warn};
use std::sync::Arc;
use std::thread;

/// 数据导入服务，负责管理和执行数据导入任务
pub struct ImporterService {
    event_bus: Arc<EventBus>,
    active_tasks: Arc<std::sync::Mutex<Vec<String>>>,
}

impl ImporterService {
    /// 创建新的导入服务实例
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            event_bus,
            active_tasks: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// 启动数据导入任务
    pub fn start_import(
        &self,
        asset_type: String,
        symbol: String,
        source: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: String,
    ) -> Result<ImportTask, String> {
        info!(
            "Starting import task for {}/{} from {} to {}",
            source, symbol, start_time, end_time
        );

        // 创建导入任务
        let mut task = ImportTask::new(
            asset_type.clone(),
            symbol.clone(),
            source.clone(),
            start_time,
            end_time,
            interval.clone(),
        );

        // 保存任务到数据库
        self.save_import_task(&task)?;

        // 添加到活动任务列表
        let mut active_tasks = self
            .active_tasks
            .lock()
            .map_err(|e| format!("Failed to lock active tasks: {}", e))?;
        active_tasks.push(task.id.clone());

        // 启动导入任务
        let task_id = task.id.clone();
        let event_bus = self.event_bus.clone();
        let active_tasks = self.active_tasks.clone();

        thread::spawn(move || {
            let result = Self::import_data(
                task_id.clone(),
                asset_type,
                symbol,
                source,
                start_time,
                end_time,
                interval,
                event_bus.clone(),
            );

            // 从活动任务列表中移除
            if let Ok(mut active_tasks) = active_tasks.lock() {
                if let Some(pos) = active_tasks.iter().position(|id| *id == task_id) {
                    active_tasks.remove(pos);
                }
            }

            if let Err(e) = result {
                error!("Import task failed: {}", e);

                // 更新任务状态为失败
                let service = ImporterService::new(event_bus.clone());
                if let Ok(Some(mut task)) = service.get_task(&task_id) {
                    task.status = ImportStatus::Failed;
                    task.error = Some(e.clone());
                    task.updated_at = Utc::now();
                    if let Err(e) = service.save_import_task(&task) {
                        error!("Failed to update failed task status: {}", e);
                    }

                    // 发布错误事件
                    let event = Event {
                        event_type: EventType::Error,
                        timestamp: Utc::now(),
                        data: EventData::Error(format!("Import task failed: {}", e)),
                    };
                    event_bus.publish(event);
                }
            }
        });

        Ok(task)
    }

    /// 核心数据导入逻辑
    fn import_data(
        task_id: String,
        asset_type: String,
        symbol: String,
        source: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: String,
        event_bus: Arc<EventBus>,
    ) -> Result<(), String> {
        // 获取适配器
        let adapter = adapters::get_adapter(&asset_type, &source)?;

        // 创建服务实例用于更新任务状态
        let service = ImporterService::new(event_bus.clone());

        // 更新任务状态为运行中
        if let Ok(Some(mut task)) = service.get_task(&task_id) {
            task.status = ImportStatus::Running;
            task.updated_at = Utc::now();
            service.save_import_task(&task)?;
        }

        // 计算时间范围，分割成多个小块
        let chunk_size = match asset_type.as_str() {
            "crypto" => Duration::days(30), // 加密货币数据量大，每次获取30天
            "stock" => Duration::days(90),  // 股票每次获取90天
            _ => Duration::days(365),       // 其他资产类型每次获取1年
        };

        let mut current_start = start_time;
        let mut imported_candles = 0;

        // 估算总K线数量
        let days = (end_time - start_time).num_days();
        let estimated_candles_per_day = match interval.as_str() {
            "1m" => 1440,      // 每分钟一条，一天1440条
            "5m" => 288,       // 每5分钟一条，一天288条
            "15m" => 96,       // 每15分钟一条，一天96条
            "30m" => 48,       // 每30分钟一条，一天48条
            "1h" => 24,        // 每小时一条，一天24条
            "4h" => 6,         // 每4小时一条，一天6条
            "1d" => 1,         // 每天一条
            "1w" => 1.0 / 7.0, // 每周一条，约等于每天1/7条
            _ => 1,            // 默认每天一条
        };

        let estimated_total = (days as f64 * estimated_candles_per_day as f64) as usize;

        // 更新任务的总K线数估计
        if let Ok(Some(mut task)) = service.get_task(&task_id) {
            task.total_candles = Some(estimated_total);
            service.save_import_task(&task)?;
        }

        // 分块导入数据
        while current_start < end_time {
            let chunk_end = (current_start + chunk_size).min(end_time);

            // 获取数据
            let candles = adapter.get_candles(&symbol, current_start, chunk_end, &interval)?;

            // 为每个K线添加symbol和source信息
            let candles_with_info: Vec<Candle> = candles
                .into_iter()
                .map(|mut c| {
                    c.symbol = symbol.clone();
                    c.source = source.clone();
                    c.interval = interval.clone();
                    c
                })
                .collect();

            // 保存到数据库
            service.save_candles(&candles_with_info)?;

            // 更新进度
            imported_candles += candles_with_info.len();
            let progress = if estimated_total > 0 {
                imported_candles as f64 / estimated_total as f64
            } else {
                0.0
            };

            if let Ok(Some(mut task)) = service.get_task(&task_id) {
                task.progress = progress;
                task.imported_candles = Some(imported_candles);
                task.updated_at = Utc::now();
                service.save_import_task(&task)?;

                // 发布进度事件
                let event = Event {
                    event_type: EventType::ImportProgress,
                    timestamp: Utc::now(),
                    data: EventData::ImportProgress(task.clone()),
                };
                event_bus.publish(event);
            }

            // 移动到下一个时间块
            current_start = chunk_end;
        }

        // 更新任务状态为完成
        if let Ok(Some(mut task)) = service.get_task(&task_id) {
            task.status = ImportStatus::Completed;
            task.progress = 1.0;
            task.completed_at = Some(Utc::now());
            task.updated_at = Utc::now();
            service.save_import_task(&task)?;

            // 发布完成事件
            let event = Event {
                event_type: EventType::ImportCompleted,
                timestamp: Utc::now(),
                data: EventData::ImportCompleted(task.clone()),
            };
            event_bus.publish(event);
        }

        Ok(())
    }

    /// 获取导入任务
    pub fn get_task(&self, id: &str) -> Result<Option<ImportTask>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut stmt = conn
            .prepare(
                "SELECT id, asset_type, source, symbol, start_time, end_time, interval, 
                    status, progress, error, total_candles, imported_candles,
                    created_at, updated_at, completed_at 
             FROM import_tasks WHERE id = ?",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let task_result = stmt.query_row(rusqlite::params![id], |row| {
            Ok(ImportTask {
                id: row.get(0)?,
                asset_type: row.get(1)?,
                source: row.get(2)?,
                symbol: row.get(3)?,
                start_time: Utc.timestamp(row.get::<_, i64>(4)?, 0),
                end_time: Utc.timestamp(row.get::<_, i64>(5)?, 0),
                interval: row.get(6)?,
                status: ImportStatus::from_str(&row.get::<_, String>(7)?),
                progress: row.get(8)?,
                error: row.get(9)?,
                total_candles: row.get(10)?,
                imported_candles: row.get(11)?,
                created_at: Utc.timestamp(row.get::<_, i64>(12)?, 0),
                updated_at: Utc.timestamp(row.get::<_, i64>(13)?, 0),
                completed_at: row
                    .get::<_, Option<i64>>(14)?
                    .map(|ts| Utc.timestamp(ts, 0)),
            })
        });

        match task_result {
            Ok(task) => Ok(Some(task)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get import task: {}", e)),
        }
    }

    /// 获取所有导入任务
    pub fn get_tasks(&self) -> Result<Vec<ImportTask>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut stmt = conn
            .prepare(
                "SELECT id, asset_type, source, symbol, start_time, end_time, interval, 
                    status, progress, error, total_candles, imported_candles,
                    created_at, updated_at, completed_at 
             FROM import_tasks ORDER BY created_at DESC",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let task_iter = stmt
            .query_map(rusqlite::params![], |row| {
                Ok(ImportTask {
                    id: row.get(0)?,
                    asset_type: row.get(1)?,
                    source: row.get(2)?,
                    symbol: row.get(3)?,
                    start_time: Utc.timestamp(row.get::<_, i64>(4)?, 0),
                    end_time: Utc.timestamp(row.get::<_, i64>(5)?, 0),
                    interval: row.get(6)?,
                    status: ImportStatus::from_str(&row.get::<_, String>(7)?),
                    progress: row.get(8)?,
                    error: row.get(9)?,
                    total_candles: row.get(10)?,
                    imported_candles: row.get(11)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(12)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(13)?, 0),
                    completed_at: row
                        .get::<_, Option<i64>>(14)?
                        .map(|ts| Utc.timestamp(ts, 0)),
                })
            })
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let mut tasks = Vec::new();
        for task_result in task_iter {
            match task_result {
                Ok(task) => tasks.push(task),
                Err(e) => return Err(format!("Failed to process task row: {}", e)),
            }
        }

        Ok(tasks)
    }

    /// 获取可用数据集信息
    pub fn get_available_datasets(&self) -> Result<Vec<AvailableData>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let mut stmt = conn
            .prepare(
                "SELECT 
                c.asset_type, 
                c.source, 
                c.symbol, 
                p.name,
                MIN(c.timestamp) as min_timestamp, 
                MAX(c.timestamp) as max_timestamp,
                COUNT(*) as candle_count,
                GROUP_CONCAT(DISTINCT c.interval) as intervals
             FROM candles c
             LEFT JOIN products p ON c.symbol = p.symbol AND c.source = p.source
             GROUP BY c.asset_type, c.source, c.symbol
             ORDER BY c.asset_type, c.source, c.symbol",
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let dataset_iter = stmt
            .query_map(rusqlite::params![], |row| {
                Ok(AvailableData {
                    asset_type: row.get(0)?,
                    symbol: row.get(2)?,
                    name: row.get(3)?,
                    source: row.get(1)?,
                    min_timestamp: row.get(4)?,
                    max_timestamp: row.get(5)?,
                    candle_count: row.get(6)?,
                    intervals: row.get(7)?,
                })
            })
            .map_err(|e| format!("Failed to execute query: {}", e))?;

        let mut datasets = Vec::new();
        for dataset_result in dataset_iter {
            match dataset_result {
                Ok(dataset) => datasets.push(dataset),
                Err(e) => return Err(format!("Failed to process dataset row: {}", e)),
            }
        }

        Ok(datasets)
    }

    /// 保存导入任务
    fn save_import_task(&self, task: &ImportTask) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        let status_str = match task.status {
            ImportStatus::Pending => "pending",
            ImportStatus::Running => "running",
            ImportStatus::Completed => "completed",
            ImportStatus::Failed => "failed",
        };

        let completed_at = task.completed_at.map(|dt| dt.timestamp());

        conn.execute(
            "INSERT OR REPLACE INTO import_tasks (
                id, asset_type, source, symbol, start_time, end_time, interval,
                status, progress, error, total_candles, imported_candles,
                created_at, updated_at, completed_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                task.id,
                task.asset_type,
                task.source,
                task.symbol,
                task.start_time.timestamp(),
                task.end_time.timestamp(),
                task.interval,
                status_str,
                task.progress,
                task.error,
                task.total_candles,
                task.imported_candles,
                task.created_at.timestamp(),
                task.updated_at.timestamp(),
                completed_at
            ],
        )
        .map_err(|e| format!("Failed to save import task: {}", e))?;

        Ok(())
    }

    /// 保存K线数据
    fn save_candles(&self, candles: &[Candle]) -> Result<(), String> {
        if candles.is_empty() {
            return Ok(());
        }

        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // 使用事务批量插入以提高性能
        let tx = conn
            .transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;

        for candle in candles {
            tx.execute(
                "INSERT INTO candles (
                    symbol, source, asset_type, timestamp, open, high, low, close, volume, interval
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(symbol, source, timestamp, interval) DO UPDATE SET
                open = excluded.open,
                high = excluded.high,
                low = excluded.low,
                close = excluded.close,
                volume = excluded.volume",
                rusqlite::params![
                    candle.symbol,
                    candle.source,
                    candle.asset_type,
                    candle.timestamp.timestamp(),
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    candle.volume,
                    candle.interval
                ],
            )
            .map_err(|e| format!("Failed to save candle: {}", e))?;
        }

        tx.commit()
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(())
    }
}
