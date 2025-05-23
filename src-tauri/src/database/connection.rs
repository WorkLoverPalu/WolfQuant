use crate::config::Config;
use crate::database::migrations::{get_database_version, migrate_database, set_database_version};
use crate::database::schema::load_all_schemas;
use crate::error::auth::AuthError;
use crate::utils::crypto::hash_password;
use chrono::Utc;
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection, Transaction};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// 数据库连接池类型别名
pub type DbPool = Pool<SqliteConnectionManager>;
pub type DbConnection = r2d2::PooledConnection<SqliteConnectionManager>;

/// 数据库管理器
pub struct DatabaseManager {
    pool: Arc<DbPool>,
}

impl DatabaseManager {
    /// 创建新的数据库管理器实例
    pub fn new() -> Result<Self, AuthError> {
        let pool = Arc::new(create_connection_pool()?);
        Ok(Self { pool })
    }

    /// 获取数据库连接
    pub fn get_connection(&self) -> Result<DbConnection, AuthError> {
        self.pool.get().map_err(|e| {
            error!("Failed to get database connection from pool: {}", e);
            AuthError::DatabaseError(format!("无法获取数据库连接: {}", e))
        })
    }

    /// 初始化数据库
    pub fn initialize(&self) -> Result<(), AuthError> {
        let config = Config::get();
        let mut conn = self.get_connection()?;

        // 启用外键约束和其他优化设置
        configure_connection(&mut conn)?;

        // 检查数据库版本并执行相应操作
        let current_version = get_database_version(&conn)?;

        match current_version {
            0 => self.initialize_new_database(&mut conn)?,
            v if v < config.database.version as u32 => self.migrate_database(&mut conn, v as u32)?,
            _ => debug!("Database is up to date (version {})", current_version),
        }

        // 验证并更新表结构
        self.verify_and_update_schemas(&mut conn)?;

        // 初始化默认数据
        self.ensure_default_data(&mut conn)?;

        info!("Database initialization completed successfully");
        Ok(())
    }

    /// 初始化新数据库
    fn initialize_new_database(&self, conn: &mut DbConnection) -> Result<(), AuthError> {
        let config = Config::get();
        let tx = conn.transaction()?;

        info!("Initializing new database...");

        // 创建版本表
        self.create_version_table(&tx)?;

        // 创建所有表
        self.create_all_tables(&tx)?;

        // 初始化基础数据
        self.initialize_base_data(&tx)?;

        // 设置数据库版本
        set_database_version(&tx, config.database.version)?;

        tx.commit()?;
        info!("New database initialized successfully");
        Ok(())
    }

    /// 执行数据库迁移
    fn migrate_database(
        &self,
        conn: &mut DbConnection,
        from_version: u32,
    ) -> Result<(), AuthError> {
        let config = Config::get();
        info!(
            "Migrating database from version {} to {}",
            from_version, config.database.version
        );

        migrate_database(conn, from_version)?;

        info!("Database migration completed successfully");
        Ok(())
    }

    /// 创建版本表
    fn create_version_table(&self, tx: &Transaction) -> Result<(), AuthError> {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS db_version (
                version INTEGER PRIMARY KEY NOT NULL,
                updated_at INTEGER NOT NULL,
                description TEXT
            )",
            [],
        )?;
        Ok(())
    }

    /// 创建所有表
    fn create_all_tables(&self, tx: &Transaction) -> Result<(), AuthError> {
        let schemas = load_all_schemas()?;

        for (table_name, schema) in schemas.iter() {
            match tx.execute_batch(schema) {
                Ok(_) => {
                    debug!("Successfully created table: {}", table_name);
                }
                Err(e) => {
                    error!("Failed to create table {}: {}", table_name, e);
                    return Err(AuthError::DatabaseError(format!(
                        "创建表 {} 失败: {}",
                        table_name, e
                    )));
                }
            }
        }

        info!("All tables created successfully");
        Ok(())
    }

    /// 初始化基础数据
    fn initialize_base_data(&self, tx: &Transaction) -> Result<(), AuthError> {
        // 初始化资产类型
        self.initialize_asset_types(tx)?;

        // 初始化角色
        self.initialize_roles(tx)?;

        info!("Base data initialized successfully");
        Ok(())
    }

    /// 初始化资产类型
    fn initialize_asset_types(&self, tx: &Transaction) -> Result<(), AuthError> {
        let asset_types = [
            ("STOCK", "股票", "股票资产"),
            ("FUND", "基金", "基金资产"),
            ("CRYPTO", "数字货币", "加密货币资产"),
            ("GOLD", "黄金", "贵金属资产"),
            ("FOREX", "外汇", "外汇资产"),
            ("BOND", "债券", "债券资产"),
        ];

        let mut stmt = tx.prepare(
            "INSERT OR IGNORE INTO asset_types (code, name, description, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;

        let now = Utc::now().timestamp();
        for (code, name, description) in asset_types.iter() {
            stmt.execute(params![code, name, description, now, now])?;
        }

        debug!("Asset types initialized");
        Ok(())
    }

    /// 初始化角色
    fn initialize_roles(&self, tx: &Transaction) -> Result<(), AuthError> {
        let roles = [
            ("user", "普通用户", "系统普通用户"),
            ("admin", "管理员", "系统管理员"),
            ("observer", "观察者", "只读用户"),
        ];

        let mut stmt = tx.prepare(
            "INSERT OR IGNORE INTO roles (name, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4)",
        )?;

        let now = Utc::now().timestamp();
        for (name, description, _) in roles.iter() {
            stmt.execute(params![name, description, now, now])?;
        }

        debug!("Roles initialized");
        Ok(())
    }

    /// 验证并更新表结构
    fn verify_and_update_schemas(&self, conn: &mut DbConnection) -> Result<(), AuthError> {
        let schemas = load_all_schemas()?;
        let tx = conn.transaction()?;

        for (table_name, expected_schema) in schemas.iter() {
            match self.get_table_schema(&tx, table_name)? {
                Some(current_schema) => {
                    if !self.schemas_match(&current_schema, expected_schema) {
                        warn!("Table {} schema mismatch, updating...", table_name);
                        self.update_table_schema(&tx, table_name, expected_schema)?;
                    } else {
                        debug!("Table {} schema is up to date", table_name);
                    }
                }
                None => {
                    info!("Table {} does not exist, creating...", table_name);
                    tx.execute_batch(expected_schema)?;
                }
            }
        }

        tx.commit()?;
        info!("Schema verification completed");
        Ok(())
    }

    /// 获取表结构
    fn get_table_schema(
        &self,
        conn: &Connection,
        table_name: &str,
    ) -> Result<Option<String>, AuthError> {
        let sql = "SELECT sql FROM sqlite_master WHERE type='table' AND name=?";

        match conn.query_row(sql, [table_name], |row| row.get::<_, String>(0)) {
            Ok(schema) => Ok(Some(schema)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AuthError::DatabaseError(format!("获取表结构失败: {}", e))),
        }
    }

    /// 比较两个表结构是否匹配
    fn schemas_match(&self, current: &str, expected: &str) -> bool {
        let normalized_current = normalize_sql(current);
        let normalized_expected = normalize_sql(expected);
        normalized_current == normalized_expected
    }

    /// 更新表结构
    fn update_table_schema(
        &self,
        tx: &Transaction,
        table_name: &str,
        new_schema: &str,
    ) -> Result<(), AuthError> {
        let backup_table = format!("{}_backup_{}", table_name, Utc::now().timestamp());

        // 1. 重命名原表为备份表
        tx.execute(
            &format!("ALTER TABLE {} RENAME TO {}", table_name, backup_table),
            [],
        )?;

        // 2. 创建新表
        tx.execute_batch(new_schema)?;

        // 3. 获取共同列并迁移数据
        let common_columns = self.get_common_columns(tx, &backup_table, table_name)?;

        if !common_columns.is_empty() {
            let columns_str = common_columns.join(", ");
            let migrate_sql = format!(
                "INSERT INTO {} ({}) SELECT {} FROM {}",
                table_name, columns_str, columns_str, backup_table
            );
            tx.execute(&migrate_sql, [])?;
        }

        // 4. 删除备份表
        tx.execute(&format!("DROP TABLE {}", backup_table), [])?;

        info!("Table {} schema updated successfully", table_name);
        Ok(())
    }

    /// 获取两个表的共同列
    fn get_common_columns(
        &self,
        tx: &Transaction,
        table1: &str,
        table2: &str,
    ) -> Result<Vec<String>, AuthError> {
        let get_columns = |table: &str| -> Result<Vec<String>, AuthError> {
            let mut stmt = tx.prepare(&format!("PRAGMA table_info({})", table))?;
            let columns: Result<Vec<String>, rusqlite::Error> =
                stmt.query_map([], |row| row.get::<_, String>(1))?.collect();
            Ok(columns?)
        };

        let columns1 = get_columns(table1)?;
        let columns2 = get_columns(table2)?;

        Ok(columns1
            .into_iter()
            .filter(|col| columns2.contains(col))
            .collect())
    }

    /// 确保默认数据存在
    fn ensure_default_data(&self, conn: &mut DbConnection) -> Result<(), AuthError> {
        // 确保默认用户存在
        self.ensure_default_user(conn)?;

        // 确保角色数据存在
        self.ensure_roles_data(conn)?;

        Ok(())
    }

    /// 确保默认用户存在
    fn ensure_default_user(&self, conn: &mut DbConnection) -> Result<(), AuthError> {
        let user_exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM users WHERE id = 10000)",
                [],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if !user_exists {
            let now = Utc::now().timestamp();
            conn.execute(
                "INSERT INTO users (id, username, email, password_hash, email_verified, created_at, updated_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    10000,
                    "nantang",
                    "nantang@qq.com",
                    hash_password("qwe123")?,
                    1,
                    now,
                    now
                ],
            )?;
            info!("Default user created");
        }

        Ok(())
    }

    /// 确保角色数据存在
    fn ensure_roles_data(&self, conn: &mut DbConnection) -> Result<(), AuthError> {
        let role_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM roles", [], |row| row.get(0))
            .unwrap_or(0);

        if role_count == 0 {
            let now = Utc::now().timestamp();
            conn.execute(
                "INSERT OR IGNORE INTO roles (name, description, created_at, updated_at)
                 VALUES 
                    ('user', '普通用户', ?1, ?2),
                    ('admin', '管理员', ?3, ?4),
                    ('observer', '观察者', ?5, ?6)",
                params![now, now, now, now, now, now],
            )?;
            info!("Default roles initialized");
        }

        Ok(())
    }
}

/// 创建数据库连接池
fn create_connection_pool() -> Result<DbPool, AuthError> {
    let config = Config::get();
    let db_path = Path::new(&config.database.path);

    // 确保数据库目录存在
    ensure_database_directory(db_path)?;

    let manager = SqliteConnectionManager::file(db_path).with_init(|conn| {
        configure_connection(conn)?;
        Ok(())
    });

    Pool::builder()
        .max_size(config.database.max_size)
        .connection_timeout(Duration::from_secs(30))
        .idle_timeout(Some(Duration::from_secs(600)))
        .max_lifetime(Some(Duration::from_secs(1800)))
        .build(manager)
        .map_err(|e| {
            error!("Failed to create connection pool: {}", e);
            AuthError::DatabaseError(format!("创建连接池失败: {}", e))
        })
}

/// 确保数据库目录存在
fn ensure_database_directory(db_path: &Path) -> Result<(), AuthError> {
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                error!("Failed to create database directory: {}", e);
                AuthError::DatabaseError(format!("创建数据库目录失败: {}", e))
            })?;
            info!("Created database directory: {:?}", parent);
        }
    }
    Ok(())
}

/// 配置数据库连接
fn configure_connection(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    // 启用外键约束
    conn.pragma_update(None, "foreign_keys", &1)?;

    // 设置WAL模式以提高并发性能
    conn.pragma_update(None, "journal_mode", &"WAL")?;

    // 设置同步模式
    conn.pragma_update(None, "synchronous", &"NORMAL")?;

    // 设置缓存大小 (10MB)
    conn.pragma_update(None, "cache_size", &-10000)?;

    // 设置临时存储为内存
    conn.pragma_update(None, "temp_store", &"MEMORY")?;

    // 设置mmap大小 (256MB)
    conn.pragma_update(None, "mmap_size", &268435456)?;

    Ok(())
}

/// 规范化SQL语句以便比较
fn normalize_sql(sql: &str) -> String {
    sql.to_lowercase()
        .replace('\n', " ")
        .replace('\t', " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_string()
}

// 连接池静态变量
lazy_static! {
    static ref DATABASE_MANAGER: DatabaseManager =
        { DatabaseManager::new().expect("Failed to create database manager") };
}

/// 获取数据库连接(从连接池)
pub fn get_connection_from_pool() -> Result<DbConnection, AuthError> {
    DATABASE_MANAGER.get_connection()
}

/// 初始化数据库
pub fn init_database() -> Result<(), AuthError> {
    DATABASE_MANAGER.initialize()
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
    let mut conn = get_connection_from_pool()?;
    let tx = conn.transaction()?;

    for (query, params) in queries {
        tx.execute(query, *params).map_err(|e| {
            error!("Failed to execute batch query: {}", e);
            AuthError::DatabaseError(format!("执行批量查询失败: {}", e))
        })?;
    }

    tx.commit().map_err(|e| {
        error!("Failed to commit batch transaction: {}", e);
        AuthError::DatabaseError(format!("提交批量事务失败: {}", e))
    })?;

    Ok(())
}

/// 执行查询并返回结果
pub fn query_one<T, F>(
    query: &str,
    params: &[&dyn rusqlite::ToSql],
    mapper: F,
) -> Result<Option<T>, AuthError>
where
    F: FnOnce(&rusqlite::Row) -> Result<T, rusqlite::Error>,
{
    let conn = get_connection_from_pool()?;
    match conn.query_row(query, params, mapper) {
        Ok(result) => Ok(Some(result)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => {
            error!("Failed to execute query: {}", e);
            Err(AuthError::DatabaseError(format!("查询失败: {}", e)))
        }
    }
}

/// 执行查询并返回多个结果
pub fn query_many<T, F>(
    query: &str,
    params: &[&dyn rusqlite::ToSql],
    mapper: F,
) -> Result<Vec<T>, AuthError>
where
    F: Fn(&rusqlite::Row) -> Result<T, rusqlite::Error>,
{
    let conn = get_connection_from_pool()?;
    let mut stmt = conn.prepare(query).map_err(|e| {
        error!("Failed to prepare query: {}", e);
        AuthError::DatabaseError(format!("准备查询失败: {}", e))
    })?;

    let results: Result<Vec<T>, rusqlite::Error> = stmt.query_map(params, mapper)?.collect();

    results.map_err(|e| {
        error!("Failed to execute query: {}", e);
        AuthError::DatabaseError(format!("查询失败: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_normalize_sql() {
        let sql1 = "CREATE TABLE test (\n    id INTEGER PRIMARY KEY,\n    name TEXT\n)";
        let sql2 = "CREATE TABLE test ( id INTEGER PRIMARY KEY, name TEXT )";

        assert_eq!(normalize_sql(sql1), normalize_sql(sql2));
    }

    #[test]
    fn test_database_manager_creation() {
        let _manager = DatabaseManager::new();
        // 测试应该不会panic
    }

    #[test]
    fn test_ensure_database_directory() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("subdir").join("test.db");

        ensure_database_directory(&db_path).unwrap();
        assert!(db_path.parent().unwrap().exists());
    }
}
