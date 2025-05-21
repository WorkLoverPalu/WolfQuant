use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::Candle;
use crate::models::order::{Order, OrderSide, OrderStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub average_price: f64,
    pub current_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
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
            self.average_price = (self.average_price * self.quantity + price * quantity) / (self.quantity + quantity);
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub initial_balance: f64,
    pub balance: f64,
    pub positions: HashMap<String, Position>,
    pub trades: Vec<Order>,
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
        order.updated_at = now;
        
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
            },
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
            },
        }
        
        self.trades.push(order.clone());
        self.update_equity_history();
        
        Ok(order)
    }
    
    pub fn update(&mut self, candle: &Candle) {
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