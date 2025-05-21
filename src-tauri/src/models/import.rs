use crate::models::repository::Repository;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTask {
    pub id: String,
    pub asset_type: String,
    pub symbol: String,
    pub source: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub interval: String,
    pub status: ImportStatus,
    pub progress: f64, // 0.0 - 1.0
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_candles: Option<usize>,
    pub imported_candles: usize,
}

impl ImportTask {
    pub fn new(
        asset_type: String,
        symbol: String,
        source: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            asset_type,
            symbol,
            source,
            start_time,
            end_time,
            interval,
            status: ImportStatus::Pending,
            progress: 0.0,
            error: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
            total_candles: None,
            imported_candles: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableData {
    pub asset_type: String,
    pub symbol: String,
    pub name: Option<String>,
    pub source: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub candle_count: usize,
    pub intervals: Vec<String>,
}

// 可用数据集模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub asset_type: String,
    pub source: String,
    pub symbol: String,
    pub name: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub candle_count: i64,
    pub intervals: Vec<String>,
}

// 导入配置
pub struct ImportConfig {
    pub chunk_size: i64,          // 每个块的天数
    pub retry_count: usize,       // 重试次数
    pub retry_delay: u64,         // 重试延迟（秒）
    pub concurrent_chunks: usize, // 并发块数
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self {
            chunk_size: 30,       // 默认每块30天
            retry_count: 3,       // 默认重试3次
            retry_delay: 5,       // 默认延迟5秒
            concurrent_chunks: 2, // 默认并发2个块
        }
    }
}

pub struct DataImporter {
    repository: Arc<Repository>,
    config: ImportConfig,
}

impl DataImporter {
    pub fn new(repository: Arc<Repository>, config: Option<ImportConfig>) -> Self {
        Self {
            repository,
            config: config.unwrap_or_default(),
        }
    }

    // 启动导入任务
    pub async fn start_import_task(&self, task_id: i64) -> Result<(), String> {
        // 获取任务信息
        let task = self
            .repository
            .get_import_task(task_id)
            .await?
            .ok_or_else(|| format!("Import task not found: {}", task_id))?;

        // 检查任务状态
        if task.status != ImportStatus::Pending {
            return Err(format!("Task is not in pending status: {:?}", task.status));
        }

        // 更新任务状态为进行中
        self.repository
            .update_import_task_status(task_id, ImportStatus::InProgress, 0.0, None)
            .await?;

        // 获取适配器
        let adapter = adapters::get_adapter(&task.asset_type, &task.source)?;

        // 启动异步导入任务
        let repository = self.repository.clone();
        let config = self.config.clone();
        let task_clone = task.clone();

        tokio::spawn(async move {
            let result =
                Self::import_data_chunked(repository.clone(), adapter, &task_clone, config).await;

            match result {
                Ok(_) => {
                    // 导入成功，更新任务状态
                    let _ = repository
                        .update_import_task_status(task_id, ImportStatus::Completed, 100.0, None)
                        .await;
                }
                Err(e) => {
                    // 导入失败，更新任务状态
                    let _ = repository
                        .update_import_task_status(task_id, ImportStatus::Failed, 0.0, Some(&e))
                        .await;
                }
            }
        });

        Ok(())
    }

    // 分块导入数据
    async fn import_data_chunked(
        repository: Arc<Repository>,
        adapter: Box<dyn MarketAdapter>,
        task: &ImportTask,
        config: ImportConfig,
    ) -> Result<(), String> {
        // 计算总时间范围
        let total_duration = task.end_time - task.start_time;
        let total_days = total_duration.num_days();

        // 计算块数
        let chunk_count = (total_days + config.chunk_size - 1) / config.chunk_size;

        // 创建信号通道，用于限制并发数
        let (tx, mut rx) = mpsc::channel(config.concurrent_chunks);

        // 创建任务向量
        let mut chunk_tasks = Vec::new();

        // 为每个块创建导入任务
        for i in 0..chunk_count {
            let start_offset = i * config.chunk_size;
            let end_offset = ((i + 1) * config.chunk_size).min(total_days);

            let chunk_start = task.start_time + Duration::days(start_offset);
            let chunk_end = task.start_time + Duration::days(end_offset);

            // 计算进度百分比
            let progress_start = (i as f64 / chunk_count as f64) * 100.0;
            let progress_end = ((i + 1) as f64 / chunk_count as f64) * 100.0;

            // 克隆必要的变量
            let repository_clone = repository.clone();
            let adapter_clone = adapter.clone();
            let symbol = task.symbol.clone();
            let interval = task.interval.clone();
            let asset_type = task.asset_type.clone();
            let source = task.source.clone();
            let task_id = task.id.unwrap();
            let tx_clone = tx.clone();

            // 创建块导入任务
            let chunk_task = tokio::spawn(async move {
                // 获取令牌，限制并发
                let _permit = tx_clone.send(()).await;

                // 尝试导入数据，支持重试
                for attempt in 0..config.retry_count {
                    match adapter_clone
                        .get_candles(&symbol, chunk_start, chunk_end, &interval)
                        .await
                    {
                        Ok(candles) => {
                            // 为每个K线添加资产类型和数据源
                            let candles_with_meta: Vec<Candle> = candles
                                .into_iter()
                                .map(|mut c| {
                                    c.asset_type = asset_type.clone();
                                    c.source = source.clone();
                                    c.symbol = symbol.clone();
                                    c
                                })
                                .collect();

                            // 保存K线数据
                            match repository_clone
                                .save_candles_with_interval(&candles_with_meta, &interval)
                                .await
                            {
                                Ok(_) => {
                                    // 更新任务进度
                                    let _ = repository_clone
                                        .update_import_task_status(
                                            task_id,
                                            ImportStatus::InProgress,
                                            progress_end,
                                            None,
                                        )
                                        .await;

                                    // 释放令牌
                                    drop(_permit);
                                    return Ok(());
                                }
                                Err(e) => {
                                    if attempt == config.retry_count - 1 {
                                        // 最后一次尝试失败
                                        drop(_permit);
                                        return Err(format!("Failed to save candles: {}", e));
                                    }

                                    // 等待后重试
                                    time::sleep(time::Duration::from_secs(config.retry_delay))
                                        .await;
                                }
                            }
                        }
                        Err(e) => {
                            if attempt == config.retry_count - 1 {
                                // 最后一次尝试失败
                                drop(_permit);
                                return Err(format!("Failed to get candles: {}", e));
                            }

                            // 等待后重试
                            time::sleep(time::Duration::from_secs(config.retry_delay)).await;
                        }
                    }
                }

                // 所有重试都失败
                drop(_permit);
                Err("All retry attempts failed".to_string())
            });

            chunk_tasks.push(chunk_task);
        }

        // 释放发送端，否则接收端永远不会关闭
        drop(tx);

        // 等待所有块完成
        for task in chunk_tasks {
            match task.await {
                Ok(result) => {
                    if let Err(e) = result {
                        return Err(e);
                    }
                }
                Err(e) => {
                    return Err(format!("Chunk task failed: {}", e));
                }
            }
        }

        Ok(())
    }

    // 取消导入任务
    pub async fn cancel_import_task(&self, task_id: i64) -> Result<(), String> {
        // 获取任务信息
        let task = self
            .repository
            .get_import_task(task_id)
            .await?
            .ok_or_else(|| format!("Import task not found: {}", task_id))?;

        // 只有进行中的任务可以取消
        if task.status != ImportStatus::InProgress {
            return Err(format!("Task is not in progress: {:?}", task.status));
        }

        // 更新任务状态为失败
        self.repository
            .update_import_task_status(
                task_id,
                ImportStatus::Failed,
                task.progress,
                Some("Task cancelled by user"),
            )
            .await?;

        Ok(())
    }
}
