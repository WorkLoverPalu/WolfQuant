use chrono::{DateTime, Utc};
use tauri::{command, State};
use serde_json::Value;

use crate::backtest::{BacktestConfig, BacktestResult, Backtester};
use crate::core::Engine;
use crate::strategy::StrategyConfig;

#[command]
pub async fn run_backtest(
    strategy_name: String,
    strategy_config: Value,
    symbol: String,
    asset_type: String,
    source: String,
    start_time: String,
    end_time: String,
    backtest_config: BacktestConfig,
    engine: State<'_, Engine>,
) -> Result<BacktestResult, String> {
    let start = DateTime::parse_from_rfc3339(&start_time)
        .map_err(|e| format!("Invalid start time: {}", e))?
        .with_timezone(&Utc);
    
    let end = DateTime::parse_from_rfc3339(&end_time)
        .map_err(|e| format!("Invalid end time: {}", e))?
        .with_timezone(&Utc);
    
    // 获取历史数据
    let candles = engine.get_repository().get_candles(&symbol, &source, start, end).await?;
    
    if candles.is_empty() {
        return Err("No candles found for the specified period".to_string());
    }
    
    // 获取策略
    let strategy = engine.strategies.get(&strategy_name)
        .ok_or_else(|| format!("Strategy not found: {}", strategy_name))?
        .clone();
    
    // 创建回测器
    let mut backtester = Backtester::new(backtest_config, strategy, candles);
    
    // 运行回测
    backtester.run()
}