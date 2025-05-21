use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::order::{Order, OrderSide, OrderStatus};
use crate::models::CandleModel;

/// 资产汇总信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetSummary {
    /// 资产类型（如股票、加密货币等）
    pub asset_type: String,
    /// 当前总市值
    pub total_value: f64,
    /// 总成本
    pub total_cost: f64,
    /// 总收益
    pub total_profit: f64,
    /// 总收益率（百分比）
    pub total_profit_percent: f64,
    /// 当日收益
    pub daily_profit: f64,
    /// 当日收益率（百分比）
    pub daily_profit_percent: f64,
}

/// 投资组合汇总信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioSummary {
    /// 当前总市值
    pub total_value: f64,
    /// 总成本
    pub total_cost: f64,
    /// 总收益
    pub total_profit: f64,
    /// 总收益率（百分比）
    pub total_profit_percent: f64,
    /// 当日收益
    pub daily_profit: f64,
    /// 当日收益率（百分比）
    pub daily_profit_percent: f64,
    /// 各类资产汇总信息
    pub asset_summaries: Vec<AssetSummary>,
}

/// 持仓信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// 交易品种代码
    pub symbol: String,
    /// 持仓数量
    pub quantity: f64,
    /// 持仓均价
    pub average_price: f64,
    /// 当前价格
    pub current_price: f64,
    /// 未实现盈亏
    pub unrealized_pnl: f64,
    /// 已实现盈亏
    pub realized_pnl: f64,
    /// 最后更新时间
    pub last_update: DateTime<Utc>,
}

impl Position {
    pub fn new(symbol: String, quantity: f64, price: f64) -> Self {
        Self {
            symbol,
            quantity,
            average_price: price,
            current_price: price,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            last_update: Utc::now(),
        }
    }

    pub fn update_price(&mut self, price: f64) {
        self.unrealized_pnl = (price - self.average_price) * self.quantity;
        self.current_price = price;
        self.last_update = Utc::now();
    }

    pub fn add(&mut self, quantity: f64, price: f64) {
        if self.quantity + quantity == 0.0 {
            self.average_price = 0.0;
        } else {
            self.average_price = (self.average_price * self.quantity + price * quantity)
                / (self.quantity + quantity);
        }
        self.quantity += quantity;
        self.current_price = price;
        self.unrealized_pnl = (self.current_price - self.average_price) * self.quantity;
        self.last_update = Utc::now();
    }

    pub fn close(&mut self, quantity: f64, price: f64) -> f64 {
        let close_quantity = quantity.min(self.quantity);
        let pnl = (price - self.average_price) * close_quantity;

        self.realized_pnl += pnl;
        self.quantity -= close_quantity;

        if self.quantity <= 0.0 {
            self.quantity = 0.0;
            self.average_price = 0.0;
            self.unrealized_pnl = 0.0;
        } else {
            self.unrealized_pnl = (self.current_price - self.average_price) * self.quantity;
        }

        self.current_price = price;
        self.last_update = Utc::now();

        pnl
    }

    pub fn value(&self) -> f64 {
        self.quantity * self.current_price
    }
}

/// 投资组合结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// 初始资金
    pub initial_balance: f64,
    /// 当前可用资金
    pub balance: f64,
    /// 当前持仓，键为交易品种代码
    pub positions: HashMap<String, Position>,
    /// 历史成交订单
    pub trades: Vec<Order>,
    /// 账户权益历史 (时间, 权益)
    pub equity_history: Vec<(DateTime<Utc>, f64)>,
}

impl Portfolio {
    pub fn new(initial_balance: f64) -> Self {
        let now = Utc::now();
        Self {
            initial_balance,
            balance: initial_balance,
            positions: HashMap::new(),
            trades: Vec::new(),
            equity_history: vec![(now, initial_balance)],
        }
    }

    pub fn process_order(&mut self, mut order: Order) -> Result<Order, String> {
        let now = Utc::now();

        // 简化实现，假设所有订单都是市价单并立即成交
        order.status = OrderStatus::Filled;
        order.filled_quantity = order.quantity;
        order.average_price = order.price;

        let symbol = order.symbol.clone();
        let price = order.price.unwrap_or(0.0);

        match order.side {
            OrderSide::Buy => {
                let cost = price * order.quantity;
                if cost > self.balance {
                    return Err("Insufficient balance".to_string());
                }

                self.balance -= cost;

                if let Some(position) = self.positions.get_mut(&symbol) {
                    position.add(order.quantity, price);
                } else {
                    let position = Position::new(symbol.clone(), order.quantity, price);
                    self.positions.insert(symbol, position);
                }
            }
            OrderSide::Sell => {
                if let Some(position) = self.positions.get_mut(&symbol) {
                    if position.quantity < order.quantity {
                        return Err("Insufficient position".to_string());
                    }

                    let pnl = position.close(order.quantity, price);
                    self.balance += price * order.quantity;

                    if position.quantity <= 0.0 {
                        self.positions.remove(&symbol);
                    }
                } else {
                    return Err("Position not found".to_string());
                }
            }
        }

        self.trades.push(order.clone());
        self.update_equity_history();

        Ok(order)
    }

    pub fn update(&mut self, candle: &CandleModel) {
        if let Some(position) = self.positions.get_mut(&candle.symbol) {
            position.update_price(candle.close);
        }

        self.update_equity_history();
    }

    pub fn total_equity(&self) -> f64 {
        let positions_value: f64 = self.positions.values().map(|p| p.value()).sum();
        self.balance + positions_value
    }

    pub fn update_equity_history(&mut self) {
        let now = Utc::now();
        let equity = self.total_equity();
        self.equity_history.push((now, equity));
    }

    pub fn total_pnl(&self) -> f64 {
        let unrealized_pnl: f64 = self.positions.values().map(|p| p.unrealized_pnl).sum();
        let realized_pnl: f64 = self.positions.values().map(|p| p.realized_pnl).sum();
        unrealized_pnl + realized_pnl
    }

    pub fn return_rate(&self) -> f64 {
        let equity = self.total_equity();
        (equity - self.initial_balance) / self.initial_balance
    }
}
