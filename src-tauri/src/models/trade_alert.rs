use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeAlert {
    pub id: i64,
    pub user_id: i64,
    pub asset_id: i64,
    pub asset_name: String,
    pub asset_code: String,
    pub strategy_id: Option<i64>,
    pub strategy_name: Option<String>,
    pub alert_type: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkAlertReadRequest {
    pub id: i64,
    pub user_id: i64,
}
