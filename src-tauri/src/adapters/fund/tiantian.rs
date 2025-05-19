use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::market::{Candle, MarketAdapter, Product, Ticker};

pub struct TiantianFundAdapter {
    client: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct TiantianFundResponse {
    #[serde(rename = "fundcode")]
    fund_code: String,
    #[serde(rename = "name")]
    fund_name: String,
    #[serde(rename = "gsz")]
    price: String,
    #[serde(rename = "gszzl")]
    change_percent: String,
    #[serde(rename = "gztime")]
    time: String,
}

#[derive(Debug, Deserialize)]
struct TiantianHistoryResponse {
    #[serde(rename = "Data")]
    data: TiantianHistoryData,
}

#[derive(Debug, Deserialize)]
struct TiantianHistoryData {
    #[serde(rename = "LSJZList")]
    history: Vec<TiantianHistoryItem>,
}

#[derive(Debug, Deserialize)]
struct TiantianHistoryItem {
    #[serde(rename = "FSRQ")]
    date: String,
    #[serde(rename = "DWJZ")]
    nav: String,
    #[serde(rename = "LJJZ")]
    acc_nav: String,
}

impl TiantianFundAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://fundgz.1234567.com.cn".to_string(),
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
impl MarketAdapter for TiantianFundAdapter {
    fn name(&self) -> &str {
        "tiantian"
    }
    
    fn asset_type(&self) -> &str {
        "fund"
    }
    
    async fn check_connection(&self) -> Result<bool, String> {
        match self.client.get(&format!("{}/js/ping", self.base_url)).send().await {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Connection failed: {}", e)),
        }
    }
    
    async fn get_products(&self) -> Result<Vec<Product>, String> {
        // 实际实现中，我们需要从天天基金获取基金列表
        // 这里为了简化，返回一些示例数据
        let products = vec![
            Product {
                symbol: "000001".to_string(),
                name: "华夏成长混合".to_string(),
                asset_type: "fund".to_string(),
                source: "tiantian".to_string(),
            },
            Product {
                symbol: "000002".to_string(),
                name: "华夏优势增长混合".to_string(),
                asset_type: "fund".to_string(),
                source: "tiantian".to_string(),
            },
        ];
        
        Ok(products)
    }
    
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker, String> {
        let url = format!("{}/js/{}.js", self.base_url, symbol);
        
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch ticker: {}", e))?;
        
        let text = response.text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        // 天天基金返回的是 jsonp 格式，需要提取 JSON 部分
        let json_start = text.find('{').ok_or("Invalid response format")?;
        let json_end = text.rfind('}').ok_or("Invalid response format")?;
        let json_str = &text[json_start..=json_end];
        
        let fund_data: TiantianFundResponse = serde_json::from_str(json_str)
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let price = fund_data.price.parse::<f64>()
            .map_err(|e| format!("Failed to parse price: {}", e))?;
        
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
        // 天天基金只提供日级别的数据，忽略 interval 参数
        let start_date = start_time.format("%Y-%m-%d").to_string();
        let end_date = end_time.format("%Y-%m-%d").to_string();
        
        let url = format!(
            "http://api.fund.eastmoney.com/f10/lsjz?fundCode={}&pageIndex=1&pageSize=100&startDate={}&endDate={}",
            symbol, start_date, end_date
        );
        
        let response = self.client.get(&url)
            .header("Referer", "http://fundf10.eastmoney.com/")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch historical data: {}", e))?;
        
        let json: Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let data = &json["Data"]["LSJZList"];
        
        if !data.is_array() {
            return Err("Invalid response format".to_string());
        }
        
        let mut candles = Vec::new();
        
        for item in data.as_array().unwrap() {
            let date_str = item["FSRQ"].as_str().ok_or("Missing date")?;
            let nav_str = item["DWJZ"].as_str().ok_or("Missing NAV")?;
            
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