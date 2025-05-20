use crate::config::Config;
use crate::error::auth::AuthError;
use rusqlite::Connection;
use log::error;

/// 获取当前数据库版本
pub fn get_database_version(conn: &Connection) -> Result<u32, AuthError> {
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
pub fn set_database_version(conn: &Connection, version: u32) -> Result<(), AuthError> {
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

/// 数据库迁移
pub fn migrate_database(conn: &mut Connection, current_version: u32) -> Result<(), AuthError> {
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
