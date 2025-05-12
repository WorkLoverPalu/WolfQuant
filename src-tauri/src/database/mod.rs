use crate::config::Config;
use crate::error::auth::AuthError;
use log::{error, info};
use rusqlite::{Connection, Result as SqlResult};
use std::fs;
use std::path::Path;

pub fn get_db_connection() -> Result<Connection, AuthError> {
    let config = Config::get();
    let db_path = Path::new(&config.database.path);

    // 确保目录存在
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                error!("Failed to create database directory: {}", e);
                AuthError::DatabaseError(format!("无法创建数据库目录: {}", e))
            })?;
        }
    }

    Connection::open(db_path).map_err(|e| {
        error!("Failed to open database connection: {}", e);
        AuthError::DatabaseError(format!("无法连接数据库: {}", e))
    })
}

pub fn init_database() -> Result<(), AuthError> {
    let conn = get_db_connection()?;

    // 创建用户表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create users table: {}", e);
        AuthError::DatabaseError(format!("创建用户表失败: {}", e))
    })?;

    // 创建会话表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create sessions table: {}", e);
        AuthError::DatabaseError(format!("创建会话表失败: {}", e))
    })?;

    // 创建密码重置令牌表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create password_reset_tokens table: {}", e);
        AuthError::DatabaseError(format!("创建密码重置令牌表失败: {}", e))
    })?;

    // 创建资产类型表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS asset_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create asset_types table: {}", e);
        AuthError::DatabaseError(format!("创建资产类型表失败: {}", e))
    })?;

    // 初始化资产类型
    let asset_types = [
        ("FUND", "基金"),
        ("GOLD", "黄金"),
        ("CRYPTO", "数字货币"),
        ("STOCK", "股票"),
    ];

    for (name, description) in asset_types.iter() {
        conn.execute(
            "INSERT OR IGNORE INTO asset_types (name, description) VALUES (?1, ?2)",
            [name, description],
        )
        .map_err(|e| {
            error!("Failed to insert asset type: {}", e);
            AuthError::DatabaseError(format!("初始化资产类型失败: {}", e))
        })?;
    }

    // 创建用户分组表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            asset_type_id INTEGER NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_type_id) REFERENCES asset_types (id) ON DELETE CASCADE,
            UNIQUE (user_id, name, asset_type_id)
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create user_groups table: {}", e);
        AuthError::DatabaseError(format!("创建用户分组表失败: {}", e))
    })?;

    // 创建资产表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS assets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            group_id INTEGER,
            asset_type_id INTEGER NOT NULL,
            code TEXT NOT NULL,
            name TEXT NOT NULL,
            current_price REAL,
            last_updated INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (group_id) REFERENCES user_groups (id) ON DELETE SET NULL,
            FOREIGN KEY (asset_type_id) REFERENCES asset_types (id) ON DELETE CASCADE,
            UNIQUE (user_id, asset_type_id, code)
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create assets table: {}", e);
        AuthError::DatabaseError(format!("创建资产表失败: {}", e))
    })?;

    // 创建交易记录表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            transaction_type TEXT NOT NULL,
            amount REAL NOT NULL,
            price REAL NOT NULL,
            total_cost REAL NOT NULL,
            transaction_date INTEGER NOT NULL,
            notes TEXT,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create transactions table: {}", e);
        AuthError::DatabaseError(format!("创建交易记录表失败: {}", e))
    })?;

    // 创建定投计划表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS investment_plans (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            frequency TEXT NOT NULL,
            day_of_week INTEGER,
            day_of_month INTEGER,
            amount REAL NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            last_executed INTEGER,
            next_execution INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create investment_plans table: {}", e);
        AuthError::DatabaseError(format!("创建定投计划表失败: {}", e))
    })?;

    // 创建投资策略表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS investment_strategies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            strategy_type TEXT NOT NULL,
            parameters TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create investment_strategies table: {}", e);
        AuthError::DatabaseError(format!("创建投资策略表失败: {}", e))
    })?;

    // 创建策略应用表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS strategy_applications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            strategy_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES investment_strategies (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            UNIQUE (strategy_id, asset_id)
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create strategy_applications table: {}", e);
        AuthError::DatabaseError(format!("创建策略应用表失败: {}", e))
    })?;

    // 创建历史价格表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS price_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            asset_id INTEGER NOT NULL,
            date INTEGER NOT NULL,
            open_price REAL,
            close_price REAL NOT NULL,
            high_price REAL,
            low_price REAL,
            volume REAL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            UNIQUE (asset_id, date)
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create price_history table: {}", e);
        AuthError::DatabaseError(format!("创建历史价格表失败: {}", e))
    })?;

    // 创建交易提醒表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS trade_alerts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            strategy_id INTEGER,
            alert_type TEXT NOT NULL,
            message TEXT NOT NULL,
            is_read BOOLEAN NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES investment_strategies (id) ON DELETE SET NULL
        )",
        [],
    )
    .map_err(|e| {
        error!("Failed to create trade_alerts table: {}", e);
        AuthError::DatabaseError(format!("创建交易提醒表失败: {}", e))
    })?;

    info!("Database initialized successfully");
    Ok(())
}

pub fn execute_query(query: &str, params: &[&dyn rusqlite::ToSql]) -> Result<(), AuthError> {
    let conn = get_db_connection()?;
    conn.execute(query, params).map_err(|e| {
        error!("Failed to execute query: {}", e);
        AuthError::DatabaseError(format!("执行数据库查询失败: {}", e))
    })?;
    Ok(())
}