use crate::config::Config;
use crate::error::AuthError;
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
