
/// 资产结构体，表示用户持有的某一资产的信息。
/// 
/// 字段说明：
/// - `id`: 资产ID
/// - `user_id`: 用户ID
/// - `group_id`: 分组ID（可选）
/// - `group_name`: 分组名称（可选）
/// - `asset_type_id`: 资产类型ID
/// - `asset_type_name`: 资产类型名称
/// - `code`: 资产代码
/// - `name`: 资产名称
/// - `current_price`: 当前价格（可选）
/// - `position_amount`: 持仓数量（可选）
/// - `position_cost`: 持仓成本（可选）
/// - `last_updated`: 最后更新时间（可选）
/// - `created_at`: 创建时间
/// - `updated_at`: 更新时间
/// - `total_profit`: 总利润（可选，计算字段）
/// - `total_profit_percent`: 总利润百分比（可选，计算字段）
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

/// 创建资产请求结构体。
/// 
/// 字段说明：
/// - `user_id`: 用户ID
/// - `group_id`: 分组ID（可选）
/// - `asset_type_id`: 资产类型ID
/// - `code`: 资产代码
/// - `name`: 资产名称
/// - `current_price`: 当前价格（可选）
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub asset_type_id: i64,
    pub code: String,
    pub name: String,
    pub current_price: Option<f64>,
}


/// 更新资产请求结构体。
/// 
/// 字段说明：
/// - `id`: 资产ID
/// - `user_id`: 用户ID
/// - `group_id`: 分组ID（可选）
/// - `name`: 资产名称
/// - `current_price`: 当前价格（可选）
/// - `position_amount`: 持仓数量（可选）
/// - `position_cost`: 持仓成本（可选）
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

/// 删除资产请求结构体。
/// 
/// 字段说明：
/// - `id`: 资产ID
/// - `user_id`: 用户ID
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAssetRequest {
    pub id: i64,
    pub user_id: i64,
}

/// 获取用户资产请求结构体。
/// 
/// 字段说明：
/// - `user_id`: 用户ID
/// - `asset_type_id`: 资产类型ID（可选）
/// - `group_id`: 分组ID（可选）
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserAssetsRequest {
    pub user_id: i64,
    pub asset_type_id: Option<i64>,
    pub group_id: Option<i64>,
}
