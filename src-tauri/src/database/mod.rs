use crate::config::Config;
use crate::error::auth::AuthError;
use crate::utils::crypto::hash_password;
use lazy_static::lazy_static;
use log::{error, info, warn};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result as SqlResult, Transaction};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use rusqlite::{params};
use chrono::{Duration, Utc};

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

    // 加载表结构定义
    static ref TABLE_SCHEMAS: HashMap<String, String> = {
        load_table_schemas().expect("Failed to load table schemas")
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

    //插入一个默认的用户
    // 创建用户
    conn.execute(
        "INSERT INTO users (id,username, email, password_hash, email_verified, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6,?7)",
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
        [version, now as u32],
    )
    .map_err(|e| {
        error!("Failed to set database version: {}", e);
        AuthError::DatabaseError(format!("设置数据库版本失败: {}", e))
    })?;

    Ok(())
}

/// 创建所有表(在事务中执行)
fn create_tables(tx: &Transaction) -> Result<(), AuthError> {
    // 从外部加载的表结构定义执行建表操作
    for (table_name, schema) in TABLE_SCHEMAS.iter() {
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

/// 数据库迁移
fn migrate_database(conn: &mut Connection, current_version: u32) -> Result<(), AuthError> {
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

/// 从外部文件加载表结构定义
fn load_table_schemas() -> Result<HashMap<String, String>, AuthError> {
    let mut schemas = HashMap::new();

    // 获取schema目录路径
    let config = Config::get();
    let schema_dir = if config.database.schema_dir.is_empty() {
        Path::new("data/schemas")
    } else {
        Path::new(&config.database.schema_dir)
    };

    // 确保目录存在
    if !schema_dir.exists() {
        fs::create_dir_all(schema_dir)
            .map_err(|e| AuthError::DatabaseError(format!("无法创建schema目录: {}", e)))?;

        // 如果是新创建的目录，生成默认schema文件
        generate_default_schemas(schema_dir)?;
    }

    // 读取所有.sql文件
    for entry in fs::read_dir(schema_dir)
        .map_err(|e| AuthError::DatabaseError(format!("无法读取schema目录: {}", e)))?
    {
        let entry =
            entry.map_err(|e| AuthError::DatabaseError(format!("无法读取目录项: {}", e)))?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "sql") {
            // 从文件名获取表名
            let table_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| AuthError::DatabaseError("无效的schema文件名".to_string()))?
                .to_string();

            // 读取文件内容
            let mut file = fs::File::open(&path)
                .map_err(|e| AuthError::DatabaseError(format!("无法打开schema文件: {}", e)))?;

            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| AuthError::DatabaseError(format!("无法读取schema文件: {}", e)))?;

            // 添加到schemas
            schemas.insert(table_name, content);
        }
    }

    if schemas.is_empty() {
        warn!("没有找到schema定义文件，将使用默认schema");
        generate_default_schemas(schema_dir)?;
        return load_table_schemas(); // 递归调用以加载生成的默认schema
    }

    Ok(schemas)
}

/// 生成默认schema文件
fn generate_default_schemas(schema_dir: &Path) -> Result<(), AuthError> {
    // 用户相关表
    let user_tables = [
        (
            "users",
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            email_verified BOOLEAN NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        ),
        (
            "sessions",
            "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )",
        ),
        (
            "password_reset_tokens",
            "CREATE TABLE IF NOT EXISTS password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT UNIQUE NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )",
        ),
        (
            "email_verification_codes",
            "CREATE TABLE IF NOT EXISTS email_verification_codes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            code TEXT NOT NULL,
            purpose TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            UNIQUE (email, purpose)
        )",
        ),
    ];

    // 资产相关表
    let asset_tables = [
        (
            "asset_types",
            "CREATE TABLE IF NOT EXISTS asset_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT
        )",
        ),
        (
            "user_groups",
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
        ),
        (
            "assets",
            "CREATE TABLE IF NOT EXISTS assets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            group_id INTEGER,
            asset_type_id INTEGER NOT NULL,
            code TEXT NOT NULL,
            name TEXT NOT NULL,
            current_price REAL,
            position_amount REAL DEFAULT 0,
            position_cost REAL DEFAULT 0,
            last_updated INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (group_id) REFERENCES user_groups (id) ON DELETE SET NULL,
            FOREIGN KEY (asset_type_id) REFERENCES asset_types (id) ON DELETE CASCADE,
            UNIQUE (user_id, asset_type_id, code)
        )",
        ),
        (
            "price_history",
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
        ),
    ];

    // 交易相关表
    let transaction_tables = [
        (
            "transactions",
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
        ),
        (
            "investment_plans",
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
        ),
        (
            "investment_strategies",
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
        ),
        (
            "strategy_applications",
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
        ),
        (
            "trade_alerts",
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
        ),
    ];

    // 合并所有表定义
    let all_tables = [&user_tables[..], &asset_tables[..], &transaction_tables[..]].concat();

    // 写入文件
    for (table_name, schema) in all_tables {
        let file_path = schema_dir.join(format!("{}.sql", table_name));
        fs::write(&file_path, schema)
            .map_err(|e| AuthError::DatabaseError(format!("无法写入schema文件: {}", e)))?;

        info!("生成默认schema文件: {:?}", file_path);
    }

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

/// 验证并更新表结构
fn verify_and_update_table_schemas(conn: &mut Connection) -> Result<(), AuthError> {
    let tx = conn.transaction()?;

    for (table_name, expected_schema) in TABLE_SCHEMAS.iter() {
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
