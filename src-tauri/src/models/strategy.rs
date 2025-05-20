use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyVersion {
    pub id: i64,
    pub strategy_id: i64,
    pub version: i32,
    pub parameters: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyTag {
    pub id: i64,
    pub name: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStrategyRequest {
    pub name: String,
    pub description: Option<String>,
    pub strategy_type: String,
    pub parameters: String,
    pub is_public: bool,
}

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
    fn check_signal(&self, candle: &crate::models::candle::Candle) -> Result<Option<crate::models::trading::OrderSignal>, String>;
    
    /// 获取策略名称
    fn name(&self) -> &str;
    
    /// 获取策略描述
    fn description(&self) -> Option<&str>;
    
    /// 获取策略参数
    fn parameters(&self) -> HashMap<String, serde_json::Value>;
}
