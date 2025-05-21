use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// 数据导入任务的状态枚举。
///
/// - `Pending`：等待中，任务尚未开始。
/// - `Running`：进行中，任务正在执行。
/// - `Completed`：已完成，任务成功结束。
/// - `Failed`：失败，任务执行过程中出现错误。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportStatus {
    Pending,
    Running,
    Completed,
    Failed,
}
impl ImportStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => ImportStatus::Pending,
            "running" => ImportStatus::Running,
            "completed" => ImportStatus::Completed,
            "failed" => ImportStatus::Failed,
            _ => ImportStatus::Pending,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ImportStatus::Pending => "pending",
            ImportStatus::Running => "running",
            ImportStatus::Completed => "completed",
            ImportStatus::Failed => "failed",
        }
    }
}

/// 导入任务
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

/// 可用数据
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

// 导入配置
pub struct ImportConfig {
    pub chunk_size: i64,          // 每个块的天数
    pub retry_count: usize,       // 重试次数
    pub retry_delay: u64,         // 重试延迟（秒）
    pub concurrent_chunks: usize, // 并发块数
}
/// 默认配置
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
/// 数据导入
pub struct DataImporter {
    config: ImportConfig,
}
