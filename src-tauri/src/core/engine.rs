use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

use crate::adapters;
use crate::core::config::Config;
use crate::core::event::{Event, EventBus, EventData, EventType};
use crate::db::Repository;
use crate::market::{Candle, MarketAdapter, Ticker};
use crate::strategy::Strategy;
use crate::trading::{Order, OrderSignal, Portfolio};

pub struct Engine {
    config: Config,
    event_bus: Arc<EventBus>,
    repository: Arc<Repository>,
    adapters: HashMap<String, Box<dyn MarketAdapter>>,
    strategies: HashMap<String, Box<dyn Strategy>>,
    portfolio: Arc<Mutex<Portfolio>>,
    running: Arc<Mutex<bool>>,
}

impl Engine {
    pub async fn new(config: Config) -> Result<Self, String> {
        let repository = Arc::new(Repository::new(&config.db_path).await?);
        let event_bus = Arc::new(EventBus::new());
        
        Ok(Self {
            config,
            event_bus,
            repository,
            adapters: HashMap::new(),
            strategies: HashMap::new(),
            portfolio: Arc::new(Mutex::new(Portfolio::new(10000.0))), // 默认初始资金
            running: Arc::new(Mutex::new(false)),
        })
    }
    
    pub fn register_adapter(&mut self, asset_type: &str, source: &str) -> Result<(), String> {
        let key = format!("{}:{}", asset_type, source);
        
        if self.adapters.contains_key(&key) {
            return Ok(());
        }
        
        let adapter = adapters::get_adapter(asset_type, source)?;
        self.adapters.insert(key, adapter);
        
        Ok(())
    }
    
    pub fn register_strategy(&mut self, name: &str, strategy: Box<dyn Strategy>) {
        self.strategies.insert(name.to_string(), strategy);
    }
    
    pub fn get_event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }
    
    pub fn get_repository(&self) -> Arc<Repository> {
        self.repository.clone()
    }
    
    pub async fn start_market_data(&self, symbol: &str, asset_type: &str, source: &str) -> Result<(), String> {
        let key = format!("{}:{}", asset_type, source);
        let adapter = self.adapters.get(&key)
            .ok_or_else(|| format!("Adapter not found: {}", key))?;
        
        let event_bus = self.event_bus.clone();
        let adapter_clone = adapter.clone();
        let symbol = symbol.to_string();
        let running = self.running.clone();
        
        *running.lock().unwrap() = true;
        
        tokio::spawn(async move {
            while *running.lock().unwrap() {
                match adapter_clone.get_ticker(&symbol).await {
                    Ok(ticker) => {
                        let event = Event {
                            event_type: EventType::Tick,
                            timestamp: ticker.timestamp,
                            data: EventData::Tick(ticker),
                        };
                        
                        event_bus.publish(event);
                    },
                    Err(e) => {
                        let event = Event {
                            event_type: EventType::Error,
                            timestamp: chrono::Utc::now(),
                            data: EventData::Error(format!("Failed to get ticker: {}", e)),
                        };
                        
                        event_bus.publish(event);
                    }
                }
                
                time::sleep(Duration::from_secs(1)).await;
            }
        });
        
        Ok(())
    }
    
    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }
    
    pub async fn import_historical_data(
        &self,
        symbol: &str,
        asset_type: &str,
        source: &str,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
        interval: &str,
    ) -> Result<Vec<Candle>, String> {
        let key = format!("{}:{}", asset_type, source);
        let adapter = self.adapters.get(&key)
            .ok_or_else(|| format!("Adapter not found: {}", key))?;
        
        let candles = adapter.get_candles(symbol, start_time, end_time, interval).await?;
        
        // 保存到数据库
        self.repository.save_candles(&candles).await?;
        
        Ok(candles)
    }
    
    pub async fn run_strategy(
        &self,
        strategy_name: &str,
        symbol: &str,
        asset_type: &str,
        source: &str,
        start_time: chrono::DateTime<chrono::Utc>,
        end_time: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), String> {
        let strategy = self.strategies.get(strategy_name)
            .ok_or_else(|| format!("Strategy not found: {}", strategy_name))?;
        
        let candles = self.repository.get_candles(symbol, source, start_time, end_time).await?;
        
        if candles.is_empty() {
            return Err("No candles found for the specified period".to_string());
        }
        
        let event_bus = self.event_bus.clone();
        let portfolio = self.portfolio.clone();
        let mut strategy = strategy.clone();
        
        strategy.init()?;
        
        for candle in candles {
            strategy.update(&candle)?;
            
            if let Some(signal) = strategy.check_signal(&candle)? {
                let event = Event {
                    event_type: EventType::Signal,
                    timestamp: candle.timestamp,
                    data: EventData::Signal(signal.clone()),
                };
                
                event_bus.publish(event);
                
                // 处理信号
                let order = signal.to_order();
                let mut portfolio = portfolio.lock().unwrap();
                
                match portfolio.process_order(order.clone()) {
                    Ok(processed_order) => {
                        let event = Event {
                            event_type: EventType::Trade,
                            timestamp: chrono::Utc::now(),
                            data: EventData::Trade(processed_order),
                        };
                        
                        event_bus.publish(event);
                    },
                    Err(e) => {
                        let event = Event {
                            event_type: EventType::Error,
                            timestamp: chrono::Utc::now(),
                            data: EventData::Error(format!("Failed to process order: {}", e)),
                        };
                        
                        event_bus.publish(event);
                    }
                }
            }
        }
        
        Ok(())
    }
}