use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models::Candle;
use crate::models::OrderSignal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    pub name: String,
    pub params: serde_json::Value,
}

#[async_trait]
pub trait Strategy: Send + Sync {
    // 获取策略名称
    fn name(&self) -> &str;
    
    // 获取策略配置
    fn config(&self) -> &StrategyConfig;
    
    // 初始化策略
    fn init(&mut self) -> Result<(), String>;
    
    // 更新策略状态
    fn update(&mut self, candle: &Candle) -> Result<(), String>;
    
    // 检查是否应该生成交易信号
    fn check_signal(&self, candle: &Candle) -> Result<Option<OrderSignal>, String>;
}