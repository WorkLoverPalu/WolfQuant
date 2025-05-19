use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::market::{Candle, Ticker};
use crate::trading::{Order, OrderSignal};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Tick,
    Candle,
    Signal,
    Order,
    Trade,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub timestamp: DateTime<Utc>,
    pub data: EventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventData {
    Tick(Ticker),
    Candle(Candle),
    Signal(OrderSignal),
    Order(Order),
    Trade(Order),
    Error(String),
}

pub type EventCallback = Box<dyn Fn(&Event) + Send + Sync>;

pub struct EventBus {
    subscribers: Arc<Mutex<Vec<(EventType, EventCallback)>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn subscribe(&self, event_type: EventType, callback: EventCallback) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push((event_type, callback));
    }
    
    pub fn publish(&self, event: Event) {
        let subscribers = self.subscribers.lock().unwrap();
        
        for (subscribed_type, callback) in subscribers.iter() {
            if std::mem::discriminant(subscribed_type) == std::mem::discriminant(&event.event_type) {
                callback(&event);
            }
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}