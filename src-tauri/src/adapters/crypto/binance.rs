use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::market::{Candle, MarketAdapter, Product, Ticker};

pub struct BinanceAdapter {
    client: Client,
    base_url: String,
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl BinanceAdapter {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.binance.com".to_string(),
            api_key,
            api_secret,
        }
    }
}

#[async_trait]
impl MarketAdapter for BinanceAdapter {
    fn name(&self) -> &str {
        "binance"
    }
    
    fn asset_type(&self) -> &str {
        "crypto"
    }
    
    async fn check_connection(&self) -> Result<bool, String> {
        let url = format!("{}/api/v3/ping", self.base_url);
        
        match self.client.get(&url).send().await {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Connection failed: {}", e)),
        }
    }
    
    async fn get_products(&self) -> Result<Vec<Product>, String> {
        let url = format!("{}/api/v3/exchangeInfo", self.base_url);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch products: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let symbols = &json["symbols"];
        
        if !symbols.is_array() {
            return Err("Invalid response format".to_string());
        }
        
        let mut products = Vec::new();
        
        for symbol in symbols.as_array().unwrap() {
            if symbol["status"].as_str() != Some("TRADING") {
                continue;
            }
            
            let symbol_str = symbol["symbol"].as_str().ok_or("Missing symbol")?;
            let base_asset = symbol["baseAsset"].as_str().ok_or("Missing base asset")?;
            let quote_asset = symbol["quoteAsset"].as_str().ok_or("Missing quote asset")?;
            
            products.push(Product {
                symbol: symbol_str.to_string(),
                name: format!("{}/{}", base_asset, quote_asset),
                asset_type: "crypto".to_string(),
                source: "binance".to_string(),
            });
        }
        
        Ok(products)
    }
    
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker, String> {
        let url = format!("{}/api/v3/ticker/price?symbol={}", self.base_url, symbol);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch ticker: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let price = json["price"]
            .as_str()
            .ok_or("Missing price")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid price: {}", e))?;
        
        let now = Utc::now();
        
        // 获取 24 小时统计数据
        let url_24h = format!("{}/api/v3/ticker/24hr?symbol={}", self.base_url, symbol);
        
        let response_24h = self.client.get(&url_24h)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch 24h stats: {}", e))?;
        
        let json_24h: Value = response_24h.json()
            .await
            .map_err(|e| format!("Failed to parse 24h stats: {}", e))?;
        
        let volume = json_24h["volume"]
            .as_str()
            .ok_or("Missing volume")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid volume: {}", e))?;
        
        let high_24h = json_24h["highPrice"]
            .as_str()
            .ok_or("Missing high price")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid high price: {}", e))?;
        
        let low_24h = json_24h["lowPrice"]
            .as_str()
            .ok_or("Missing low price")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid low price: {}", e))?;
        
        Ok(Ticker::with_details(
            symbol.to_string(),
            price,
            now,
            volume,
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
        // 将 interval 转换为币安的格式
        let binance_interval = match interval {
            "1m" => "1m",
            "5m" => "5m",
            "15m" => "15m",
            "30m" => "30m",
            "1h" => "1h",
            "4h" => "4h",
            "1d" => "1d",
            "1w" => "1w",
            _ => "1d", // 默认使用日线
        };
        
        let start_ms = start_time.timestamp_millis();
        let end_ms = end_time.timestamp_millis();
        
        let url = format!(
            "{}/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}&limit=1000",
            self.base_url, symbol, binance_interval, start_ms, end_ms
        );
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch candles: {}", e))?;
        
        let json: Vec<Vec<Value>> = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut candles = Vec::new();
        
        for kline in json {
            if kline.len() < 6 {
                continue;
            }
            
            let timestamp_ms = kline[0].as_i64().ok_or("Invalid timestamp")?;
            let open = kline[1].as_str().ok_or("Invalid open")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let high = kline[2].as_str().ok_or("Invalid high")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let low = kline[3].as_str().ok_or("Invalid low")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let close = kline[4].as_str().ok_or("Invalid close")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            let volume = kline[5].as_str().ok_or("Invalid volume")?.parse::<f64>().map_err(|e| format!("Parse error: {}", e))?;
            
            let timestamp = Utc.timestamp_millis(timestamp_ms);
            
            candles.push(Candle::new(timestamp, open, high, low, close, volume));
        }
        
        Ok(candles)
    }
}