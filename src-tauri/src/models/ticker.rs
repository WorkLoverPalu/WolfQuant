use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
    pub volume: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
}