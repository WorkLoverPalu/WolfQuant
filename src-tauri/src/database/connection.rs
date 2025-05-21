use crate::config::Config;
use crate::database::migrations::{get_database_version, migrate_database, set_database_version};
use crate::database::schema::load_all_schemas;
use crate::error::auth::AuthError;
use crate::utils::crypto::hash_password;
use chrono::Utc;
use lazy_static::lazy_static;
use log::{error, info};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection, Transaction};
use std::fs;
use std::path::Path;

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

/// 获取数据库连接(从连接池)
pub fn get_connection_from_pool(
) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, AuthError> {
    CONNECTION_POOL.get().map_err(|e| {
        error!("Failed to get database connection from pool: {}", e);
        AuthError::DatabaseError(format!("无法获取数据库连接: {}", e))
    })
}

/// 初始化数据库(带事务、迁移和表结构验证)
pub fn init_database() -> Result<(), AuthError> {
    let config = Config::get();
    let mut conn = get_connection_from_pool()?;

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
        migrate_database(&mut conn, version)?;
        info!(
            "Database migrated from version {} to {}",
            version, config.database.version
        );
    }

    // 验证并更新表结构
    verify_and_update_table_schemas(&mut conn)?;

    // 插入一个默认的用户（如果不存在）
    let user_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = 10000)",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !user_exists {
        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, email_verified, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                10000,
                "nantang",
                "nantang@qq.com",
                hash_password("qwe123")?,
                1, // email_verified
                Utc::now().timestamp(),
                Utc::now().timestamp()
            ],
        )?;
        info!("Default user created");
    }

    // 检查角色表是否存在且为空，如果为空则初始化角色
    let role_table_exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='roles')",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if role_table_exists {
        let role_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM roles", [], |row| row.get(0))
            .unwrap_or(0);
        if role_count == 0 {
            conn.execute(
                "INSERT OR IGNORE INTO roles (name, description, created_at, updated_at)
                 VALUES 
                    ('user', '普通用户', strftime('%s', 'now'), strftime('%s', 'now')),
                    ('admin', '管理员', strftime('%s', 'now'), strftime('%s', 'now')),
                    ('observer', '观察者', strftime('%s', 'now'), strftime('%s', 'now'))",
                [],
            )?;
            info!("Default roles initialized");
        }
    }

    Ok(())
}

/// 创建所有表(在事务中执行)
fn create_tables(tx: &Transaction) -> Result<(), AuthError> {
    // 加载所有模块的表结构定义
    let schemas = load_all_schemas()?;

    // 执行建表操作
    for (table_name, schema) in schemas.iter() {
        tx.execute_batch(schema)?;
        info!("Created table: {}", table_name);
    }

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

/// 验证并更新表结构
fn verify_and_update_table_schemas(conn: &mut Connection) -> Result<(), AuthError> {
    let schemas = load_all_schemas()?;
    let tx = conn.transaction()?;

    for (table_name, expected_schema) in schemas.iter() {
        match get_current_table_schema(&tx, table_name)? {
            Some(current_schema) => {
                // 规范化SQL以便比较
                let normalized_current = normalize_sql(&current_schema);
                let normalized_expected = normalize_sql(expected_schema);

                if normalized_current != normalized_expected {
                    info!("表 {} 结构不匹配，进行更新", table_name);
                    update_table_structure(&tx, table_name, &current_schema, expected_schema)?;
                }
            }
            None => {
                // 表不存在，创建它
                info!("表 {} 不存在，创建新表", table_name);
                tx.execute_batch(expected_schema)?;
            }
        }
    }

    tx.commit()?;
    Ok(())
}

/// 获取表的当前结构
fn get_current_table_schema(
    conn: &Connection,
    table_name: &str,
) -> Result<Option<String>, AuthError> {
    let sql = "SELECT sql FROM sqlite_master WHERE type='table' AND name=?";

    let result: Result<String, rusqlite::Error> =
        conn.query_row(sql, [table_name], |row| row.get(0));

    match result {
        Ok(schema) => Ok(Some(schema)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AuthError::DatabaseError(format!("获取表结构失败: {}", e))),
    }
}

/// 规范化SQL语句以便比较
fn normalize_sql(sql: &str) -> String {
    sql.to_lowercase()
        .replace("\n", " ")
        .replace("\t", " ")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// 更新表结构
fn update_table_structure(
    tx: &Transaction,
    table_name: &str,
    current_schema: &str,
    expected_schema: &str,
) -> Result<(), AuthError> {
    // 获取表中的数据
    let temp_table_name = format!("{}_temp", table_name);

    // 1. 获取当前表的列
    let mut stmt = tx.prepare(&format!("PRAGMA table_info({})", table_name))?;

    let current_columns: Vec<String> = stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?
        .filter_map(Result::ok)
        .collect();

    // 2. 获取期望表的列
    tx.execute_batch(&format!("DROP TABLE IF EXISTS {}", temp_table_name))?;
    tx.execute_batch(&expected_schema.replace(
        &format!("CREATE TABLE {}", table_name),
        &format!("CREATE TABLE {}", temp_table_name),
    ))?;

    let mut stmt = tx.prepare(&format!("PRAGMA table_info({})", temp_table_name))?;

    let expected_columns: Vec<String> = stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            Ok(name)
        })?
        .filter_map(Result::ok)
        .collect();

    // 3. 找出共同的列
    let common_columns: Vec<String> = current_columns
        .iter()
        .filter(|col| expected_columns.contains(col))
        .cloned()
        .collect();

    let columns_str = common_columns.join(", ");

    // 4. 创建新表并迁移数据
    tx.execute_batch(&format!(
        "
        CREATE TABLE {}_new {};
        INSERT INTO {}_new ({}) 
        SELECT {} FROM {};
        DROP TABLE {};
        ALTER TABLE {}_new RENAME TO {};
    ",
        table_name,
        &expected_schema[expected_schema.find("(").unwrap()..],
        table_name,
        columns_str,
        columns_str,
        table_name,
        table_name,
        table_name,
        table_name
    ))?;

    // 5. 清理临时表
    tx.execute_batch(&format!("DROP TABLE IF EXISTS {}", temp_table_name))?;

    info!("表 {} 结构已更新", table_name);
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
    let mut conn = get_connection_from_pool()?;
    let tx = conn.transaction()?;

    for (query, params) in queries {
        tx.execute(query, *params)?;
    }

    tx.commit()?;
    Ok(())
}
