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

impl Ticker {
    pub fn new(symbol: String, price: f64, timestamp: DateTime<Utc>) -> Self {
        Self {
            symbol,
            price,
            timestamp,
            volume: None,
            high_24h: None,
            low_24h: None,
        }
    }
    
    pub fn with_details(
        symbol: String,
        price: f64,
        timestamp: DateTime<Utc>,
        volume: f64,
        high_24h: f64,
        low_24h: f64,
    ) -> Self {
        Self {
            symbol,
            price,
            timestamp,
            volume: Some(volume),
            high_24h: Some(high_24h),
            low_24h: Some(low_24h),
        }
    }
}