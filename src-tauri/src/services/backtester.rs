use crate::database::get_connection_from_pool;
use crate::error::auth::AuthError;
use crate::models::backtester::{BacktestConfig, BacktestResult, EquityPoint, PerformanceMetrics};
use crate::models::candle::Candle;
use crate::models::strategy::Strategy;
use crate::models::trading::{Order, OrderSide, OrderSignal, Portfolio};
use chrono::{DateTime, Utc};
use log::{error, info};

/// 回测服务，负责执行策略回测并计算绩效指标
pub struct BacktesterService;

impl BacktesterService {
    /// 创建新的回测服务实例
    pub fn new() -> Self {
        Self {}
    }

    /// 运行回测流程
    pub fn run_backtest(
        &self,
        config: BacktestConfig,
        strategy: Box<dyn Strategy>,
        candles: Vec<Candle>,
    ) -> Result<BacktestResult, String> {
        info!(
            "Starting backtest with initial capital: {}",
            config.initial_capital
        );

        // 初始化策略
        strategy.init()?;

        // 初始化投资组合
        let mut portfolio = Portfolio::new(config.initial_capital);
        let mut equity_curve = Vec::new();

        // 遍历K线数据
        for candle in &candles {
            // 更新策略状态
            strategy.update(candle)?;

            // 检查交易信号
            if let Some(signal) = strategy.check_signal(candle)? {
                // 处理交易信号
                self.process_signal(&mut portfolio, signal, candle, &config);
            }

            // 更新投资组合
            portfolio.update(candle);

            // 记录权益曲线
            equity_curve.push(EquityPoint {
                timestamp: candle.timestamp,
                equity: portfolio.total_equity(),
            });
        }

        // 计算性能指标
        let performance = self.calculate_performance(&portfolio, &candles, &equity_curve);

        info!(
            "Backtest completed. Total return: {:.2}%",
            performance.total_return * 100.0
        );

        Ok(BacktestResult {
            trades: portfolio.trades.clone(),
            performance,
            equity_curve,
        })
    }

    /// 保存回测结果到数据库
    pub fn save_backtest_result(
        &self,
        user_id: i64,
        strategy_id: i64,
        config: &BacktestConfig,
        result: &BacktestResult,
    ) -> Result<i64, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // 开始事务
        let tx = conn
            .transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;

        // 1. 保存回测记录
        let now = chrono::Utc::now().timestamp();
        let backtest_id = {
            tx.execute(
                "INSERT INTO backtest_results (
                    user_id, strategy_id, initial_capital, fee_rate, slippage,
                    total_return, annual_return, sharpe_ratio, max_drawdown, win_rate,
                    profit_factor, total_trades, winning_trades, losing_trades,
                    created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                rusqlite::params![
                    user_id,
                    strategy_id,
                    config.initial_capital,
                    config.fee_rate,
                    config.slippage,
                    result.performance.total_return,
                    result.performance.annual_return,
                    result.performance.sharpe_ratio,
                    result.performance.max_drawdown,
                    result.performance.win_rate,
                    result.performance.profit_factor,
                    result.performance.total_trades as i64,
                    result.performance.winning_trades as i64,
                    result.performance.losing_trades as i64,
                    now
                ],
            )
            .map_err(|e| format!("Failed to save backtest result: {}", e))?;

            tx.last_insert_rowid()
        };

        // 2. 保存权益曲线
        for point in &result.equity_curve {
            tx.execute(
                "INSERT INTO equity_curves (
                    backtest_id, timestamp, equity
                ) VALUES (?, ?, ?)",
                rusqlite::params![backtest_id, point.timestamp.timestamp(), point.equity],
            )
            .map_err(|e| format!("Failed to save equity point: {}", e))?;
        }

        // 3. 保存交易记录
        for trade in &result.trades {
            tx.execute(
                "INSERT INTO backtest_trades (
                    backtest_id, symbol, side, quantity, price, timestamp
                ) VALUES (?, ?, ?, ?, ?, ?)",
                rusqlite::params![
                    backtest_id,
                    trade.symbol,
                    if trade.side == OrderSide::Buy {
                        "buy"
                    } else {
                        "sell"
                    },
                    trade.quantity,
                    trade.price.unwrap_or(0.0),
                    trade.timestamp.map(|dt| dt.timestamp()).unwrap_or(0)
                ],
            )
            .map_err(|e| format!("Failed to save trade: {}", e))?;
        }

        // 提交事务
        tx.commit()
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        Ok(backtest_id)
    }

    /// 获取回测结果
    pub fn get_backtest_result(&self, backtest_id: i64) -> Result<Option<BacktestResult>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;

        // 1. 获取回测基本信息
        let performance = conn.query_row(
            "SELECT 
                total_return, annual_return, sharpe_ratio, max_drawdown, win_rate,
                profit_factor, total_trades, winning_trades, losing_trades
             FROM backtest_results WHERE id = ?",
            rusqlite::params![backtest_id],
            |row| {
                Ok(PerformanceMetrics {
                    total_return: row.get(0)?,
                    annual_return: row.get(1)?,
                    sharpe_ratio: row.get(2)?,
                    max_drawdown: row.get(3)?,
                    win_rate: row.get(4)?,
                    profit_factor: row.get(5)?,
                    total_trades: row.get::<_, i64>(6)? as usize,
                    winning_trades: row.get::<_, i64>(7)? as usize,
                    losing_trades: row.get::<_, i64>(8)? as usize,
                })
            },
        );

        let performance = match performance {
            Ok(p) => p,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            Err(e) => return Err(format!("Failed to get backtest result: {}", e)),
        };

        // 2. 获取权益曲线
        let mut stmt = conn
            .prepare(
                "SELECT timestamp, equity FROM equity_curves 
             WHERE backtest_id = ? ORDER BY timestamp ASC",
            )
            .map_err(|e| format!("Failed to prepare equity curve query: {}", e))?;

        let equity_iter = stmt
            .query_map(rusqlite::params![backtest_id], |row| {
                Ok(EquityPoint {
                    timestamp: Utc.timestamp(row.get::<_, i64>(0)?, 0),
                    equity: row.get(1)?,
                })
            })
            .map_err(|e| format!("Failed to execute equity curve query: {}", e))?;

        let mut equity_curve = Vec::new();
        for point in equity_iter {
            equity_curve.push(point.map_err(|e| format!("Failed to process equity point: {}", e))?);
        }

        // 3. 获取交易记录
        let mut stmt = conn
            .prepare(
                "SELECT symbol, side, quantity, price, timestamp 
             FROM backtest_trades WHERE backtest_id = ? ORDER BY timestamp ASC",
            )
            .map_err(|e| format!("Failed to prepare trades query: {}", e))?;

        let trades_iter = stmt
            .query_map(rusqlite::params![backtest_id], |row| {
                let side_str: String = row.get(1)?;
                let side = if side_str == "buy" {
                    OrderSide::Buy
                } else {
                    OrderSide::Sell
                };
                let timestamp: i64 = row.get(4)?;

                Ok(Order {
                    symbol: row.get(0)?,
                    side,
                    quantity: row.get(2)?,
                    price: Some(row.get(3)?),
                    timestamp: Some(Utc.timestamp(timestamp, 0)),
                    average_price: None,
                    filled_quantity: 0.0,
                    status: crate::models::trading::OrderStatus::Filled,
                    id: None,
                })
            })
            .map_err(|e| format!("Failed to execute trades query: {}", e))?;

        let mut trades = Vec::new();
        for trade in trades_iter {
            trades.push(trade.map_err(|e| format!("Failed to process trade: {}", e))?);
        }

        Ok(Some(BacktestResult {
            performance,
            equity_curve,
            trades,
        }))
    }

    /// 处理交易信号
    fn process_signal(
        &self,
        portfolio: &mut Portfolio,
        signal: OrderSignal,
        candle: &Candle,
        config: &BacktestConfig,
    ) {
        // 应用滑点
        let price = match signal.price {
            Some(p) => p,
            None => {
                // 市价单，使用收盘价加上滑点
                if signal.side == OrderSide::Buy {
                    candle.close * (1.0 + config.slippage)
                } else {
                    candle.close * (1.0 - config.slippage)
                }
            }
        };

        // 创建订单
        let mut order = signal.to_order();
        order.price = Some(price);

        // 处理订单
        match portfolio.process_order(order) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to process order: {}", e);
            }
        }
    }

    /// 计算绩效指标
    fn calculate_performance(
        &self,
        portfolio: &Portfolio,
        candles: &[Candle],
        equity_curve: &[EquityPoint],
    ) -> PerformanceMetrics {
        let total_return = portfolio.return_rate();

        // 计算年化收益率
        let first_date = candles
            .first()
            .map(|c| c.timestamp)
            .unwrap_or_else(Utc::now);
        let last_date = candles.last().map(|c| c.timestamp).unwrap_or_else(Utc::now);
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

        for point in equity_curve {
            if point.equity > peak {
                peak = point.equity;
            } else if peak > 0.0 {
                let drawdown = (peak - point.equity) / peak;
                if drawdown > max_drawdown {
                    max_drawdown = drawdown;
                }
            }
        }

        // 计算胜率和盈亏比
        let total_trades = portfolio.trades.len();
        let mut winning_trades = 0;
        let mut losing_trades = 0;
        let mut total_profit = 0.0;
        let mut total_loss = 0.0;

        for trade in &portfolio.trades {
            if let (Some(avg_price), Some(price)) = (trade.average_price, trade.price) {
                let pnl = match trade.side {
                    OrderSide::Buy => (price - avg_price) * trade.quantity,
                    OrderSide::Sell => (avg_price - price) * trade.quantity,
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
        } else if total_profit > 0.0 {
            f64::INFINITY
        } else {
            0.0
        };

        // 计算夏普比率
        // 简化实现，假设无风险利率为0
        let returns: Vec<f64> = equity_curve
            .windows(2)
            .map(|w| (w[1].equity - w[0].equity) / w[0].equity)
            .collect();

        let mean_return = if returns.is_empty() {
            0.0
        } else {
            returns.iter().sum::<f64>() / returns.len() as f64
        };

        let std_dev = if returns.len() > 1 {
            let variance = returns
                .iter()
                .map(|r| (r - mean_return).powi(2))
                .sum::<f64>()
                / (returns.len() - 1) as f64;
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
