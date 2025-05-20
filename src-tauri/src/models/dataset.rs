use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub asset_type: String,
    pub source: String,
    pub symbol: String,
    pub name: Option<String>,
    pub min_timestamp: i64,
    pub max_timestamp: i64,
    pub candle_count: i64,
    pub intervals: String,
}
