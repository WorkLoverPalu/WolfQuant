use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTask {
    pub id: String,
    pub asset_type: String,
    pub source: String,
    pub symbol: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub interval: String,
    pub status: ImportStatus,
    pub progress: f64,
    pub error: Option<String>,
    pub total_candles: Option<usize>,
    pub imported_candles: Option<usize>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
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
            source,
            symbol,
            start_time,
            end_time,
            interval,
            status: ImportStatus::Pending,
            progress: 0.0,
            error: None,
            total_candles: None,
            imported_candles: None,
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }
}
