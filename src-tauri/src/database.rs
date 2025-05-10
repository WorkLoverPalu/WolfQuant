use crate::config;
use chrono::Utc;
use log::{error, info};
use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use std::sync::Mutex;

// 使用Lazy静态变量来存储数据库连接
pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let db_path = get_database_path().expect("Failed to get database path");
    let conn = Connection::open(&db_path).expect("Failed to open database");
    
    if should_init_database() {
        initialize_database(&conn).expect("Failed to initialize database");
    }
    
    Mutex::new(conn)
});

// 获取数据库路径
fn get_database_path() -> Result<std::path::PathBuf, String> {
    let config = config::get_config()?;
    
    let app_dir = tauri::api::path::app_dir(tauri::Config::default().package.name.as_str())
        .ok_or_else(|| "Failed to get app directory".to_string())?;
    
    // 确保目录存在
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create app directory: {}", e))?;
    }
    
    Ok(app_dir.join(&config.database.path))
}

// 是否应该初始化数据库
fn should_init_database() -> bool {
    match config::get_config() {
        Ok(config) => config.database.init_on_startup,
        Err(_) => true, // 默认初始化
    }
}

// 初始化数据库表
fn initialize_database(conn: &Connection) -> Result<(), rusqlite::Error> {
    info!("Initializing database tables");
    
    // 创建用户表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email_verified BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            last_login TEXT,
            login_attempts INTEGER NOT NULL DEFAULT 0,
            locked_until TEXT
        )",
        [],
    )?;

    // 创建密码重置令牌表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_reset_tokens (
            token TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;
    
    // 创建邮箱验证码表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS email_verification_codes (
            code TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            email TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;
    
    // 创建会话表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            ip_address TEXT,
            user_agent TEXT,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )",
        [],
    )?;

    info!("Database tables initialized successfully");
    Ok(())
}

// 初始化数据库
pub fn init_database() -> Result<(), String> {
    info!("Initializing database");
    
    // 仅仅访问DB静态变量就会触发初始化
    let _lock = DB.lock().map_err(|e| {
        let err_msg = format!("Failed to lock database: {}", e);
        error!("{}", err_msg);
        err_msg
    })?;
    
    // 如果配置了备份，检查是否需要创建备份
    if let Ok(config) = config::get_config() {
        if config.database.enable_backup {
            backup_database()?;
        }
    }
    
    info!("Database initialized successfully");
    Ok(())
}

// 备份数据库
fn backup_database() -> Result<(), String> {
    let config = config::get_config()?;
    
    if !config.database.enable_backup {
        return Ok(());
    }
    
    let db_path = get_database_path()?;
    
    // 如果数据库文件不存在，不需要备份
    if !db_path.exists() {
        return Ok(());
    }
    
    let app_dir = tauri::api::path::app_dir(tauri::Config::default().package.name.as_str())
        .ok_or_else(|| "Failed to get app directory".to_string())?;
    
    let backup_dir = app_dir.join("backups");
    
    // 确保备份目录存在
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;
    }
    
    // 创建备份文件名，包含时间戳
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!("backup_{}.db", timestamp);
    let backup_path = backup_dir.join(&backup_filename);
    
    // 复制数据库文件
    fs::copy(&db_path, &backup_path)
        .map_err(|e| format!("Failed to create database backup: {}", e))?;
    
    info!("Database backup created: {}", backup_filename);
    
    // 清理旧备份
    cleanup_old_backups(&backup_dir, config.database.max_backups)?;
    
    Ok(())
}

// 清理旧备份
fn cleanup_old_backups(backup_dir: &Path, max_backups: usize) -> Result<(), String> {
    // 如果max_backups为0，不限制备份数量
    if max_backups == 0 {
        return Ok(());
    }
    
    // 获取所有备份文件
    let mut backups = fs::read_dir(backup_dir)
        .map_err(|e| format!("Failed to read backup directory: {}", e))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            
            // 只考虑.db文件
            if path.extension()? != "db" {
                return None;
            }
            
            // 获取文件修改时间
            let metadata = fs::metadata(&path).ok()?;
            let modified = metadata.modified().ok()?;
            
            Some((path, modified))
        })
        .collect::<Vec<_>>();
    
    // 按修改时间排序（最新的在前）
    backups.sort_by(|a, b| b.1.cmp(&a.1));
    
    // 删除超出限制的旧备份
    if backups.len() > max_backups {
        for (path, _) in backups.iter().skip(max_backups) {
            if let Err(e) = fs::remove_file(path) {
                error!("Failed to delete old backup {}: {}", path.display(), e);
            } else {
                info!("Deleted old backup: {}", path.display());
            }
        }
    }
    
    Ok(())
}