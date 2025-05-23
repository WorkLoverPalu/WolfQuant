/// 数据库迁移
use crate::config::Config;
use crate::error::auth::AuthError;
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use rusqlite::{params, Connection, Transaction};
use std::collections::HashMap;
use std::fmt;

/// 迁移信息结构体
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: u32,
    pub name: String,
    pub description: String,
    pub up_sql: Vec<String>,
    pub down_sql: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Migration {
    /// 创建新的迁移
    pub fn new(
        version: u32,
        name: &str,
        description: &str,
        up_sql: Vec<String>,
        down_sql: Vec<String>,
    ) -> Self {
        Self {
            version,
            name: name.to_string(),
            description: description.to_string(),
            up_sql,
            down_sql,
            created_at: Utc::now(),
        }
    }
}

impl fmt::Display for Migration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Migration {} ({}): {}",
            self.version, self.name, self.description
        )
    }
}

/// 迁移历史记录
#[derive(Debug)]
pub struct MigrationRecord {
    pub version: u32,
    pub name: String,
    pub applied_at: DateTime<Utc>,
    pub execution_time_ms: u64,
    pub checksum: String,
}

/// 数据库迁移管理器
pub struct MigrationManager {
    migrations: HashMap<u32, Migration>,
}

impl MigrationManager {
    /// 创建新的迁移管理器
    pub fn new() -> Self {
        let mut manager = Self {
            migrations: HashMap::new(),
        };

        // 注册所有迁移
        manager.register_migrations();
        manager
    }

    /// 注册所有迁移
    fn register_migrations(&mut self) {
        // 迁移 1: 添加用户表增强字段
        self.add_migration(Migration::new(
            1,
            "add_user_enhancements",
            "为用户表添加增强字段",
            vec![
                "ALTER TABLE users ADD COLUMN last_login_at INTEGER".to_string(),
                "ALTER TABLE users ADD COLUMN login_count INTEGER DEFAULT 0".to_string(),
                "ALTER TABLE users ADD COLUMN is_active BOOLEAN DEFAULT 1".to_string(),
                "CREATE INDEX IF NOT EXISTS idx_users_last_login ON users(last_login_at)"
                    .to_string(),
            ],
            vec![
                "DROP INDEX IF EXISTS idx_users_last_login".to_string(),
                // 注意：SQLite 不支持 DROP COLUMN，需要重建表
            ],
        ));

        // 迁移 2: 添加会话管理表
        self.add_migration(Migration::new(
            2,
            "add_session_management",
            "添加会话管理相关表",
            vec![
                r#"CREATE TABLE IF NOT EXISTS user_sessions (
                    id TEXT PRIMARY KEY,
                    user_id INTEGER NOT NULL,
                    device_info TEXT,
                    ip_address TEXT,
                    user_agent TEXT,
                    created_at INTEGER NOT NULL,
                    last_accessed_at INTEGER NOT NULL,
                    expires_at INTEGER NOT NULL,
                    is_active BOOLEAN DEFAULT 1,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                )"#.to_string(),
                "CREATE INDEX IF NOT EXISTS idx_user_sessions_user_id ON user_sessions(user_id)".to_string(),
                "CREATE INDEX IF NOT EXISTS idx_user_sessions_expires_at ON user_sessions(expires_at)".to_string(),
            ],
            vec![
                "DROP INDEX IF EXISTS idx_user_sessions_expires_at".to_string(),
                "DROP INDEX IF NOT EXISTS idx_user_sessions_user_id".to_string(),
                "DROP TABLE IF EXISTS user_sessions".to_string(),
            ],
        ));

        // 迁移 3: 添加审计日志表
        self.add_migration(Migration::new(
            3,
            "add_audit_logs",
            "添加系统审计日志表",
            vec![
                r#"CREATE TABLE IF NOT EXISTS audit_logs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_id INTEGER,
                    action TEXT NOT NULL,
                    resource_type TEXT NOT NULL,
                    resource_id TEXT,
                    old_values TEXT,
                    new_values TEXT,
                    ip_address TEXT,
                    user_agent TEXT,
                    created_at INTEGER NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
                )"#
                .to_string(),
                "CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id)"
                    .to_string(),
                "CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action)"
                    .to_string(),
                "CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at)"
                    .to_string(),
            ],
            vec![
                "DROP INDEX IF EXISTS idx_audit_logs_created_at".to_string(),
                "DROP INDEX IF EXISTS idx_audit_logs_action".to_string(),
                "DROP INDEX IF EXISTS idx_audit_logs_user_id".to_string(),
                "DROP TABLE IF EXISTS audit_logs".to_string(),
            ],
        ));

        // 迁移 4: 优化现有索引
        self.add_migration(Migration::new(
            4,
            "optimize_indexes",
            "优化现有表的索引",
            vec![
                "CREATE INDEX IF NOT EXISTS idx_users_email_verified ON users(email_verified) WHERE email_verified = 1".to_string(),
                "CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at)".to_string(),
                "CREATE INDEX IF NOT EXISTS idx_assets_updated_at ON assets(updated_at)".to_string(),
                "CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(transaction_date)".to_string(),
            ],
            vec![
                "DROP INDEX IF EXISTS idx_transactions_date".to_string(),
                "DROP INDEX IF EXISTS idx_assets_updated_at".to_string(),
                "DROP INDEX IF EXISTS idx_users_created_at".to_string(),
                "DROP INDEX IF EXISTS idx_users_email_verified".to_string(),
            ],
        ));

        // 迁移 5: 添加系统配置表
        self.add_migration(Migration::new(
            5,
            "add_system_config",
            "添加系统配置管理表",
            vec![
                r#"CREATE TABLE IF NOT EXISTS system_config (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL,
                    description TEXT,
                    config_type TEXT NOT NULL DEFAULT 'string',
                    is_encrypted BOOLEAN DEFAULT 0,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )"#.to_string(),
                "CREATE INDEX IF NOT EXISTS idx_system_config_type ON system_config(config_type)".to_string(),
                // 插入默认配置
                r#"INSERT OR IGNORE INTO system_config (key, value, description, config_type, created_at, updated_at)
                   VALUES 
                   ('app_name', 'WolfQuant', '应用程序名称', 'string', strftime('%s', 'now'), strftime('%s', 'now')),
                   ('app_version', '1.0.0', '应用程序版本', 'string', strftime('%s', 'now'), strftime('%s', 'now')),
                   ('maintenance_mode', 'false', '维护模式', 'boolean', strftime('%s', 'now'), strftime('%s', 'now'))"#.to_string(),
            ],
            vec![
                "DROP INDEX IF EXISTS idx_system_config_type".to_string(),
                "DROP TABLE IF EXISTS system_config".to_string(),
            ],
        ));
    }

    /// 添加迁移
    fn add_migration(&mut self, migration: Migration) {
        self.migrations.insert(migration.version, migration);
    }

    /// 获取所有迁移，按版本排序
    pub fn get_migrations(&self) -> Vec<&Migration> {
        let mut migrations: Vec<&Migration> = self.migrations.values().collect();
        migrations.sort_by_key(|m| m.version);
        migrations
    }

    /// 获取待执行的迁移
    pub fn get_pending_migrations(&self, current_version: u32) -> Vec<&Migration> {
        self.get_migrations()
            .into_iter()
            .filter(|m| m.version > current_version)
            .collect()
    }

    /// 执行迁移
    pub fn execute_migration(
        &self,
        conn: &Connection,
        migration: &Migration,
    ) -> Result<u64, AuthError> {
        let start_time = std::time::Instant::now();

        info!("Executing migration: {}", migration);

        for (i, sql) in migration.up_sql.iter().enumerate() {
            debug!(
                "Executing migration {} step {}: {}",
                migration.version,
                i + 1,
                sql
            );

            conn.execute_batch(sql).map_err(|e| {
                error!(
                    "Failed to execute migration {} step {}: {}",
                    migration.version,
                    i + 1,
                    e
                );
                AuthError::DatabaseError(format!(
                    "迁移 {} 第 {} 步执行失败: {}",
                    migration.version,
                    i + 1,
                    e
                ))
            })?;
        }

        let execution_time = start_time.elapsed().as_millis() as u64;

        // 记录迁移历史
        self.record_migration(conn, migration, execution_time)?;

        info!(
            "Migration {} completed in {}ms",
            migration.version, execution_time
        );
        Ok(execution_time)
    }

    /// 回滚迁移
    pub fn rollback_migration(
        &self,
        conn: &Connection,
        migration: &Migration,
    ) -> Result<(), AuthError> {
        info!("Rolling back migration: {}", migration);

        for (i, sql) in migration.down_sql.iter().enumerate() {
            debug!(
                "Executing rollback {} step {}: {}",
                migration.version,
                i + 1,
                sql
            );

            conn.execute_batch(sql).map_err(|e| {
                error!(
                    "Failed to rollback migration {} step {}: {}",
                    migration.version,
                    i + 1,
                    e
                );
                AuthError::DatabaseError(format!(
                    "迁移 {} 回滚第 {} 步失败: {}",
                    migration.version,
                    i + 1,
                    e
                ))
            })?;
        }

        // 删除迁移记录
        self.remove_migration_record(conn, migration.version)?;

        info!("Migration {} rolled back successfully", migration.version);
        Ok(())
    }

    /// 记录迁移历史
    fn record_migration(
        &self,
        conn: &Connection,
        migration: &Migration,
        execution_time_ms: u64,
    ) -> Result<(), AuthError> {
        let checksum = self.calculate_migration_checksum(migration);
        let now = Utc::now().timestamp();

        conn.execute(
            "INSERT INTO migration_history (version, name, applied_at, execution_time_ms, checksum) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![migration.version, migration.name, now, execution_time_ms, checksum],
        ).map_err(|e| {
            error!("Failed to record migration history: {}", e);
            AuthError::DatabaseError(format!("记录迁移历史失败: {}", e))
        })?;

        Ok(())
    }

    /// 删除迁移记录
    fn remove_migration_record(&self, conn: &Connection, version: u32) -> Result<(), AuthError> {
        conn.execute(
            "DELETE FROM migration_history WHERE version = ?1",
            params![version],
        )
        .map_err(|e| {
            error!("Failed to remove migration record: {}", e);
            AuthError::DatabaseError(format!("删除迁移记录失败: {}", e))
        })?;

        Ok(())
    }

    /// 计算迁移校验和
    fn calculate_migration_checksum(&self, migration: &Migration) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        migration.up_sql.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// 验证迁移完整性
    pub fn verify_migration_integrity(&self, conn: &Connection) -> Result<Vec<String>, AuthError> {
        let mut issues = Vec::new();

        // 获取已应用的迁移
        let applied_migrations = self.get_applied_migrations(conn)?;

        for record in applied_migrations {
            if let Some(migration) = self.migrations.get(&record.version) {
                let expected_checksum = self.calculate_migration_checksum(migration);
                if record.checksum != expected_checksum {
                    issues.push(format!(
                        "Migration {} checksum mismatch: expected {}, got {}",
                        record.version, expected_checksum, record.checksum
                    ));
                }
            } else {
                issues.push(format!(
                    "Unknown migration {} found in history",
                    record.version
                ));
            }
        }

        Ok(issues)
    }

    /// 获取已应用的迁移
    fn get_applied_migrations(&self, conn: &Connection) -> Result<Vec<MigrationRecord>, AuthError> {
        let mut stmt = conn.prepare(
            "SELECT version, name, applied_at, execution_time_ms, checksum 
             FROM migration_history ORDER BY version",
        )?;

        let records: Result<Vec<MigrationRecord>, rusqlite::Error> = stmt
            .query_map([], |row| {
                Ok(MigrationRecord {
                    version: row.get(0)?,
                    name: row.get(1)?,
                    applied_at: DateTime::from_timestamp(row.get::<_, i64>(2)?, 0)
                        .unwrap_or_else(Utc::now),
                    execution_time_ms: row.get(3)?,
                    checksum: row.get(4)?,
                })
            })?
            .collect();

        records.map_err(|e| {
            error!("Failed to get applied migrations: {}", e);
            AuthError::DatabaseError(format!("获取已应用迁移失败: {}", e))
        })
    }
}

/// 获取当前数据库版本
pub fn get_database_version(conn: &Connection) -> Result<u32, AuthError> {
    // 检查版本表是否存在
    let table_exists: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='db_version'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    if table_exists == 0 {
        debug!("Version table does not exist, assuming version 0");
        return Ok(0);
    }

    // 获取版本号
    match conn.query_row(
        "SELECT version FROM db_version ORDER BY version DESC LIMIT 1",
        [],
        |row| row.get::<_, u32>(0),
    ) {
        Ok(version) => {
            debug!("Current database version: {}", version);
            Ok(version)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            debug!("No version record found, assuming version 0");
            Ok(0)
        }
        Err(e) => {
            error!("Failed to get database version: {}", e);
            Err(AuthError::DatabaseError(format!(
                "获取数据库版本失败: {}",
                e
            )))
        }
    }
}

/// 设置数据库版本
pub fn set_database_version(conn: &Connection, version: u32) -> Result<(), AuthError> {
    let now = Utc::now().timestamp();

    conn.execute(
        "INSERT OR REPLACE INTO db_version (version, updated_at, description) VALUES (?1, ?2, ?3)",
        params![version, now, format!("Updated to version {}", version)],
    )
    .map_err(|e| {
        error!("Failed to set database version to {}: {}", version, e);
        AuthError::DatabaseError(format!("设置数据库版本失败: {}", e))
    })?;

    info!("Database version set to {}", version);
    Ok(())
}

/// 创建迁移历史表
pub fn create_migration_history_table(conn: &Connection) -> Result<(), AuthError> {
    conn.execute_batch(
        r#"CREATE TABLE IF NOT EXISTS migration_history (
            version INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at INTEGER NOT NULL,
            execution_time_ms INTEGER NOT NULL,
            checksum TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_migration_history_applied_at ON migration_history(applied_at);"#
    ).map_err(|e| {
        error!("Failed to create migration history table: {}", e);
        AuthError::DatabaseError(format!("创建迁移历史表失败: {}", e))
    })?;

    Ok(())
}

/// 数据库迁移主函数
pub fn migrate_database(conn: &mut Connection, current_version: u32) -> Result<(), AuthError> {
    let config = Config::get();
    let target_version = config.database.version;

    info!(
        "Starting database migration from version {} to {}",
        current_version, target_version
    );

    if current_version >= target_version {
        info!("Database is already at target version {}", target_version);
        return Ok(());
    }

    // 创建迁移历史表
    create_migration_history_table(conn)?;

    // 创建迁移管理器
    let migration_manager = MigrationManager::new();

    // 验证迁移完整性
    let integrity_issues = migration_manager.verify_migration_integrity(conn)?;
    if !integrity_issues.is_empty() {
        warn!("Migration integrity issues found:");
        for issue in &integrity_issues {
            warn!("  - {}", issue);
        }
    }

    // 获取待执行的迁移
    let pending_migrations = migration_manager.get_pending_migrations(current_version);

    if pending_migrations.is_empty() {
        info!("No pending migrations found");
        return Ok(());
    }

    info!("Found {} pending migrations", pending_migrations.len());

    // 在事务中执行所有迁移
    let tx = conn.transaction()?;

    let mut total_execution_time = 0u64;

    for migration in pending_migrations {
        if migration.version > target_version {
            debug!(
                "Skipping migration {} (beyond target version {})",
                migration.version, target_version
            );
            continue;
        }

        let execution_time = migration_manager.execute_migration(&tx, migration)?;
        total_execution_time += execution_time;
    }

    // 更新数据库版本
    set_database_version(&tx, target_version)?;

    // 提交事务
    tx.commit().map_err(|e| {
        error!("Failed to commit migration transaction: {}", e);
        AuthError::DatabaseError(format!("提交迁移事务失败: {}", e))
    })?;

    info!(
        "Database migration completed successfully in {}ms. Version: {} -> {}",
        total_execution_time, current_version, target_version
    );

    Ok(())
}

/// 回滚数据库到指定版本
pub fn rollback_database(conn: &mut Connection, target_version: u32) -> Result<(), AuthError> {
    let current_version = get_database_version(conn)?;

    if target_version >= current_version {
        return Err(AuthError::DatabaseError(
            "目标版本必须小于当前版本".to_string(),
        ));
    }

    info!(
        "Rolling back database from version {} to {}",
        current_version, target_version
    );

    let migration_manager = MigrationManager::new();
    let tx = conn.transaction()?;

    // 获取需要回滚的迁移（按版本倒序）
    let mut rollback_migrations: Vec<&Migration> = migration_manager
        .get_migrations()
        .into_iter()
        .filter(|m| m.version > target_version && m.version <= current_version)
        .collect();

    rollback_migrations.sort_by(|a, b| b.version.cmp(&a.version));

    for migration in rollback_migrations {
        migration_manager.rollback_migration(&tx, migration)?;
    }

    // 更新数据库版本
    set_database_version(&tx, target_version)?;

    tx.commit().map_err(|e| {
        error!("Failed to commit rollback transaction: {}", e);
        AuthError::DatabaseError(format!("提交回滚事务失败: {}", e))
    })?;

    info!(
        "Database rollback completed successfully. Version: {} -> {}",
        current_version, target_version
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migration_manager_creation() {
        let manager = MigrationManager::new();
        assert!(!manager.migrations.is_empty());
    }

    #[test]
    fn test_get_database_version_no_table() {
        let conn = Connection::open_in_memory().unwrap();
        let version = get_database_version(&conn).unwrap();
        assert_eq!(version, 0);
    }

    #[test]
    fn test_set_and_get_database_version() {
        let conn = Connection::open_in_memory().unwrap();

        // 创建版本表
        conn.execute_batch(
            "CREATE TABLE db_version (
                version INTEGER PRIMARY KEY NOT NULL,
                updated_at INTEGER NOT NULL,
                description TEXT
            )",
        )
        .unwrap();

        set_database_version(&conn, 5).unwrap();
        let version = get_database_version(&conn).unwrap();
        assert_eq!(version, 5);
    }

    #[test]
    fn test_migration_checksum() {
        let migration = Migration::new(
            1,
            "test",
            "test migration",
            vec!["CREATE TABLE test (id INTEGER)".to_string()],
            vec!["DROP TABLE test".to_string()],
        );

        let manager = MigrationManager::new();
        let checksum1 = manager.calculate_migration_checksum(&migration);
        let checksum2 = manager.calculate_migration_checksum(&migration);

        assert_eq!(checksum1, checksum2);
    }
}
