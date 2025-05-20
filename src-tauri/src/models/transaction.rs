/// 交易记录结构体，表示一条用户的资产交易信息。
/// 
/// 字段说明：
/// - `id`: 交易记录唯一标识
/// - `user_id`: 用户ID
/// - `asset_id`: 资产ID
/// - `asset_name`: 资产名称
/// - `asset_code`: 资产代码
/// - `transaction_type`: 交易类型（如买入、卖出等）
/// - `amount`: 交易数量
/// - `price`: 交易价格
/// - `total_cost`: 交易总金额
/// - `transaction_date`: 交易日期（时间戳）
/// - `notes`: 备注（可选）
/// - `created_at`: 创建时间（时间戳）
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: i64,
    pub user_id: i64,
    pub asset_id: i64,
    pub asset_name: String,
    pub asset_code: String,
    pub transaction_type: String,
    pub amount: f64,
    pub price: f64,
    pub total_cost: f64,
    pub transaction_date: i64,
    pub notes: Option<String>,
    pub created_at: i64,
}
/// 创建交易请求结构体，用于新增一条交易记录。
/// 
/// 字段说明：
/// - `user_id`: 用户ID
/// - `asset_id`: 资产ID
/// - `transaction_type`: 交易类型
/// - `amount`: 交易数量
/// - `price`: 交易价格
/// - `transaction_date`: 交易日期（时间戳）
/// - `notes`: 备注（可选）
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub user_id: i64,
    pub asset_id: i64,
    pub transaction_type: String,
    pub amount: f64,
    pub price: f64,
    pub transaction_date: i64,
    pub notes: Option<String>,
}
/// 更新交易请求结构体，用于修改已有的交易记录。
/// 
/// 字段说明：
/// - `id`: 交易记录唯一标识
/// - `user_id`: 用户ID
/// - `transaction_type`: 交易类型
/// - `amount`: 交易数量
/// - `price`: 交易价格
/// - `transaction_date`: 交易日期（时间戳）
/// - `notes`: 备注（可选）
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransactionRequest {
    pub id: i64,
    pub user_id: i64,
    pub transaction_type: String,
    pub amount: f64,
    pub price: f64,
    pub transaction_date: i64,
    pub notes: Option<String>,
}
/// 删除交易请求结构体，用于删除指定的交易记录。
/// 
/// 字段说明：
/// - `id`: 交易记录唯一标识
/// - `user_id`: 用户ID
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTransactionRequest {
    pub id: i64,
    pub user_id: i64,
}
/// 获取用户交易记录请求结构体，用于查询用户的交易记录。
/// 
/// 字段说明：
/// - `user_id`: 用户ID
/// - `asset_id`: 资产ID（可选）
/// - `start_date`: 查询起始日期（时间戳，可选）
/// - `end_date`: 查询结束日期（时间戳，可选）
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserTransactionsRequest {
    pub user_id: i64,
    pub asset_id: Option<i64>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}
