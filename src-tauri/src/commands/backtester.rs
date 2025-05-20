use tauri::{command, State, AppHandle};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::services::backtester_service::BacktesterService;
use crate::services::candle_service::CandleService;
use crate::services::strategy_service::StrategyService;
use crate::models::backtester::{BacktestConfig, BacktestResult};
use crate::models::strategy::StrategyType;

#[derive(Debug, Deserialize)]
pub struct RunBacktestParams {
    pub strategy_id: i64,
    pub symbol: String,
    pub source: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub interval: String,
    pub initial_capital: f64,
    pub fee_rate: f64,
    pub slippage: f64,
}

#[command]
pub fn run_backtest(
    params: RunBacktestParams,
    app_handle: AppHandle,
) -> Result<BacktestResult, String> {
    // 创建服务实例
    let backtester_service = BacktesterService::new();
    let candle_service = CandleService;
    let strategy_service = StrategyService::new();
    
    // 获取策略
    let strategy = strategy_service.get_strategy(params.strategy_id)?;
    
    // 获取K线数据
    let candles = candle_service.get_candles_with_interval(
        &params.symbol,
        &params.source,
        params.start_time,
        params.end_time,
        &params.interval,
    )?;
    
    if candles.is_empty() {
        return Err("No candle data available for the specified period".to_string());
    }
    
    // 创建策略实例
    let strategy_instance = strategy_service.create_strategy_instance(
        strategy.strategy_type,
        &strategy.parameters,
    )?;
    
    // 创建回测配置
    let config = BacktestConfig {
        initial_capital: params.initial_capital,
        fee_rate: params.fee_rate,
        slippage: params.slippage,
    };
    
    // 运行回测
    let result = backtester_service.run_backtest(config, strategy_instance, candles)?;
    
    // 保存回测结果
    let user_id = 1; // 从应用状态获取当前用户ID
    let backtest_id = backtester_service.save_backtest_result(
        user_id,
        params.strategy_id,
        &result.performance,
        &result.equity_curve,
        &result.trades,
    )?;
    
    Ok(result)
}

#[command]
pub fn get_backtest_result(
    backtest_id: i64,
    app_handle: AppHandle,
) -> Result<Option<BacktestResult>, String> {
    let backtester_service = BacktesterService::new();
    backtester_service.get_backtest_result(backtest_id)
}

#[command]
pub fn get_backtest_results_by_strategy(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<Vec<BacktestResult>, String> {
    let backtester_service = BacktesterService::new();
    backtester_service.get_backtest_results_by_strategy(strategy_id)
}
