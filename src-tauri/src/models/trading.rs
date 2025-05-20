use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 订单方向（买入/卖出）
///
/// 表示订单的方向，包括买入（Buy）和卖出（Sell）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    /// 从字符串创建订单方向，支持 "buy" 和 "sell"，默认为买入。
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => OrderSide::Buy, // 默认为买入
        }
    }
    /// 将订单方向转换为字符串（"buy" 或 "sell"）。
    pub fn to_str(&self) -> &'static str {
        match self {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        }
    }
}

/// 订单类型
///
/// 表示订单的类型，包括市价单、限价单、止损单、止损限价单和跟踪止损单。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
    TrailingStop,
}

impl OrderType {
    /// 从字符串创建订单类型，默认为市价单。
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "market" => OrderType::Market,
            "limit" => OrderType::Limit,
            "stop_loss" => OrderType::StopLoss,
            "stop_limit" => OrderType::StopLimit,
            "trailing_stop" => OrderType::TrailingStop,
            _ => OrderType::Market, // 默认为市价单
        }
    }
    /// 将订单类型转换为字符串。
    pub fn to_str(&self) -> &'static str {
        match self {
            OrderType::Market => "market",
            OrderType::Limit => "limit",
            OrderType::StopLoss => "stop_loss",
            OrderType::StopLimit => "stop_limit",
            OrderType::TrailingStop => "trailing_stop",
        }
    }
}
/// 订单状态
///
/// 表示订单的当前状态，如已创建、已提交、部分成交、全部成交、已取消、已拒绝、已过期等。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Created,
    Submitted,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}

impl OrderStatus {
    /// 从字符串创建订单状态，默认为已创建。
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "created" => OrderStatus::Created,
            "submitted" => OrderStatus::Submitted,
            "partially_filled" => OrderStatus::PartiallyFilled,
            "filled" => OrderStatus::Filled,
            "canceled" => OrderStatus::Canceled,
            "rejected" => OrderStatus::Rejected,
            "expired" => OrderStatus::Expired,
            _ => OrderStatus::Created,
        }
    }
    /// 将订单状态转换为字符串。
    pub fn to_str(&self) -> &'static str {
        match self {
            OrderStatus::Created => "created",
            OrderStatus::Submitted => "submitted",
            OrderStatus::PartiallyFilled => "partially_filled",
            OrderStatus::Filled => "filled",
            OrderStatus::Canceled => "canceled",
            OrderStatus::Rejected => "rejected",
            OrderStatus::Expired => "expired",
        }
    }
}

/// 交易信号 - 由策略生成的交易信号
///
/// 用于描述策略生成的交易信号，包括交易品种、方向、类型、数量、价格、止损价格、有效期、原因和时间戳等信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSignal {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: Option<String>,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl OrderSignal {
    /// 创建新的买入信号
    ///
    /// # 参数
    /// - `symbol`: 交易品种
    /// - `quantity`: 买入数量
    /// - `price`: 买入价格（可选，若为 None 则为市价单）
    pub fn buy(symbol: &str, quantity: f64, price: Option<f64>) -> Self {
        Self {
            symbol: symbol.to_string(),
            side: OrderSide::Buy,
            order_type: if price.is_some() {
                OrderType::Limit
            } else {
                OrderType::Market
            },
            quantity,
            price,
            stop_price: None,
            time_in_force: Some("GTC".to_string()), // Good Till Canceled
            reason: None,
            timestamp: Utc::now(),
        }
    }

    /// 创建新的卖出信号
    ///
    /// # 参数
    /// - `symbol`: 交易品种
    /// - `quantity`: 卖出数量
    /// - `price`: 卖出价格（可选，若为 None 则为市价单）
    pub fn sell(symbol: &str, quantity: f64, price: Option<f64>) -> Self {
        Self {
            symbol: symbol.to_string(),
            side: OrderSide::Sell,
            order_type: if price.is_some() {
                OrderType::Limit
            } else {
                OrderType::Market
            },
            quantity,
            price,
            stop_price: None,
            time_in_force: Some("GTC".to_string()),
            reason: None,
            timestamp: Utc::now(),
        }
    }

    /// 设置信号原因
    ///
    /// # 参数
    /// - `reason`: 信号产生的原因
    pub fn with_reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_string());
        self
    }

    /// 设置止损价格
    ///
    /// # 参数
    /// - `stop_price`: 止损价格
    pub fn with_stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self.order_type = OrderType::StopLoss;
        self
    }

    /// 将信号转换为订单
    pub fn to_order(&self) -> Order {
        Order {
            id: None,
            symbol: self.symbol.clone(),
            side: self.side,
            quantity: self.quantity,
            price: self.price,
            average_price: None,
            filled_quantity: 0.0,
            status: OrderStatus::Created,
            timestamp: Some(self.timestamp),
        }
    }
}

/// 订单 - 表示一个交易订单
///
/// 包含订单的唯一标识、品种、方向、数量、价格、成交均价、已成交数量、状态和时间戳等信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<String>,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: Option<f64>,
    pub average_price: Option<f64>,
    pub filled_quantity: f64,
    pub status: OrderStatus,
    pub timestamp: Option<DateTime<Utc>>,
}

/// 持仓 - 表示一个资产的持仓
///
/// 包含品种、持仓数量、平均成本、当前价格、未实现盈亏、已实现盈亏和最后更新时间等信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub average_cost: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub last_updated: DateTime<Utc>,
}

impl Position {
    /// 创建新的持仓
    ///
    /// # 参数
    /// - `symbol`: 资产品种
    /// - `quantity`: 持仓数量
    /// - `average_cost`: 平均成本
    /// - `current_price`: 当前价格
    pub fn new(symbol: &str, quantity: f64, average_cost: f64, current_price: f64) -> Self {
        let unrealized_pnl = (current_price - average_cost) * quantity;

        Self {
            symbol: symbol.to_string(),
            quantity,
            average_cost,
            current_price,
            unrealized_pnl,
            realized_pnl: 0.0,
            last_updated: Utc::now(),
        }
    }

    /// 更新持仓价格
    ///
    /// # 参数
    /// - `price`: 新的当前价格
    pub fn update_price(&mut self, price: f64) {
        self.current_price = price;
        self.unrealized_pnl = (price - self.average_cost) * self.quantity;
        self.last_updated = Utc::now();
    }

    /// 增加持仓
    ///
    /// # 参数
    /// - `quantity`: 增加的数量
    /// - `price`: 增加部分的价格
    pub fn add(&mut self, quantity: f64, price: f64) {
        if self.quantity + quantity <= 0.0 {
            // 全部平仓
            self.realized_pnl += (price - self.average_cost) * self.quantity;
            self.quantity = 0.0;
            self.unrealized_pnl = 0.0;
        } else {
            // 计算新的平均成本
            let total_cost = self.average_cost * self.quantity + price * quantity;
            self.quantity += quantity;
            self.average_cost = total_cost / self.quantity;
            self.unrealized_pnl = (self.current_price - self.average_cost) * self.quantity;
        }

        self.last_updated = Utc::now();
    }

    /// 减少持仓
    ///
    /// # 参数
    /// - `quantity`: 减少的数量
    /// - `price`: 卖出价格
    pub fn remove(&mut self, quantity: f64, price: f64) {
        if quantity >= self.quantity {
            // 全部平仓
            self.realized_pnl += (price - self.average_cost) * self.quantity;
            self.quantity = 0.0;
            self.unrealized_pnl = 0.0;
        } else {
            // 部分平仓
            self.realized_pnl += (price - self.average_cost) * quantity;
            self.quantity -= quantity;
            self.unrealized_pnl = (self.current_price - self.average_cost) * self.quantity;
        }

        self.last_updated = Utc::now();
    }

    /// 获取总盈亏（未实现盈亏 + 已实现盈亏）
    pub fn total_pnl(&self) -> f64 {
        self.unrealized_pnl + self.realized_pnl
    }
}

/// 投资组合 - 管理资金和持仓
///
/// 用于管理账户资金、持仓、交易记录和初始资金等信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: HashMap<String, Position>,
    pub trades: Vec<Order>,
    pub initial_capital: f64,
}

impl Portfolio {
    /// 创建新的投资组合
    ///
    /// # 参数
    /// - `initial_capital`: 初始资金
    pub fn new(initial_capital: f64) -> Self {
        Self {
            cash: initial_capital,
            positions: HashMap::new(),
            trades: Vec::new(),
            initial_capital,
        }
    }

    /// 处理订单
    ///
    /// 根据订单类型和方向，更新资金、持仓和订单状态。
    ///
    /// # 参数
    /// - `order`: 需要处理的订单
    /// # 返回
    /// - `Result<(), String>`: 成功返回 Ok，失败返回错误信息
    pub fn process_order(&mut self, mut order: Order) -> Result<(), String> {
        // 设置订单时间戳
        if order.timestamp.is_none() {
            order.timestamp = Some(Utc::now());
        }

        // 获取订单价格
        let price = match order.price {
            Some(p) => p,
            None => return Err("Order price is required".to_string()),
        };

        match order.side {
            OrderSide::Buy => {
                // 检查资金是否足够
                let cost = price * order.quantity;
                if cost > self.cash {
                    return Err(format!(
                        "Insufficient funds: required {}, available {}",
                        cost, self.cash
                    ));
                }

                // 更新资金
                self.cash -= cost;

                // 更新持仓
                let symbol = order.symbol.clone();
                if let Some(position) = self.positions.get_mut(&symbol) {
                    position.add(order.quantity, price);
                } else {
                    let position = Position::new(&symbol, order.quantity, price, price);
                    self.positions.insert(symbol, position);
                }

                // 更新订单状态
                order.status = OrderStatus::Filled;
                order.filled_quantity = order.quantity;
                order.average_price = Some(price);
            }
            OrderSide::Sell => {
                // 检查持仓是否足够
                let symbol = order.symbol.clone();
                let position = match self.positions.get_mut(&symbol) {
                    Some(p) => p,
                    None => return Err(format!("No position for symbol {}", symbol)),
                };

                if position.quantity < order.quantity {
                    return Err(format!(
                        "Insufficient position: required {}, available {}",
                        order.quantity, position.quantity
                    ));
                }

                // 更新资金
                let proceeds = price * order.quantity;
                self.cash += proceeds;

                // 更新持仓
                position.remove(order.quantity, price);

                // 如果持仓为零，移除该持仓
                if position.quantity <= 0.0 {
                    self.positions.remove(&symbol);
                }

                // 更新订单状态
                order.status = OrderStatus::Filled;
                order.filled_quantity = order.quantity;
                order.average_price = Some(price);
            }
        }

        // 记录交易
        self.trades.push(order);

        Ok(())
    }

    /// 更新投资组合
    ///
    /// 根据最新的K线数据更新持仓价格。
    ///
    /// # 参数
    /// - `candle`: 最新的K线数据
    pub fn update(&mut self, candle: &crate::models::candle::Candle) {
        // 更新持仓价格
        if let Some(position) = self.positions.get_mut(&candle.symbol) {
            position.update_price(candle.close);
        }
    }

    /// 获取总权益（现金 + 持仓市值）
    pub fn total_equity(&self) -> f64 {
        let positions_value: f64 = self
            .positions
            .values()
            .map(|p| p.quantity * p.current_price)
            .sum();

        self.cash + positions_value
    }

    /// 获取总盈亏（所有持仓的未实现盈亏与已实现盈亏之和）
    pub fn total_pnl(&self) -> f64 {
        let unrealized_pnl: f64 = self.positions.values().map(|p| p.unrealized_pnl).sum();

        let realized_pnl: f64 = self.positions.values().map(|p| p.realized_pnl).sum();

        unrealized_pnl + realized_pnl
    }

    /// 获取收益率（总权益/初始资金 - 1）
    pub fn return_rate(&self) -> f64 {
        let equity = self.total_equity();
        (equity - self.initial_capital) / self.initial_capital
    }
}
