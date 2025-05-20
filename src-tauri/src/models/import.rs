
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
    pub progress: f64,  // 0.0 - 1.0
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