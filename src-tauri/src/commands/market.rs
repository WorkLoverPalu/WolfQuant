use chrono::{DateTime, Utc};
use tauri::{command, State};

use crate::core::Engine;
use crate::market::{Candle, Product, Ticker};

#[command]
pub async fn get_products(
    asset_type: String,
    source: String,
    engine: State<'_, Engine>,
) -> Result<Vec<Product>, String> {
    let key = format!("{}:{}", asset_type, source);
    let adapter = engine.adapters.get(&key)
        .ok_or_else(|| format!("Adapter not found: {}", key))?;
    
    adapter.get_products().await
}

#[command]
pub async fn get_ticker(
    symbol: String,
    asset_type: String,
    source: String,
    engine: State<'_, Engine>,
) -> Result<Ticker, String> {
    let key = format!("{}:{}", asset_type, source);
    let adapter = engine.adapters.get(&key)
        .ok_or_else(|| format!("Adapter not found: {}", key))?;
    
    adapter.get_ticker(&symbol).await
}

#[command]
pub async fn get_candles(
    symbol: String,
    asset_type: String,
    source: String,
    start_time: String,
    end_time: String,
    interval: String,
    engine: State<'_, Engine>,
) -> Result<Vec<Candle>, String> {
    let start = DateTime::parse_from_rfc3339(&start_time)
        .map_err(|e| format!("Invalid start time: {}", e))?
        .with_timezone(&Utc);
    
    let end = DateTime::parse_from_rfc3339(&end_time)
        .map_err(|e| format!("Invalid end time: {}", e))?
        .with_timezone(&Utc);
    
    // 先尝试从数据库获取
    let repository = engine.get_repository();
    let candles = repository.get_candles(&symbol, &source, start, end).await?;
    
    if !candles.is_empty() {
        return Ok(candles);
    }
    
    // 如果数据库中没有，则从适配器获取
    engine.import_historical_data(&symbol, &asset_type, &source, start, end, &interval).await
}

#[command]
pub async fn import_historical_data(
    symbol: String,
    asset_type: String,
    source: String,
    start_time: String,
    end_time: String,
    interval: String,
    engine: State<'_, Engine>,
) -> Result<Vec<Candle>, String> {
    let start = DateTime::parse_from_rfc3339(&start_time)
        .map_err(|e| format!("Invalid start time: {}", e))?
        .with_timezone(&Utc);
    
    let end = DateTime::parse_from_rfc3339(&end_time)
        .map_err(|e| format!("Invalid end time: {}", e))?
        .with_timezone(&Utc);
    
    engine.import_historical_data(&symbol, &asset_type, &source, start, end, &interval).await
}

#[command]
pub async fn start_market_data(
    symbol: String,
    asset_type: String,
    source: String,
    engine: State<'_, Engine>,
) -> Result<(), String> {
    engine.start_market_data(&symbol, &asset_type, &source).await
}

#[command]
pub async fn stop_market_data(
    engine: State<'_, Engine>,
) -> Result<(), String> {
    engine.stop();
    Ok(())
}