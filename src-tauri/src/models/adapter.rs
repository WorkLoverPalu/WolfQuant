// src-tauri/src/market/adapter.rs
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Candle;
use crate::models::Ticker;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
}

#[async_trait]
pub trait MarketAdapter: Send + Sync {
    // 获取适配器名称
    fn name(&self) -> &str;
    
    // 获取适配器类型
    fn asset_type(&self) -> &str;
    
    // 检查连接状态
    async fn check_connection(&self) -> Result<bool, String>;
    
    // 获取产品列表
    async fn get_products(&self) -> Result<Vec<Product>, String>;
    
    // 获取当前价格
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker, String>;
    
    // 获取历史K线数据
    async fn get_candles(
        &self,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: &str,
    ) -> Result<Vec<Candle>, String>;
}