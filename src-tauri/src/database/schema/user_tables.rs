use std::collections::HashMap;

/// 数据库表名常量
pub mod tables {
    pub const USERS: &str = "users";
    pub const JWT_TOKENS: &str = "jwt_tokens";
    pub const REVOKED_TOKENS: &str = "revoked_tokens";
    pub const ROLES: &str = "roles";
    pub const USER_ROLES: &str = "user_roles";
    pub const PASSWORD_RESET_TOKENS: &str = "password_reset_tokens";
    pub const EMAIL_VERIFICATION_CODES: &str = "email_verification_codes";
}

/// 索引名常量
pub mod indexes {
    pub const IDX_JWT_TOKENS_USER_ID: &str = "idx_jwt_tokens_user_id";
    pub const IDX_JWT_TOKENS_TOKEN: &str = "idx_jwt_tokens_token";
    pub const IDX_REVOKED_TOKENS_TOKEN: &str = "idx_revoked_tokens_token";
    pub const IDX_USER_ROLES_USER_ID: &str = "idx_user_roles_user_id";
    pub const IDX_USER_ROLES_ROLE_ID: &str = "idx_user_roles_role_id";
}

/// 获取用户相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();

    // 添加表定义
    add_table_schemas(&mut schemas);
    
    // 添加索引定义
    add_index_schemas(&mut schemas);
    
    // 添加默认角色数据
    add_default_data(&mut schemas);

    schemas
}

/// 添加表定义
fn add_table_schemas(schemas: &mut HashMap<String, String>) {
    // 用户表 - 存储用户基本信息
    schemas.insert(
        tables::USERS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email_verified BOOLEAN NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )"#.to_string(),
    );

    // JWT 令牌表 - 存储有效的 JWT 令牌
    schemas.insert(
        tables::JWT_TOKENS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS jwt_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT NOT NULL UNIQUE,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 已撤销令牌表 - 存储已撤销的 JWT 令牌
    schemas.insert(
        tables::REVOKED_TOKENS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS revoked_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            token TEXT NOT NULL UNIQUE,
            revoked_at INTEGER NOT NULL
        )"#.to_string(),
    );
    
    // 角色表 - 定义系统中的角色
    schemas.insert(
        tables::ROLES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )"#.to_string(),
    );
    
    // 用户角色关联表 - 定义用户与角色的多对多关系
    schemas.insert(
        tables::USER_ROLES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS user_roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            role_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
            UNIQUE(user_id, role_id)
        )"#.to_string(),
    );

    // 密码重置令牌表 - 存储密码重置令牌
    schemas.insert(
        tables::PASSWORD_RESET_TOKENS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )"#.to_string(),
    );

    // 邮箱验证码表 - 存储邮箱验证码
    schemas.insert(
        tables::EMAIL_VERIFICATION_CODES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS email_verification_codes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            code TEXT NOT NULL,
            purpose TEXT NOT NULL,
            is_used BOOLEAN NOT NULL DEFAULT 0,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            UNIQUE (email, purpose)
        )"#.to_string(),
    );
}

/// 添加索引定义
fn add_index_schemas(schemas: &mut HashMap<String, String>) {
    // JWT 令牌表索引
    schemas.insert(
        indexes::IDX_JWT_TOKENS_USER_ID.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON jwt_tokens(user_id)", indexes::IDX_JWT_TOKENS_USER_ID),
    );
    schemas.insert(
        indexes::IDX_JWT_TOKENS_TOKEN.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON jwt_tokens(token)", indexes::IDX_JWT_TOKENS_TOKEN),
    );
    
    // 已撤销令牌表索引
    schemas.insert(
        indexes::IDX_REVOKED_TOKENS_TOKEN.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON revoked_tokens(token)", indexes::IDX_REVOKED_TOKENS_TOKEN),
    );
    
    // 用户角色关联表索引
    schemas.insert(
        indexes::IDX_USER_ROLES_USER_ID.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON user_roles(user_id)", indexes::IDX_USER_ROLES_USER_ID),
    );
    schemas.insert(
        indexes::IDX_USER_ROLES_ROLE_ID.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON user_roles(role_id)", indexes::IDX_USER_ROLES_ROLE_ID),
    );
}

/// 添加默认数据
fn add_default_data(schemas: &mut HashMap<String, String>) {
    // 添加默认角色
    schemas.insert(
        "default_roles".to_string(),
        r#"INSERT OR IGNORE INTO roles (name, description, created_at, updated_at)
        VALUES 
            ('user', '普通用户', strftime('%s', 'now'), strftime('%s', 'now')),
            ('admin', '管理员', strftime('%s', 'now'), strftime('%s', 'now')),
            ('moderator', '版主', strftime('%s', 'now'), strftime('%s', 'now'))"#.to_string(),
    );
}

/// 获取数据库迁移 SQL 脚本
pub fn get_migration_sql() -> String {
    let schemas = get_schemas();
    let mut sql = String::new();
    
    // 开始事务
    sql.push_str("BEGIN TRANSACTION;\n\n");
    
    // 添加所有表和索引
    for (_, schema) in schemas {
        sql.push_str(&schema);
        sql.push_str(";\n\n");
    }
    
    // 提交事务
    sql.push_str("COMMIT;");
    
    sql
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_schemas() {
        let schemas = get_schemas();
        
        // 验证所有表都已定义
        assert!(schemas.contains_key(&tables::USERS.to_string()));
        assert!(schemas.contains_key(&tables::JWT_TOKENS.to_string()));
        assert!(schemas.contains_key(&tables::REVOKED_TOKENS.to_string()));
        assert!(schemas.contains_key(&tables::ROLES.to_string()));
        assert!(schemas.contains_key(&tables::USER_ROLES.to_string()));
        assert!(schemas.contains_key(&tables::PASSWORD_RESET_TOKENS.to_string()));
        assert!(schemas.contains_key(&tables::EMAIL_VERIFICATION_CODES.to_string()));
        
        // 验证所有索引都已定义
        assert!(schemas.contains_key(&indexes::IDX_JWT_TOKENS_USER_ID.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_JWT_TOKENS_TOKEN.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_REVOKED_TOKENS_TOKEN.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_USER_ROLES_USER_ID.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_USER_ROLES_ROLE_ID.to_string()));
        
        // 验证默认数据已定义
        assert!(schemas.contains_key("default_roles"));
    }
    
    #[test]
    fn test_get_migration_sql() {
        let sql = get_migration_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含所有表
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS users"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS jwt_tokens"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS roles"));
        
        // 验证 SQL 脚本包含默认数据
        assert!(sql.contains("INSERT OR IGNORE INTO roles"));
    }
}