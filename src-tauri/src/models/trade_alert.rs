/// 交易提醒结构体，表示一条交易相关的提醒信息。
/// 
/// 字段说明：
/// - `id`: 提醒的唯一标识符。
/// - `user_id`: 用户ID，表示该提醒属于哪个用户。
/// - `asset_id`: 资产ID，关联的资产标识。
/// - `asset_name`: 资产名称。
/// - `asset_code`: 资产代码。
/// - `strategy_id`: 策略ID，可选，关联的策略标识。
/// - `strategy_name`: 策略名称，可选，关联的策略名称。
/// - `alert_type`: 提醒类型，例如“买入”、“卖出”等。
/// - `message`: 提醒内容。
/// - `is_read`: 是否已读，布尔值。
/// - `created_at`: 创建时间，时间戳格式。
///
/// 用于序列化和反序列化，支持克隆和调试。
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
