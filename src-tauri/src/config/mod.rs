use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::RwLock;
use lazy_static::lazy_static;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    // 应用基本配置
    pub app_name: String,
    pub version: String,
    
    // 开发模式配置
    pub dev_mode: bool,
    
    // 认证相关配置
    pub auth: AuthConfig,
    
    // 日志相关配置
    pub logging: LoggingConfig,
    
    // 数据库配置
    pub database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthConfig {
    pub enable_email_verification: bool,
    pub token_expiry_hours: u32,
    pub min_password_length: u8,
    pub password_reset_token_expiry_minutes: u32,
    pub session_timeout_days: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String,
    pub file_output: bool,
    pub console_output: bool,
    pub log_file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub backup_enabled: bool,
    pub backup_interval_days: u32,
    pub backup_path: String,
    pub max_size:u32,
    pub version:u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: "WolfQuant".to_string(),
            version: "1.0.0".to_string(),
            dev_mode: false,
            auth: AuthConfig {
                enable_email_verification: false,
                token_expiry_hours: 24,
                min_password_length: 6,
                password_reset_token_expiry_minutes: 30,
                session_timeout_days: 30,
            },
            logging: LoggingConfig {
                enabled: true,
                level: "info".to_string(),
                file_output: true,
                console_output: true,
                log_file_path: "logs/app.log".to_string(),
            },
            database: DatabaseConfig {
                path: "data/wolfquant.db".to_string(),
                backup_enabled: true,
                backup_interval_days: 7,
                backup_path: "data/backups".to_string(),
                max_size:10,//连接池的最大连接数量
                version:1,//当前数据库版本
            },
        }
    }
}

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::default());
}

impl Config {
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
    
    pub fn get() -> Config {
        CONFIG.read().unwrap().clone()
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(config_path, toml_string)?;
        
        let mut config = CONFIG.write().unwrap();
        *config = self.clone();
        
        Ok(())
    }
}