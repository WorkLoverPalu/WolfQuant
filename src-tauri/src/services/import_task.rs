use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Result as SqliteResult};
use log::error;
use crate::error::auth::AuthError;
use crate::database::get_connection_from_pool;
use crate::models::candle::{Candle, CandleModel};
use crate::models::import_task::{DatasetInfo, ImportStatus, ImportTask};

pub struct ImportTaskService;

impl ImportTaskService {
    // 创建导入任务
    pub fn create_import_task(
        &self,
        asset_type: &str,
        source: &str,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: &str,
    ) -> Result<i64, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let now = Utc::now();
        let now_timestamp = now.timestamp();

        let result = conn.execute(
            "INSERT INTO import_tasks (
                asset_type, source, symbol, start_time, end_time, interval,
                status, progress, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                asset_type,
                source,
                symbol,
                start_time.timestamp(),
                end_time.timestamp(),
                interval,
                "pending",
                0.0,
                now_timestamp,
                now_timestamp
            ],
        ).map_err(|e| format!("Failed to create import task: {}", e))?;

        Ok(conn.last_insert_rowid())
    }

    // 更新导入任务状态
    pub fn update_import_task_status(
        &self,
        task_id: i64,
        status: ImportStatus,
        progress: f64,
        error: Option<&str>,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let now = Utc::now();
        let now_timestamp = now.timestamp();
        let status_str = status.to_str();

        let completed_at = if status == ImportStatus::Completed || status == ImportStatus::Failed {
            Some(now_timestamp)
        } else {
            None
        };

        conn.execute(
            "UPDATE import_tasks 
             SET status = ?1, progress = ?2, error = ?3, updated_at = ?4, completed_at = ?5
             WHERE id = ?6",
            params![
                status_str,
                progress,
                error,
                now_timestamp,
                completed_at,
                task_id
            ],
        ).map_err(|e| format!("Failed to update import task: {}", e))?;

        Ok(())
    }

    // 获取导入任务
    pub fn get_import_task(&self, task_id: i64) -> Result<Option<ImportTask>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, asset_type, source, symbol, start_time, end_time, interval, 
                    status, progress, error, created_at, updated_at, completed_at 
             FROM import_tasks WHERE id = ?1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let task_result = stmt.query_row(
            params![task_id],
            |row| {
                Ok(ImportTask {
                    id: row.get(0)?,
                    asset_type: row.get(1)?,
                    source: row.get(2)?,
                    symbol: row.get(3)?,
                    start_time: Utc.timestamp(row.get::<_, i64>(4)?, 0),
                    end_time: Utc.timestamp(row.get::<_, i64>(5)?, 0),
                    interval: row.get(6)?,
                    status: row.get(7)?,
                    progress: row.get(8)?,
                    error: row.get(9)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(10)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(11)?, 0),
                    completed_at: row.get::<_, Option<i64>>(12)?.map(|ts| Utc.timestamp(ts, 0)),
                })
            }
        );
        
        match task_result {
            Ok(task) => Ok(Some(task)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get import task: {}", e)),
        }
    }

    // 获取所有导入任务
    pub fn get_import_tasks(&self) -> Result<Vec<ImportTask>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, asset_type, source, symbol, start_time, end_time, interval, 
                    status, progress, error, created_at, updated_at, completed_at 
             FROM import_tasks ORDER BY created_at DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let task_iter = stmt.query_map(
            params![],
            |row| {
                Ok(ImportTask {
                    id: row.get(0)?,
                    asset_type: row.get(1)?,
                    source: row.get(2)?,
                    symbol: row.get(3)?,
                    start_time: Utc.timestamp(row.get::<_, i64>(4)?, 0),
                    end_time: Utc.timestamp(row.get::<_, i64>(5)?, 0),
                    interval: row.get(6)?,
                    status: row.get(7)?,
                    progress: row.get(8)?,
                    error: row.get(9)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(10)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(11)?, 0),
                    completed_at: row.get::<_, Option<i64>>(12)?.map(|ts| Utc.timestamp(ts, 0)),
                })
            }
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut tasks = Vec::new();
        for task_result in task_iter {
            match task_result {
                Ok(task) => tasks.push(task),
                Err(e) => return Err(format!("Failed to process task row: {}", e)),
            }
        }
        
        Ok(tasks)
    }

    // 获取可用数据集信息
    pub fn get_available_datasets(&self) -> Result<Vec<DatasetInfo>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
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
             ORDER BY c.asset_type, c.source, c.symbol"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let dataset_iter = stmt.query_map(
            params![],
            |row| {
                Ok(DatasetInfo {
                    asset_type: row.get(0)?,
                    source: row.get(1)?,
                    symbol: row.get(2)?,
                    name: row.get(3)?,
                    min_timestamp: row.get(4)?,
                    max_timestamp: row.get(5)?,
                    candle_count: row.get(6)?,
                    intervals: row.get(7)?,
                })
            }
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut datasets = Vec::new();
        for dataset_result in dataset_iter {
            match dataset_result {
                Ok(dataset) => datasets.push(dataset),
                Err(e) => return Err(format!("Failed to process dataset row: {}", e)),
            }
        }
        
        Ok(datasets)
    }

    // 检查数据集是否存在
    pub fn check_dataset_exists(
        &self,
        symbol: &str,
        source: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<bool, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let start_timestamp = start_time.timestamp();
        let end_timestamp = end_time.timestamp();
        
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM candles 
             WHERE symbol = ?1 AND source = ?2 
             AND timestamp BETWEEN ?3 AND ?4",
            params![symbol, source, start_timestamp, end_timestamp],
            |row| row.get(0)
        ).map_err(|e| format!("Failed to check dataset: {}", e))?;
        
        Ok(count > 0)
    }

    // 扩展 Candle 模型以包含 interval 字段
    pub fn save_candle_with_interval(
        &self,
        candle: &Candle,
        interval: &str,
    ) -> Result<i64, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        conn.execute(
            "INSERT INTO candles (symbol, source, asset_type, timestamp, open, high, low, close, volume, interval)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(symbol, source, timestamp, interval) DO UPDATE SET
             open = excluded.open,
             high = excluded.high,
             low = excluded.low,
             close = excluded.close,
             volume = excluded.volume",
            params![
                &candle.symbol,
                &candle.source,
                &candle.asset_type,
                candle.timestamp,
                candle.open,
                candle.high,
                candle.low,
                candle.close,
                candle.volume,
                interval
            ],
        ).map_err(|e| format!("Failed to save candle: {}", e))?;
        
        Ok(conn.last_insert_rowid())
    }

    // 批量保存带有间隔的K线数据
    pub fn save_candles_with_interval(
        &self,
        candles: &[Candle],
        interval: &str,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // 使用事务批量插入以提高性能
        let tx = conn.transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        
        for candle in candles {
            tx.execute(
                "INSERT INTO candles (symbol, source, asset_type, timestamp, open, high, low, close, volume, interval)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                 ON CONFLICT(symbol, source, timestamp, interval) DO UPDATE SET
                 open = excluded.open,
                 high = excluded.high,
                 low = excluded.low,
                 close = excluded.close,
                 volume = excluded.volume",
                params![
                    &candle.symbol,
                    &candle.source,
                    &candle.asset_type,
                    candle.timestamp,
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    candle.volume,
                    interval
                ],
            ).map_err(|e| format!("Failed to save candle in batch: {}", e))?;
        }
        
        tx.commit().map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
        Ok(())
    }

    // 获取带有间隔的K线数据
    pub fn get_candles_with_interval(
        &self,
        symbol: &str,
        source: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: &str,
    ) -> Result<Vec<Candle>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let start_timestamp = start_time.timestamp();
        let end_timestamp = end_time.timestamp();
        
        let mut stmt = conn.prepare(
            "SELECT id, symbol, source, asset_type, timestamp, open, high, low, close, volume, interval 
             FROM candles
             WHERE symbol = ?1 AND source = ?2 AND interval = ?3 AND timestamp BETWEEN ?4 AND ?5
             ORDER BY timestamp ASC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let candle_iter = stmt.query_map(
            params![symbol, source, interval, start_timestamp, end_timestamp],
            |row| {
                Ok(Candle {
                    symbol: row.get(1)?,
                    source: row.get(2)?,
                    asset_type: row.get(3)?,
                    timestamp: row.get(4)?,
                    open: row.get(5)?,
                    high: row.get(6)?,
                    low: row.get(7)?,
                    close: row.get(8)?,
                    volume: row.get(9)?,
                })
            }
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut candles = Vec::new();
        for candle_result in candle_iter {
            match candle_result {
                Ok(candle) => candles.push(candle),
                Err(e) => return Err(format!("Failed to process candle row: {}", e)),
            }
        }
        
        Ok(candles)
    }
}
