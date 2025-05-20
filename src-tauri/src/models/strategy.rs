use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 策略类型枚举，定义了支持的各种策略类型。
/// - `MovingAverageCrossover`：均线交叉策略
/// - `BollingerBands`：布林带策略
/// - `RSI`：相对强弱指数策略
/// - `MACD`：指数平滑异同移动平均线策略
/// - `Custom`：自定义策略
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StrategyType {
    MovingAverageCrossover,
    BollingerBands,
    RSI,
    MACD,
    Custom,
    // 可以添加更多策略类型
}

impl StrategyType {
    /// 根据字符串创建对应的策略类型。
    /// 未知类型将返回 `Custom`。
    pub fn from_str(s: &str) -> Self {
        match s {
            "MovingAverageCrossover" => StrategyType::MovingAverageCrossover,
            "BollingerBands" => StrategyType::BollingerBands,
            "RSI" => StrategyType::RSI,
            "MACD" => StrategyType::MACD,
            "Custom" => StrategyType::Custom,
            _ => StrategyType::Custom,
        }
    }
    // 获取策略类型的字符串表示。
    pub fn to_str(&self) -> &'static str {
        match self {
            StrategyType::MovingAverageCrossover => "MovingAverageCrossover",
            StrategyType::BollingerBands => "BollingerBands",
            StrategyType::RSI => "RSI",
            StrategyType::MACD => "MACD",
            StrategyType::Custom => "Custom",
        }
    }
}
/// 策略结构体，包含策略的基本信息和参数。
/// - `id`：策略ID
/// - `user_id`：所属用户ID
/// - `name`：策略名称
/// - `description`：策略描述
/// - `strategy_type`：策略类型
/// - `parameters`：策略参数（JSON格式）
/// - `is_public`：是否公开
/// - `is_active`：是否激活
/// - `created_at`：创建时间
/// - `updated_at`：更新时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub strategy_type: StrategyType,
    pub parameters: String, // JSON格式的策略参数
    pub is_public: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
/// 策略版本结构体，记录策略的历史版本信息。
/// - `id`：版本ID
/// - `strategy_id`：关联的策略ID
/// - `version`：版本号
/// - `parameters`：版本参数（JSON格式）
/// - `description`：版本描述
/// - `created_at`：创建时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyVersion {
    pub id: i64,
    pub strategy_id: i64,
    pub version: i32,
    pub parameters: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
/// 策略标签结构体，用于对策略进行分类或标记。
/// - `id`：标签ID
/// - `name`：标签名称
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyTag {
    pub id: i64,
    pub name: String,
}
/// 策略评分结构体，记录用户对策略的评分和评价。
/// - `id`：评分ID
/// - `user_id`：用户ID
/// - `strategy_id`：策略ID
/// - `rating`：评分（整数）
/// - `comment`：评价内容
/// - `created_at`：创建时间
/// - `updated_at`：更新时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyRating {
    pub id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
/// 策略应用结构体，记录策略在某个资产上的应用情况。
/// - `id`：应用ID
/// - `user_id`：用户ID
/// - `strategy_id`：策略ID
/// - `asset_id`：资产ID
/// - `is_active`：是否激活
/// - `created_at`：创建时间
/// - `updated_at`：更新时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyApplication {
    pub id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    pub asset_id: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
/// 创建策略请求结构体，用于创建新策略时的数据传输。
/// - `name`：策略名称
/// - `description`：策略描述
/// - `strategy_type`：策略类型（字符串）
/// - `parameters`：策略参数（JSON格式）
/// - `is_public`：是否公开
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStrategyRequest {
    pub name: String,
    pub description: Option<String>,
    pub strategy_type: String,
    pub parameters: String,
    pub is_public: bool,
}
/// 更新策略请求结构体，用于更新已有策略时的数据传输。
/// - `id`：策略ID
/// - `name`：策略名称
/// - `description`：策略描述
/// - `parameters`：策略参数（JSON格式）
/// - `is_public`：是否公开
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStrategyRequest {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub parameters: String,
    pub is_public: bool,
}

/// 策略接口，所有策略实现都需要实现这个trait
pub trait IStrategy {
    /// 初始化策略
    fn init(&self) -> Result<(), String>;

    /// 更新策略状态
    fn update(&self, candle: &crate::models::candle::Candle) -> Result<(), String>;

    /// 检查是否有交易信号
    fn check_signal(
        &self,
        candle: &crate::models::candle::Candle,
    ) -> Result<Option<crate::models::trading::OrderSignal>, String>;

    /// 获取策略名称
    fn name(&self) -> &str;

    /// 获取策略描述
    fn description(&self) -> Option<&str>;

    /// 获取策略参数
    fn parameters(&self) -> HashMap<String, serde_json::Value>;
}
