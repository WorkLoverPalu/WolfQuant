use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 订单方向（买入/卖出）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => OrderSide::Buy, // 默认为买入
        }
    }
    
    pub fn to_str(&self) -> &'static str {
        match self {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        }
    }
}

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
    TrailingStop,
}

impl OrderType {
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
    pub fn buy(symbol: &str, quantity: f64, price: Option<f64>) -> Self {
        Self {
            symbol: symbol.to_string(),
            side: OrderSide::Buy,
            order_type: if price.is_some() { OrderType::Limit } else { OrderType::Market },
            quantity,
            price,
            stop_price: None,
            time_in_force: Some("GTC".to_string()), // Good Till Canceled
            reason: None,
            timestamp: Utc::now(),
        }
    }
    
    /// 创建新的卖出信号
    pub fn sell(symbol: &str, quantity: f64, price: Option<f64>) -> Self {
        Self {
            symbol: symbol.to_string(),
            side: OrderSide::Sell,
            order_type: if price.is_some() { OrderType::Limit } else { OrderType::Market },
            quantity,
            price,
            stop_price: None,
            time_in_force: Some("GTC".to_string()),
            reason: None,
            timestamp: Utc::now(),
        }
    }
    
    /// 设置信号原因
    pub fn with_reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_string());
        self
    }
    
    /// 设置止损价格
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
    pub fn update_price(&mut self, price: f64) {
        self.current_price = price;
        self.unrealized_pnl = (price - self.average_cost) * self.quantity;
        self.last_updated = Utc::now();
    }
    
    /// 增加持仓
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
    
    /// 获取总盈亏
    pub fn total_pnl(&self) -> f64 {
        self.unrealized_pnl + self.realized_pnl
    }
}

/// 投资组合 - 管理资金和持仓
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub cash: f64,
    pub positions: HashMap<String, Position>,
    pub trades: Vec<Order>,
    pub initial_capital: f64,
}

impl Portfolio {
    /// 创建新的投资组合
    pub fn new(initial_capital: f64) -> Self {
        Self {
            cash: initial_capital,
            positions: HashMap::new(),
            trades: Vec::new(),
            initial_capital,
        }
    }
    
    /// 处理订单
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
            },
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
            },
        }
        
        // 记录交易
        self.trades.push(order);
        
        Ok(())
    }
    
    /// 更新投资组合
    pub fn update(&mut self, candle: &crate::models::candle::Candle) {
        // 更新持仓价格
        if let Some(position) = self.positions.get_mut(&candle.symbol) {
            position.update_price(candle.close);
        }
    }
    
    /// 获取总权益
    pub fn total_equity(&self) -> f64 {
        let positions_value: f64 = self.positions.values()
            .map(|p| p.quantity * p.current_price)
            .sum();
        
        self.cash + positions_value
    }
    
    /// 获取总盈亏
    pub fn total_pnl(&self) -> f64 {
        let unrealized_pnl: f64 = self.positions.values()
            .map(|p| p.unrealized_pnl)
            .sum();
        
        let realized_pnl: f64 = self.positions.values()
            .map(|p| p.realized_pnl)
            .sum();
        
        unrealized_pnl + realized_pnl
    }
    
    /// 获取收益率
    pub fn return_rate(&self) -> f64 {
        let equity = self.total_equity();
        (equity - self.initial_capital) / self.initial_capital
    }
}
