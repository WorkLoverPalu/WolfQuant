use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub group_name: Option<String>,
    pub asset_type_id: i64,
    pub asset_type_name: String,
    pub code: String,
    pub name: String,
    pub current_price: Option<f64>,//当前价格
    pub position_amount: Option<f64>,//持仓数量
    pub position_cost: Option<f64>,//持仓成本
    pub last_updated: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    // 计算字段
    pub total_profit: Option<f64>,//总利润
    pub total_profit_percent: Option<f64>,//总利润百分比
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub asset_type_id: i64,
    pub code: String,
    pub name: String,
    pub current_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAssetRequest {
    pub id: i64,
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub name: String,
    pub current_price: Option<f64>,
    pub position_amount: Option<f64>,
    pub position_cost: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAssetRequest {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserAssetsRequest {
    pub user_id: i64,
    pub asset_type_id: Option<i64>,
    pub group_id: Option<i64>,
}
