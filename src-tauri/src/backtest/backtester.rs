use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::market::Candle;
use crate::strategy::Strategy;
use crate::trading::{Order, OrderSignal, Portfolio};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestConfig {
    pub initial_capital: f64,
    pub fee_rate: f64,
    pub slippage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub trades: Vec<Order>,
    pub performance: PerformanceMetrics,
    pub equity_curve: Vec<EquityPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_return: f64,
    pub annual_return: f64,
    pub sharpe_ratio: f64,
    pub max_drawdown: f64,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    pub timestamp: DateTime<Utc>,
    pub equity: f64,
}

pub struct Backtester {
    config: BacktestConfig,
    strategy: Box<dyn Strategy>,
    candles: Vec<Candle>,
    portfolio: Portfolio,
    equity_curve: Vec<EquityPoint>,
}

impl Backtester {
    pub fn new(config: BacktestConfig, strategy: Box<dyn Strategy>, candles: Vec<Candle>) -> Self {
        Self {
            config,
            strategy,
            candles,
            portfolio: Portfolio::new(config.initial_capital),
            equity_curve: Vec::new(),
        }
    }
    
    pub fn run(&mut self) -> Result<BacktestResult, String> {
        // 初始化策略
        self.strategy.init()?;
        
        // 遍历K线数据
        for candle in &self.candles {
            // 更新策略状态
            self.strategy.update(candle)?;
            
            // 检查交易信号
            if let Some(signal) = self.strategy.check_signal(candle)? {
                // 处理交易信号
                self.process_signal(signal, candle);
            }
            
            // 更新投资组合
            self.portfolio.update(candle);
            
            // 记录权益曲线
            self.equity_curve.push(EquityPoint {
                timestamp: candle.timestamp,
                equity: self.portfolio.total_equity(),
            });
        }
        
        // 计算性能指标
        let performance = self.calculate_performance();
        
        Ok(BacktestResult {
            trades: self.portfolio.trades.clone(),
            performance,
            equity_curve: self.equity_curve.clone(),
        })
    }
    
    fn process_signal(&mut self, signal: OrderSignal, candle: &Candle) {
        // 应用滑点
        let price = match signal.price {
            Some(p) => p,
            None => {
                // 市价单，使用收盘价加上滑点
                if signal.side == crate::trading::OrderSide::Buy {
                    candle.close * (1.0 + self.config.slippage)
                } else {
                    candle.close * (1.0 - self.config.slippage)
                }
            }
        };
        
        // 创建订单
        let mut order = signal.to_order();
        order.price = Some(price);
        
        // 处理订单
        match self.portfolio.process_order(order) {
            Ok(_) => {},
            Err(e) => {
                println!("Failed to process order: {}", e);
            }
        }
    }
    
    fn calculate_performance(&self) -> PerformanceMetrics {
        let total_return = self.portfolio.return_rate();
        
        // 计算年化收益率
        let first_date = self.candles.first().map(|c| c.timestamp).unwrap_or_else(Utc::now);
        let last_date = self.candles.last().map(|c| c.timestamp).unwrap_or_else(Utc::now);
        let days = (last_date - first_date).num_days() as f64;
        let years = days / 365.0;
        let annual_return = if years > 0.0 {
            (1.0 + total_return).powf(1.0 / years) - 1.0
        } else {
            0.0
        };
        
        // 计算最大回撤
        let mut max_drawdown = 0.0;
        let mut peak = 0.0;
        
        for point in &self.equity_curve {
            if point.equity > peak {
                peak = point.equity;
            } else {
                let drawdown = (peak - point.equity) / peak;
                if drawdown > max_drawdown {
                    max_drawdown = drawdown;
                }
            }
        }
        
        // 计算胜率和盈亏比
        let total_trades = self.portfolio.trades.len();
        let mut winning_trades = 0;
        let mut losing_trades = 0;
        let mut total_profit = 0.0;
        let mut total_loss = 0.0;
        
        for trade in &self.portfolio.trades {
            if let (Some(avg_price), Some(price)) = (trade.average_price, trade.price) {
                let pnl = match trade.side {
                    crate::trading::OrderSide::Buy => (price - avg_price) * trade.quantity,
                    crate::trading::OrderSide::Sell => (avg_price - price) * trade.quantity,
                };
                
                if pnl > 0.0 {
                    winning_trades += 1;
                    total_profit += pnl;
                } else {
                    losing_trades += 1;
                    total_loss += -pnl;
                }
            }
        }
        
        let win_rate = if total_trades > 0 {
            winning_trades as f64 / total_trades as f64
        } else {
            0.0
        };
        
        let profit_factor = if total_loss > 0.0 {
            total_profit / total_loss
        } else {
            if total_profit > 0.0 { f64::INFINITY } else { 0.0 }
        };
        
        // 计算夏普比率
        // 简化实现，假设无风险利率为0
        let returns: Vec<f64> = self.equity_curve.windows(2)
            .map(|w| (w[1].equity - w[0].equity) / w[0].equity)
            .collect();
        
        let mean_return = if returns.is_empty() {
            0.0
        } else {
            returns.iter().sum::<f64>() / returns.len() as f64
        };
        
        let std_dev = if returns.len() > 1 {
            let variance = returns.iter()
                .map(|r| (r - mean_return).powi(2))
                .sum::<f64>() / (returns.len() - 1) as f64;
            variance.sqrt()
        } else {
            0.0
        };
        
        let sharpe_ratio = if std_dev > 0.0 {
            mean_return / std_dev * (252.0_f64).sqrt() // 假设一年有252个交易日
        } else {
            0.0
        };
        
        PerformanceMetrics {
            total_return,
            annual_return,
            sharpe_ratio,
            max_drawdown,
            win_rate,
            profit_factor,
            total_trades,
            winning_trades,
            losing_trades,
        }
    }
}