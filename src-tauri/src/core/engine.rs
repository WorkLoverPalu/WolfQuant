/// `Engine` 结构体是交易系统的核心组件，负责协调主流程。
/// 它管理适配器、策略、事件总线、数据仓库、投资组合和数据导入任务。
///
/// # 字段说明
/// - `config`：引擎配置。
/// - `event_bus`：事件总线，用于事件的发布与订阅。
/// - `repository`：数据库仓库，负责数据持久化。
/// - `adapters`：已注册的市场适配器，按资产类型和数据源区分。
/// - `strategies`：已注册的交易策略。
/// - `portfolio`：投资组合，管理订单和持仓。
/// - `running`：引擎运行状态标志。
/// - `importer`：数据导入器，处理历史数据导入任务。
///
/// # 方法说明
/// - `new`：使用指定配置初始化引擎实例。
/// - `register_adapter`：注册指定资产类型和数据源的市场适配器。
/// - `register_strategy`：按名称注册交易策略。
/// - `get_event_bus`：获取事件总线的克隆。
/// - `get_repository`：获取数据仓库的克隆。
/// - `start_market_data`：启动指定适配器的市场数据流。
/// - `stop`：停止引擎运行。
/// - `import_historical_data`：导入指定标的的历史K线数据并保存到数据库。
/// - `run_strategy`：在历史K线数据上运行指定策略。
/// - `start_import`：启动新的历史数据导入任务。
/// - `get_import_task`：根据ID获取导入任务。
/// - `get_import_tasks`：获取所有导入任务。
/// - `get_available_data`：获取已导入数据的列表。
///
/// # 示例
/// ```rust
/// let config = Config::default();
/// let mut engine = Engine::new(config).await.unwrap();
/// engine.register_adapter("crypto", "binance").unwrap();
/// engine.register_strategy("my_strategy", Box::new(MyStrategy::new()));
/// engine.start_market_data("BTCUSDT", "crypto", "binance").await.unwrap();
/// ```
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
use serde::{Deserialize, Serialize};

use crate::import::{AvailableData, DataImporter, ImportTask};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Engine {
    config: Config,
    event_bus: Arc<EventBus>,
    repository: Arc<Repository>,
    adapters: HashMap<String, Box<dyn MarketAdapter>>,
    strategies: HashMap<String, Box<dyn Strategy>>,
    portfolio: Arc<Mutex<Portfolio>>,
    running: Arc<Mutex<bool>>,
    importer: Arc<DataImporter>,
}

impl Engine {
    pub async fn new(config: Config) -> Result<Self, String> {
        let repository = Arc::new(Repository::new(&config.db_path).await?);
        let event_bus = Arc::new(EventBus::new());
        let importer = Arc::new(DataImporter::new(repository.clone(), event_bus.clone()));

        // 初始化导入任务表
        repository.init_import_tasks_table().await?;

        Ok(Self {
            config,
            event_bus,
            repository,
            adapters: HashMap::new(),
            strategies: HashMap::new(),
            portfolio: Arc::new(Mutex::new(Portfolio::new(10000.0))), // 默认初始资金
            running: Arc::new(Mutex::new(false)),
            importer,
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

    pub async fn start_market_data(
        &self,
        symbol: &str,
        asset_type: &str,
        source: &str,
    ) -> Result<(), String> {
        let key = format!("{}:{}", asset_type, source);
        let adapter = self
            .adapters
            .get(&key)
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
                    }
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
        let adapter = self
            .adapters
            .get(&key)
            .ok_or_else(|| format!("Adapter not found: {}", key))?;

        let candles = adapter
            .get_candles(symbol, start_time, end_time, interval)
            .await?;

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
        let strategy = self
            .strategies
            .get(strategy_name)
            .ok_or_else(|| format!("Strategy not found: {}", strategy_name))?;

        let candles = self
            .repository
            .get_candles(symbol, source, start_time, end_time)
            .await?;

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
                    }
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

    // 添加导入相关方法
    pub async fn start_import(
        &self,
        asset_type: &str,
        symbol: &str,
        source: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: &str,
    ) -> Result<ImportTask, String> {
        self.importer
            .start_import(
                asset_type.to_string(),
                symbol.to_string(),
                source.to_string(),
                start_time,
                end_time,
                interval.to_string(),
            )
            .await
    }

    pub async fn get_import_task(&self, id: &str) -> Result<Option<ImportTask>, String> {
        self.importer.get_task(id).await
    }

    pub async fn get_import_tasks(&self) -> Result<Vec<ImportTask>, String> {
        self.importer.get_tasks().await
    }

    pub async fn get_available_data(&self) -> Result<Vec<AvailableData>, String> {
        self.importer.get_available_data().await
    }
}
