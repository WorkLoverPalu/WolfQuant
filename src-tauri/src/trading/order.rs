use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Created,
    Submitted,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<String>,
    pub symbol: String,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: Option<f64>,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub filled_quantity: f64,
    pub average_price: Option<f64>,
}

impl Order {
    pub fn new(symbol: String, order_type: OrderType, side: OrderSide, quantity: f64, price: Option<f64>) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            symbol,
            order_type,
            side,
            quantity,
            price,
            status: OrderStatus::Created,
            created_at: now,
            updated_at: now,
            filled_quantity: 0.0,
            average_price: None,
        }
    }
    
    pub fn is_filled(&self) -> bool {
        self.status == OrderStatus::Filled
    }
    
    pub fn is_active(&self) -> bool {
        matches!(self.status, OrderStatus::Created | OrderStatus::Submitted | OrderStatus::PartiallyFilled)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSignal {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub price: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

impl OrderSignal {
    pub fn new(symbol: String, side: OrderSide, quantity: f64, price: Option<f64>) -> Self {
        Self {
            symbol,
            side,
            quantity,
            price,
            timestamp: Utc::now(),
        }
    }
    
    pub fn to_order(&self) -> Order {
        let order_type = if self.price.is_some() {
            OrderType::Limit
        } else {
            OrderType::Market
        };
        
        Order::new(
            self.symbol.clone(),
            order_type,
            self.side.clone(),
            self.quantity,
            self.price,
        )
    }
}