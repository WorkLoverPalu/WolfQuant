// src/import/importer.rs
use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task;

use crate::adapters;
use crate::core::event::{Event, EventBus, EventData, EventType};
use crate::db::Repository;
use crate::import::models::{ImportStatus, ImportTask};
use crate::market::Candle;

pub struct DataImporter {
    repository: Arc<Repository>,
    event_bus: Arc<EventBus>,
    active_tasks: Arc<tokio::sync::Mutex<Vec<String>>>,
}

impl DataImporter {
    pub fn new(repository: Arc<Repository>, event_bus: Arc<EventBus>) -> Self {
        Self {
            repository,
            event_bus,
            active_tasks: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }
    
    pub async fn start_import(
        &self,
        asset_type: String,
        symbol: String,
        source: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: String,
    ) -> Result<ImportTask, String> {
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
        self.repository.save_import_task(&task).await?;
        
        // 添加到活动任务列表
        let mut active_tasks = self.active_tasks.lock().await;
        active_tasks.push(task.id.clone());
        
        // 启动导入任务
        let task_id = task.id.clone();
        let repository = self.repository.clone();
        let event_bus = self.event_bus.clone();
        let active_tasks = self.active_tasks.clone();
        
        task::spawn(async move {
            let result = Self::import_data(
                task_id.clone(),
                asset_type,
                symbol,
                source,
                start_time,
                end_time,
                interval,
                repository.clone(),
                event_bus.clone(),
            ).await;
            
            // 从活动任务列表中移除
            let mut active_tasks = active_tasks.lock().await;
            if let Some(pos) = active_tasks.iter().position(|id| *id == task_id) {
                active_tasks.remove(pos);
            }
            
            if let Err(e) = result {
                // 更新任务状态为失败
                if let Ok(Some(mut task)) = repository.get_import_task(&task_id).await {
                    task.status = ImportStatus::Failed;
                    task.error = Some(e.clone());
                    task.updated_at = Utc::now();
                    let _ = repository.save_import_task(&task).await;
                    
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
    
    async fn import_data(
        task_id: String,
        asset_type: String,
        symbol: String,
        source: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: String,
        repository: Arc<Repository>,
        event_bus: Arc<EventBus>,
    ) -> Result<(), String> {
        // 获取适配器
        let adapter = adapters::get_adapter(&asset_type, &source)?;
        
        // 更新任务状态为运行中
        if let Ok(Some(mut task)) = repository.get_import_task(&task_id).await {
            task.status = ImportStatus::Running;
            task.updated_at = Utc::now();
            repository.save_import_task(&task).await?;
        }
        
        // 计算时间范围，分割成多个小块
        let chunk_size = match asset_type.as_str() {
            "crypto" => Duration::days(30),  // 加密货币数据量大，每次获取30天
            "stock" => Duration::days(90),   // 股票每次获取90天
            _ => Duration::days(365),        // 其他资产类型每次获取1年
        };
        
        let mut current_start = start_time;
        let mut total_candles = 0;
        let mut imported_candles = 0;
        
        // 估算总K线数量
        let days = (end_time - start_time).num_days();
        let estimated_candles_per_day = match interval.as_str() {
            "1m" => 1440,  // 每分钟一条，一天1440条
            "5m" => 288,   // 每5分钟一条，一天288条
            "15m" => 96,   // 每15分钟一条，一天96条
            "30m" => 48,   // 每30分钟一条，一天48条
            "1h" => 24,    // 每小时一条，一天24条
            "4h" => 6,     // 每4小时一条，一天6条
            "1d" => 1,     // 每天一条
            "1w" => 1/7,   // 每周一条，约等于每天1/7条
            _ => 1,        // 默认每天一条
        };
        
        let estimated_total = (days as f64 * estimated_candles_per_day as f64) as usize;
        
        // 更新任务的总K线数估计
        if let Ok(Some(mut task)) = repository.get_import_task(&task_id).await {
            task.total_candles = Some(estimated_total);
            repository.save_import_task(&task).await?;
        }
        
        // 分块导入数据
        while current_start < end_time {
            let chunk_end = (current_start + chunk_size).min(end_time);
            
            // 获取数据
            let candles = adapter.get_candles(&symbol, current_start, chunk_end, &interval).await?;
            
            // 为每个K线添加symbol和source信息
            let candles_with_info: Vec<Candle> = candles.into_iter()
                .map(|mut c| {
                    c.symbol = symbol.clone();
                    c.source = source.clone();
                    c.interval = interval.clone();
                    c
                })
                .collect();
            
            // 保存到数据库
            repository.save_candles(&candles_with_info).await?;
            
            // 更新进度
            imported_candles += candles_with_info.len();
            let progress = if estimated_total > 0 {
                imported_candles as f64 / estimated_total as f64
            } else {
                0.0
            };
            
            if let Ok(Some(mut task)) = repository.get_import_task(&task_id).await {
                task.progress = progress;
                task.imported_candles = imported_candles;
                task.updated_at = Utc::now();
                repository.save_import_task(&task).await?;
                
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
        if let Ok(Some(mut task)) = repository.get_import_task(&task_id).await {
            task.status = ImportStatus::Completed;
            task.progress = 1.0;
            task.completed_at = Some(Utc::now());
            task.updated_at = Utc::now();
            repository.save_import_task(&task).await?;
            
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
    
    pub async fn get_task(&self, id: &str) -> Result<Option<ImportTask>, String> {
        self.repository.get_import_task(id).await
    }
    
    pub async fn get_tasks(&self) -> Result<Vec<ImportTask>, String> {
        self.repository.get_import_tasks().await
    }
    
    pub async fn get_available_data(&self) -> Result<Vec<AvailableData>, String> {
        self.repository.get_available_data().await
    }
}