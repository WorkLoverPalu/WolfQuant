use std::collections::HashMap;

/// 获取用户相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();
    
    // 用户表
    schemas.insert(
        "users".to_string(),
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email_verified BOOLEAN NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )".to_string(),
    );
    
    // 会话表
    schemas.insert(
        "sessions".to_string(),
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )".to_string(),
    );
    
    // 密码重置令牌表
    schemas.insert(
        "password_reset_tokens".to_string(),
        "CREATE TABLE IF NOT EXISTS password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )".to_string(),
    );
    
    // 邮箱验证码表
    schemas.insert(
        "email_verification_codes".to_string(),
        "CREATE TABLE IF NOT EXISTS email_verification_codes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            code TEXT NOT NULL,
            purpose TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            UNIQUE (email, purpose)
        )".to_string(),
    );
    
    schemas
}
