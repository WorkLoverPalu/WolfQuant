use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::market::{Candle, MarketAdapter, Product, Ticker};

pub struct SinaFundAdapter {
    client: Client,
    base_url: String,
}

impl SinaFundAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://stock.finance.sina.com.cn/fundInfo".to_string(),
        }
    }
    
    fn parse_date(&self, date_str: &str) -> Result<DateTime<Utc>, String> {
        let date_format = "%Y-%m-%d";
        chrono::NaiveDate::parse_from_str(date_str, date_format)
            .map_err(|e| format!("Failed to parse date: {}", e))
            .map(|date| Utc.from_utc_datetime(&date.and_hms(0, 0, 0)))
    }
}

#[async_trait]
impl MarketAdapter for SinaFundAdapter {
    fn name(&self) -> &str {
        "sina"
    }
    
    fn asset_type(&self) -> &str {
        "fund"
    }
    
    async fn check_connection(&self) -> Result<bool, String> {
        match self.client.get(&self.base_url).send().await {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Connection failed: {}", e)),
        }
    }
    
    async fn get_products(&self) -> Result<Vec<Product>, String> {
        // 实际实现中，我们需要从新浪基金获取基金列表
        // 这里为了简化，返回一些示例数据
        let products = vec![
            Product {
                symbol: "000001".to_string(),
                name: "华夏成长混合".to_string(),
                asset_type: "fund".to_string(),
                source: "sina".to_string(),
            },
            Product {
                symbol: "000002".to_string(),
                name: "华夏优势增长混合".to_string(),
                asset_type: "fund".to_string(),
                source: "sina".to_string(),
            },
        ];
        
        Ok(products)
    }
    
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker, String> {
        let url = format!("{}/api/fund/get_nav?symbol={}", self.base_url, symbol);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch ticker: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let price = json["data"]["nav"]
            .as_str()
            .ok_or("Missing NAV")?
            .parse::<f64>()
            .map_err(|e| format!("Invalid NAV: {}", e))?;
        
        let now = Utc::now();
        
        Ok(Ticker::new(symbol.to_string(), price, now))
    }
    
    async fn get_candles(
        &self,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        _interval: &str,
    ) -> Result<Vec<Candle>, String> {
        // 新浪基金只提供日级别的数据，忽略 interval 参数
        let start_date = start_time.format("%Y-%m-%d").to_string();
        let end_date = end_time.format("%Y-%m-%d").to_string();
        
        let url = format!(
            "{}/api/fund/history_nav?symbol={}&start_date={}&end_date={}",
            self.base_url, symbol, start_date, end_date
        );
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch historical data: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let data = &json["data"];
        
        if !data.is_array() {
            return Err("Invalid response format".to_string());
        }
        
        let mut candles = Vec::new();
        
        for item in data.as_array().unwrap() {
            let date_str = item["date"].as_str().ok_or("Missing date")?;
            let nav_str = item["nav"].as_str().ok_or("Missing NAV")?;
            
            let timestamp = self.parse_date(date_str)?;
            let nav = nav_str.parse::<f64>().map_err(|e| format!("Invalid NAV: {}", e))?;
            
            // 基金只有净值，没有开高低收，所以我们用相同的值
            let candle = Candle::new(timestamp, nav, nav, nav, nav, 0.0);
            candles.push(candle);
        }
        
        // 按时间排序
        candles.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        Ok(candles)
    }
}