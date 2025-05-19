use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::trading::order::{Order, OrderStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeConfig {
    pub api_key: String,
    pub api_secret: String,
    pub test_mode: bool,
}

#[async_trait]
pub trait Trader: Send + Sync {
    // 获取交易器名称
    fn name(&self) -> &str;
    
    // 获取交易器配置
    fn config(&self) -> &TradeConfig;
    
    // 检查连接状态
    async fn check_connection(&self) -> Result<bool, String>;
    
    // 获取账户余额
    async fn get_balance(&self) -> Result<f64, String>;
    
    // 提交订单
    async fn submit_order(&self, order: &Order) -> Result<String, String>;
    
    // 取消订单
    async fn cancel_order(&self, order_id: &str) -> Result<bool, String>;
    
    // 获取订单状态
    async fn get_order_status(&self, order_id: &str) -> Result<OrderStatus, String>;
    
    // 获取未完成订单
    async fn get_open_orders(&self) -> Result<Vec<Order>, String>;
}