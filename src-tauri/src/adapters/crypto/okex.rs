/// OKEx 加密货币交易所 API 适配器。
///
/// # 概述
///
/// `OkexAdapter` 实现了 `MarketAdapter` trait，提供以下异步方法：
/// - 检查 API 连通性
/// - 获取可交易产品（现货）
/// - 获取指定交易对的最新行情
/// - 获取指定交易对的历史 K 线（OHLCV）数据
///
/// # 字段说明
/// - `client`：用于 HTTP 请求的 `reqwest::Client` 实例
/// - `base_url`：OKEx API 的基础 URL
/// - `api_key`：可选的 API key（当前未用到，仅为扩展预留）
/// - `api_secret`：可选的 API secret（当前未用到，仅为扩展预留）
///
/// # 方法
/// - `new(api_key, api_secret)`：构造新的 `OkexAdapter` 实例
///
/// # Trait 实现：`MarketAdapter`
/// - `name()`：返回适配器名称（"okex"）
/// - `asset_type()`：返回资产类型（"crypto"）
/// - `check_connection()`：检测 API 是否可用
/// - `get_products()`：获取现货可交易产品列表
/// - `get_ticker(symbol)`：获取指定交易对的最新行情
/// - `get_candles(symbol, start_time, end_time, interval)`：获取指定交易对的历史 K 线数据
///
/// # 说明
/// - 所有 API 请求均为异步
/// - 错误通过 `Result<T, String>` 返回
/// - 当前仅支持现货产品
/// - K 线数据按时间正序返回（从旧到新）
/// - 周期字符串会自动映射为 OKEx 格式
///
/// # 依赖
/// - `async_trait` 用于异步 trait 方法
/// - `chrono` 处理时间
/// - `reqwest` 发送 HTTP 请求
/// - `serde` 和 `serde_json` 解析 JSON
///
/// # 示例
/// ```rust
/// let adapter = OkexAdapter::new(None, None);
/// let products = adapter.get_products().await?;
/// let ticker = adapter.get_ticker("BTC-USDT").await?;
/// let candles = adapter.get_candles("BTC-USDT", start, end, "1h").await?;
/// ```
use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{Candle, MarketAdapter, Product, Ticker};

pub struct OkexAdapter {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl OkexAdapter {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://www.okex.com".to_string(),
            api_key,
            api_secret,
        }
    }
}

#[async_trait]
impl MarketAdapter for OkexAdapter {
    fn name(&self) -> &str {
        "okex"
    }
    
    fn asset_type(&self) -> &str {
        "crypto"
    }
    
    async fn check_connection(&self) -> Result<bool, String> {
        let url = format!("{}/api/v5/public/time", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Connection failed: {}", e)),
        }
    }
    
    async fn get_products(&self) -> Result<Vec<Product>, String> {
        let url = format!("{}/api/v5/public/instruments?instType=SPOT", self.base_url);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch products: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let data = &json["data"];
        
        if !data.is_array() {
            return Err("Invalid response format".to_string());
        }
        
        let mut products = Vec::new();
        
        for instrument in data.as_array().unwrap() {
            let symbol = instrument["instId"].as_str().ok_or("Missing instId")?;
            let base_ccy = instrument["baseCcy"].as_str().ok_or("Missing baseCcy")?;
            let quote_ccy = instrument["quoteCcy"].as_str().ok_or("Missing quoteCcy")?;
            
            products.push(Product {
                symbol: symbol.to_string(),
                name: format!("{}/{}", base_ccy, quote_ccy),
                asset_type: "crypto".to_string(),
                source: "okex".to_string(),
            });
        }
        
        Ok(products)
    }
    
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker, String> {
        let url = format!("{}/api/v5/market/ticker?instId={}", self.base_url, symbol);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch ticker: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let data = &json["data"];
        
        if !data.is_array() || data.as_array().unwrap().is_empty() {
            return Err("Invalid response format".to_string());
        }
        
        let ticker_data = &data.as_array().unwrap()[0];
        
        let price = ticker_data["last"]
            .as_str()
            .ok_or("Missing last price")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid price: {}", e))?;
        
        let volume_24h = ticker_data["vol24h"]
            .as_str()
            .ok_or("Missing 24h volume")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid volume: {}", e))?;
        
        let high_24h = ticker_data["high24h"]
            .as_str()
            .ok_or("Missing 24h high")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid high: {}", e))?;
        
        let low_24h = ticker_data["low24h"]
            .as_str()
            .ok_or("Missing 24h low")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid low: {}", e))?;
        
        let timestamp_str = ticker_data["ts"]
            .as_str()
            .ok_or("Missing timestamp")?;
        
        let timestamp_ms = timestamp_str
            .parse::<i64>()
            .map_err(|e| format!("Invalid timestamp: {}", e))?;
        
        let timestamp = Utc.timestamp_millis(timestamp_ms);
        
        Ok(Ticker::with_details(
            symbol.to_string(),
            price,
            timestamp,
            volume_24h,
            high_24h,
            low_24h,
        ))
    }
    
    async fn get_candles(
        &self,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: &str,
    ) -> Result<Vec<Candle>, String> {
        // 将 interval 转换为 OKEx 的格式
        let okex_interval = match interval {
            "1m" => "1m",
            "5m" => "5m",
            "15m" => "15m",
            "30m" => "30m",
            "1h" => "1H",
            "4h" => "4H",
            "1d" => "1D",
            "1w" => "1W",
            _ => "1D", // 默认使用日线
        };
        
        let start_ts = start_time.timestamp();
        let end_ts = end_time.timestamp();
        
        let url = format!(
            "{}/api/v5/market/candles?instId={}&bar={}&before={}&after={}&limit=100",
            self.base_url, symbol, okex_interval, start_ts * 1000, end_ts * 1000
        );
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch candles: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let data = &json["data"];
        
        if !data.is_array() {
            return Err("Invalid response format".to_string());
        }
        
        let mut candles = Vec::new();
        
        for candle_data in data.as_array().unwrap() {
            if !candle_data.is_array() || candle_data.as_array().unwrap().len() < 6 {
                continue;
            }
            
            let candle_array = candle_data.as_array().unwrap();
            
            let timestamp_str = candle_array[0].as_str().ok_or("Missing timestamp")?;
            let timestamp_ms = timestamp_str
                .parse::<i64>()
                .map_err(|e| format!("Invalid timestamp: {}", e))?;
            
            let open = candle_array[1].as_str().ok_or("Missing open")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let high = candle_array[2].as_str().ok_or("Missing high")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let low = candle_array[3].as_str().ok_or("Missing low")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let close = candle_array[4].as_str().ok_or("Missing close")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let volume = candle_array[5].as_str().ok_or("Missing volume")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            
            let timestamp = Utc.timestamp_millis(timestamp_ms);
            
            candles.push(Candle::new(timestamp, open, high, low, close, volume));
        }
        
        // OKEx 返回的数据是倒序的，需要反转
        candles.reverse();
        
        Ok(candles)
    }
}