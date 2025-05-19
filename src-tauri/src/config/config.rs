use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    // 应用基本配置
    pub app_name: String, // 应用名称
    pub version: String,  // 应用版本

    // 开发模式配置
    pub dev_mode: bool, // 是否启用开发模式

    // 认证相关配置
    pub auth: AuthConfig, // 用户认证配置

    // 日志相关配置
    pub logging: LoggingConfig, // 日志系统配置

    // 数据库配置
    pub database: DatabaseConfig, // 数据库连接配置

    // 新增交易相关配置
    pub trading: TradingConfig, // 交易系统配置
}

// ==================== 交易系统配置 ====================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradingConfig {
    pub debug: bool,                                     // 是否启用调试模式
    pub watch: MarketWatchConfig,                        // 市场监控配置
    pub trading_advisor: TradingAdvisorConfig,           // 交易建议配置
    pub paper_trader: PaperTraderConfig,                 // 模拟交易配置
    pub performance_analyzer: PerformanceAnalyzerConfig, // 性能分析配置
    pub trader: TraderConfig,                            // 真实交易配置
    pub backtest: BacktestConfig,                        // 回测配置
}

// 市场监控配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketWatchConfig {
    pub exchange: String, // 交易所名称(如: binance)
    pub currency: String, // 交易货币(如: USDT)
    pub asset: String,    // 交易资产(如: BTC)
}

// 交易建议配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradingAdvisorConfig {
    pub enabled: bool,     // 是否启用交易建议
    pub method: String,    // 使用的交易策略(如: MACD)
    pub candle_size: u32,  // K线周期(分钟)
    pub history_size: u32, // 历史数据数量
}

// 模拟交易配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaperTraderConfig {
    pub enabled: bool,                         // 是否启用模拟交易
    pub report_in_currency: bool,              // 是否以货币报告收益
    pub simulation_balance: SimulationBalance, // 模拟初始余额
    pub fee_maker: f64,                        // Maker手续费率(%)
    pub fee_taker: f64,                        // Taker手续费率(%)
    pub fee_using: String,                     // 使用的手续费类型(maker/taker)
    pub slippage: f64,                         // 交易滑点(%)
}

// 模拟初始余额
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimulationBalance {
    pub asset: f64,    // 初始资产数量
    pub currency: f64, // 初始货币数量
}

// 性能分析配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceAnalyzerConfig {
    pub enabled: bool,         // 是否启用性能分析
    pub risk_free_return: f64, // 无风险回报率(%)
}

// 真实交易配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraderConfig {
    pub enabled: bool,      // 是否启用真实交易
    pub key: String,        // API密钥
    pub secret: String,     // API密钥
    pub username: String,   // 用户名(某些交易所需要)
    pub passphrase: String, // 密码短语(GDAX等交易所需要)
}

// 回测配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BacktestConfig {
    pub daterange: String, // 回测时间范围
    pub batch_size: u32,   // 每批处理数据量
}

// ==================== 策略配置 ====================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MACDConfig {
    pub short: u32,             // 短期EMA周期
    pub long: u32,              // 长期EMA周期
    pub signal: u32,            // 信号线周期
    pub thresholds: Thresholds, // 交易阈值配置
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thresholds {
    pub down: f64,        // 卖出信号阈值
    pub up: f64,          // 买入信号阈值
    pub persistence: u32, // 信号持续周期数(确认趋势)
}

// ==================== 服务配置结构 ====================
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub enable_email_verification: bool,          // 是否启用邮箱验证
    pub token_expiry_hours: u32,                  // Token过期时间(小时)
    pub min_password_length: u8,                  // 密码最小长度
    pub password_reset_token_expiry_minutes: u32, // 密码重置Token过期时间(分钟)
    pub session_timeout_days: u32,                // 会话超时时间(天)
    pub emial_code_valid_duration: u32,           // 邮箱验证码有效时间(分钟)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub enabled: bool,         // 是否启用日志
    pub level: String,         // 日志级别(info/debug/warn/error)
    pub file_output: bool,     // 是否输出到文件
    pub console_output: bool,  // 是否输出到控制台
    pub log_file_path: String, // 日志文件路径
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,              // 数据库文件路径
    pub backup_enabled: bool,      // 是否启用备份
    pub backup_interval_days: u32, // 备份间隔(天)
    pub backup_path: String,       // 备份路径
    pub max_size: u32,             // 连接池最大连接数
    pub version: u32,              // 数据库版本
    pub schema_dir: String,        // 数据库schema目录
}

// ==================== 默认配置实现 ====================
impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: "WolfQuant".to_string(),
            version: "1.0.0".to_string(),
            dev_mode: false,
            //认证配置
            auth: AuthConfig {
                enable_email_verification: false,
                token_expiry_hours: 24,
                min_password_length: 6,
                password_reset_token_expiry_minutes: 30,
                session_timeout_days: 30,
                emial_code_valid_duration: 10,
            },
            //日志配置
            logging: LoggingConfig {
                enabled: true,
                level: "info".to_string(),
                file_output: true,
                console_output: true,
                log_file_path: "logs/app.log".to_string(),
            },
            //数据库配置
            database: DatabaseConfig {
                path: "data/wolfquant.db".to_string(),
                backup_enabled: true,
                backup_interval_days: 7,
                backup_path: "data/backups".to_string(),
                max_size: 10,
                version: 1,
                schema_dir: "data/schemas".to_string(),
            },
            //交易配置
            trading: TradingConfig {
                debug: true,
                //市场监控配置
                watch: MarketWatchConfig {
                    exchange: "binance".to_string(),
                    currency: "USDT".to_string(),
                    asset: "BTC".to_string(),
                },
                //交易建议
                trading_advisor: TradingAdvisorConfig {
                    enabled: true,
                    method: "MACD".to_string(),
                    candle_size: 60,
                    history_size: 10,
                },
                //模拟交易
                paper_trader: PaperTraderConfig {
                    enabled: true,
                    report_in_currency: true,
                    simulation_balance: SimulationBalance {
                        asset: 1.0,
                        currency: 100.0,
                    },
                    fee_maker: 0.15,
                    fee_taker: 0.25,
                    fee_using: "maker".to_string(),
                    slippage: 0.05,
                },
                //性能分析
                performance_analyzer: PerformanceAnalyzerConfig {
                    enabled: true,
                    risk_free_return: 5.0,
                },
                //真实交易配置
                trader: TraderConfig {
                    enabled: false,
                    key: "".to_string(),
                    secret: "".to_string(),
                    username: "".to_string(),
                    passphrase: "".to_string(),
                },
                //回测
                backtest: BacktestConfig {
                    daterange: "scan".to_string(),
                    batch_size: 50,
                },
            },
        }
    }
}

// ==================== 全局配置访问 ====================
lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

impl Config {
    /// 加载配置文件
    pub fn load() -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");

        // 如果配置文件不存在，创建默认配置
        if !config_path.exists() {
            let default_config = Config::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            fs::create_dir_all(config_path.parent().unwrap_or(Path::new("")))?;
            fs::write(config_path, toml_string)?;

            let mut config = CONFIG.write().unwrap();
            *config = default_config;
            return Ok(());
        }

        // 读取配置文件
        let config_str = fs::read_to_string(config_path)?;
        let loaded_config: Config = toml::from_str(&config_str)?;

        let mut config = CONFIG.write().unwrap();
        *config = loaded_config;

        Ok(())
    }

    /// 获取当前配置(克隆)
    pub fn get() -> Config {
        CONFIG.read().unwrap().clone()
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(config_path, toml_string)?;

        let mut config = CONFIG.write().unwrap();
        *config = self.clone();

        Ok(())
    }
}
