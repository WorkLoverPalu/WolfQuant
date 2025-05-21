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
        )"
        .to_string(),
    );

    // JWT 令牌表
    schemas.insert(
        "jwt_tokens".to_string(),
        "CREATE TABLE IF NOT EXISTS jwt_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT NOT NULL UNIQUE,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"
        .to_string(),
    );
    // 创建已撤销令牌表
    schemas.insert(
        "revoked_tokens".to_string(),
        "CREATE TABLE IF NOT EXISTS revoked_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            token TEXT NOT NULL UNIQUE,
            revoked_at INTEGER NOT NULL
        )"
        .to_string(),
    );
    // 创建角色表
    schemas.insert(
        "roles".to_string(),
        "CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )"
        .to_string(),
    );
    // 创建用户角色关联表
    schemas.insert(
        "user_roles".to_string(),
        "CREATE TABLE IF NOT EXISTS user_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            role_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
            UNIQUE(user_id, role_id)
        )"
        .to_string(),
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
        )"
        .to_string(),
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
        )"
        .to_string(),
    );

    // 添加索引以提高查询性能
    schemas.insert(
        "idx_jwt_tokens_user_id".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_jwt_tokens_user_id ON jwt_tokens(user_id)".to_string(),
    );
    schemas.insert(
        "idx_jwt_tokens_token".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_jwt_tokens_token ON jwt_tokens(token)".to_string(),
    );
    schemas.insert(
        "idx_revoked_tokens_token".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_revoked_tokens_token ON revoked_tokens(token)".to_string(),
    );
    schemas.insert(
        "idx_user_roles_user_id".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_user_roles_user_id ON user_roles(user_id)".to_string(),
    );
    schemas.insert(
        "idx_user_roles_role_id".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_user_roles_role_id ON user_roles(role_id)".to_string(),
    );

    schemas
}
