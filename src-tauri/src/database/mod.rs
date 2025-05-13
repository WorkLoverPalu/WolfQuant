use crate::config::Config;
use crate::error::auth::AuthError;
use lazy_static::lazy_static;
use log::{error, info};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result as SqlResult, Transaction};
use std::fs;
use std::path::Path;
use std::sync::Arc;

// 连接池静态变量
lazy_static! {
    static ref CONNECTION_POOL: Pool<SqliteConnectionManager> = {
        let config = Config::get();
        let db_path = Path::new(&config.database.path);

        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create database directory");
            }
        }

        let manager = SqliteConnectionManager::file(db_path)
            .with_init(|conn| {
                conn.pragma_update(None, "foreign_keys", &1)?; // 启用外键约束
                Ok(())
            });

        Pool::builder()
            .max_size(config.database.max_size) // 最大连接数
            .build(manager)
            .expect("Failed to create connection pool")
    };
}

/// 获取数据库连接
pub fn get_db_connection() -> Result<Connection, AuthError> {
    CONNECTION_POOL.get().map_err(|e| {
        error!("Failed to get database connection from pool: {}", e);
        AuthError::DatabaseError(format!("无法获取数据库连接: {}", e))
    })
}

/// 获取数据库连接(从连接池)
pub fn get_connection_from_pool(
) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, AuthError> {
    CONNECTION_POOL.get().map_err(|e| {
        error!("Failed to get database connection from pool: {}", e);
        AuthError::DatabaseError(format!("无法获取数据库连接: {}", e))
    })
}

/// 初始化数据库(带事务和迁移)
pub fn init_database() -> Result<(), AuthError> {
    let config = Config::get();
    let conn = get_db_connection()?;

    // 启用外键约束
    conn.pragma_update(None, "foreign_keys", &1)?;

    // 检查数据库版本
    let version = get_database_version(&conn)?;

    if version == 0 {
        // 新数据库，执行完整初始化
        let tx = conn.transaction()?;

        // 创建版本表
        tx.execute(
            "CREATE TABLE IF NOT EXISTS db_version (
                version INTEGER PRIMARY KEY NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        // 执行所有建表操作
        create_tables(&tx)?;

        // 初始化数据
        initialize_data(&tx)?;

        // 设置版本号
        set_database_version(&tx, config.database.version)?;

        tx.commit()?;
        info!("Database initialized successfully");
    } else if version < config.database.version {
        // 执行迁移
        migrate_database(&conn, version)?;
        info!(
            "Database migrated from version {} to {}",
            version, config.database.version
        );
    }

    Ok(())
}

/// 获取当前数据库版本
fn get_database_version(conn: &Connection) -> Result<u32, AuthError> {
    // 检查版本表是否存在
    let table_exists: i32 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='db_version'",
        [],
        |row| row.get(0),
    )?;

    if table_exists == 0 {
        return Ok(0);
    }

    // 获取版本号
    conn.query_row("SELECT version FROM db_version LIMIT 1", [], |row| {
        row.get(0)
    })
    .map_err(|e| {
        error!("Failed to get database version: {}", e);
        AuthError::DatabaseError(format!("获取数据库版本失败: {}", e))
    })
}

/// 设置数据库版本
fn set_database_version(conn: &Connection, version: u32) -> Result<(), AuthError> {
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT OR REPLACE INTO db_version (version, updated_at) VALUES (?, ?)",
        [version, now],
    )
    .map_err(|e| {
        error!("Failed to set database version: {}", e);
        AuthError::DatabaseError(format!("设置数据库版本失败: {}", e))
    })?;

    Ok(())
}

/// 创建所有表(在事务中执行)
fn create_tables(tx: &Transaction) -> Result<(), AuthError> {
    // 用户相关表
    tx.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        );
        
        CREATE TABLE IF NOT EXISTS password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        );
        
        CREATE TABLE IF NOT EXISTS email_verification_codes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            code TEXT NOT NULL,
            purpose TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            UNIQUE (email, purpose)
        );

        "#,
    )?;

    // 资产相关表
    tx.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS asset_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT
        );
        
        CREATE TABLE IF NOT EXISTS user_groups (
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
        );
        
        CREATE TABLE IF NOT EXISTS assets (
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
        );
        
        CREATE TABLE IF NOT EXISTS price_history (
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
        );
        "#,
    )?;

    // 交易相关表
    tx.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
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
        );
        
        CREATE TABLE IF NOT EXISTS investment_plans (
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
        );
        
        CREATE TABLE IF NOT EXISTS investment_strategies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            strategy_type TEXT NOT NULL,
            parameters TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        );
        
        CREATE TABLE IF NOT EXISTS strategy_applications (
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
        );
        
        CREATE TABLE IF NOT EXISTS trade_alerts (
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
        );
        "#,
    )?;

    Ok(())
}

/// 初始化数据(批量插入)
fn initialize_data(tx: &Transaction) -> Result<(), AuthError> {
    // 批量插入资产类型
    let asset_types = [
        ("FUND", "基金"),
        ("GOLD", "黄金"),
        ("CRYPTO", "数字货币"),
        ("STOCK", "股票"),
    ];

    let mut stmt =
        tx.prepare("INSERT OR IGNORE INTO asset_types (name, description) VALUES (?1, ?2)")?;

    for (name, description) in asset_types.iter() {
        stmt.execute([name, description])?;
    }

    Ok(())
}

/// 数据库迁移
fn migrate_database(conn: &Connection, current_version: u32) -> Result<(), AuthError> {
    let config = Config::get();
    let tx = conn.transaction()?;

    // 版本1的迁移(示例)
    if current_version < 1 {
        // 这里可以添加迁移逻辑
        // 例如修改表结构或转换数据
    }

    // 更新版本号
    set_database_version(&tx, config.database.version)?;
    tx.commit()?;

    Ok(())
}

/// 执行查询(使用连接池)
pub fn execute_query(query: &str, params: &[&dyn rusqlite::ToSql]) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;
    conn.execute(query, params).map_err(|e| {
        error!("Failed to execute query: {}", e);
        AuthError::DatabaseError(format!("执行数据库查询失败: {}", e))
    })?;
    Ok(())
}

/// 执行批量查询(在事务中)
pub fn execute_batch_queries(queries: &[(&str, &[&dyn rusqlite::ToSql])]) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;
    let tx = conn.transaction()?;

    for (query, params) in queries {
        tx.execute(query, params)?;
    }

    tx.commit()?;
    Ok(())
}
