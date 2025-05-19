use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceHistory {
    pub id: i64,
    pub asset_id: i64,
    pub date: i64,
    pub open_price: Option<f64>,
    pub close_price: f64,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    pub volume: Option<f64>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetPriceHistoryRequest {
    pub asset_id: i64,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}
