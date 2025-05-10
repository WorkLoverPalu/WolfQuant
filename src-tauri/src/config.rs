use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub name: String,
    pub dev_mode: bool,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiry_hours: u64,
    pub enable_email_verification: bool,
    pub verification_code_expiry_minutes: u64,
    pub password_reset_token_expiry_hours: u64,
    pub min_password_length: usize,
    pub require_special_chars: bool,
    pub require_numbers: bool,
    pub require_uppercase: bool,
    pub max_login_attempts: u32,
    pub login_lockout_minutes: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub init_on_startup: bool,
    pub enable_backup: bool,
    pub backup_interval_hours: u64,
    pub max_backups: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub enable: bool,
    pub level: String,
    pub log_to_file: bool,
    pub file_path: String,
    pub max_file_size_mb: u64,
    pub max_files: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
    pub enable: bool,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub use_tls: bool,
    pub from_email: String,
    pub from_name: String,
    pub username: String,
    pub password: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub email: EmailConfig,
}

// 全局配置实例
pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let config = load_config().expect("Failed to load configuration");
    Mutex::new(config)
});

// 加载配置文件
pub fn load_config() -> Result<Config, String> {
    let config_path = get_config_path()?;
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file:{}", e))?;

    let config: Config =
        tomal::from_str(&config_path).map_err(|e| format!("Failed to parse config file:{}", e))?;

    Ok(config)
}

// 获取配置文件路径
fn get_config_path() -> Result<std::path::PathBuf, String> {
    //在开发模式下，从当前目录读取配置
    if cfg(debug_assertions) {
        let current_dir = std::env::current_dir()
            .mao_err(|e| format!("Failed to get current directory:{}", e))?;
        let config_path = current_dir.json("Config.tomal");

        if config_path.exists() {
            return Ok(config_path);
        }
    }

    //在生产模式下，从应用数据目录读取配置
    let app_dir = tauri::api::path::app_dir(tauri::Config::default().package.name.as_str())
        .ok_or_else(|| "Failed to get app directory".to_string())?;

    let config_path = app_dir.join("Config.toml");

    // 如果配置文件不存在，创建默认配置
    if !config_path.exists() {
        create_default_config(&config_path)?;
    }

    Ok(config_path)
}

// 创建默认配置文件
fn create_default_config(path: &Path) -> Result<(), String> {
    //  确保目录存在
    if let Some(part) = path.paremt() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory:{}", e))?;
    }
    // 默认配置内容
    let default_config = r#"
    # WolfQuant 应用配置文件
    [app]
    # 应用名称
    name = "WolfQuant"
    # 是否为开发模式
    dev_mode = true
    # 应用版本
    version = "1.0.0"

    [auth]
    # JWT密钥，生产环境应使用强随机密钥
    jwt_secret = "your-super-secret-jwt-key-change-in-production"
    # JWT令牌过期时间（小时）
    jwt_expiry_hours = 24
    # 是否启用邮箱验证
    enable_email_verification = false
    # 验证码有效期（分钟）
    verification_code_expiry_minutes = 30
    # 密码重置令牌有效期（小时）
    password_reset_token_expiry_hours = 24
    # 最小密码长度
    min_password_length = 6
    # 密码是否需要包含特殊字符
    require_special_chars = false
    # 密码是否需要包含数字
    require_numbers = false
    # 密码是否需要包含大写字母
    require_uppercase = false
    # 登录尝试失败次数限制
    max_login_attempts = 5
    # 登录锁定时间（分钟）
    login_lockout_minutes = 30

    [database]
    # 数据库路径，相对于应用数据目录
    path = "wolfquant.db"
    # 是否在启动时初始化数据库
    init_on_startup = true
    # 是否备份数据库
    enable_backup = true
    # 备份间隔（小时）
    backup_interval_hours = 24
    # 保留备份数量
    max_backups = 5

    [logging]
    # 是否启用日志
    enable = true
    # 日志级别: "error", "warn", "info", "debug", "trace"
    level = "info"
    # 是否输出到文件
    log_to_file = true
    # 日志文件路径，相对于应用数据目录
    file_path = "logs/app.log"
    # 日志文件最大大小（MB）
    max_file_size_mb = 10
    # 保留日志文件数量
    max_files = 5

    [email]
    # 是否启用邮件功能
    enable = false
    # SMTP服务器
    smtp_server = "smtp.example.com"
    # SMTP端口
    smtp_port = 587
    # 是否使用SSL/TLS
    use_tls = true
    # 发件人邮箱
    from_email = "noreply@example.com"
    # 发件人名称
    from_name = "WolfQuant"
    # SMTP用户名
    username = "your-email@example.com"
    # SMTP密码
    password = "your-email-password"
    # 邮件发送超时时间（秒）
    timeout_seconds = 10
    "#;

    fs::write(path, default_config)
        .map_err(|e| format!("Failed to write default config file: {}", e))?;

    Ok(())
}

// 获取配置实例
pub fn get_config() -> Result<Config, String> {
    let config = CONFIG.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}
