use std::collections::HashMap;

/// 导入任务相关表名常量
pub mod tables {
    pub const IMPORT_TASKS: &str = "import_tasks";
    pub const IMPORT_LOGS: &str = "import_logs";
}

/// 索引名常量
pub mod indexes {
    pub const IDX_IMPORT_TASKS_STATUS: &str = "idx_import_tasks_status";
    pub const IDX_IMPORT_TASKS_SYMBOL: &str = "idx_import_tasks_symbol";
    pub const IDX_IMPORT_TASKS_SOURCE: &str = "idx_import_tasks_source";
    pub const IDX_IMPORT_TASKS_CREATED: &str = "idx_import_tasks_created_at";
    pub const IDX_IMPORT_LOGS_TASK_ID: &str = "idx_import_logs_task_id";
}

/// 获取导入任务相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();

    // 添加表定义
    add_table_schemas(&mut schemas);
    
    // 添加索引定义
    add_index_schemas(&mut schemas);

    schemas
}

/// 添加表定义
fn add_table_schemas(schemas: &mut HashMap<String, String>) {
    // 导入任务表 - 存储数据导入任务信息
    schemas.insert(
        tables::IMPORT_TASKS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS import_tasks (
            id TEXT PRIMARY KEY,             -- 唯一任务ID（UUID）
            user_id INTEGER NOT NULL,        -- 用户ID
            asset_type TEXT NOT NULL,        -- 资产类型：'stock', 'crypto', 'forex', etc.
            source TEXT NOT NULL,            -- 数据源：'yahoo', 'binance', 'alphavantage', etc.
            symbol TEXT NOT NULL,            -- 资产代码
            start_time INTEGER NOT NULL,     -- 开始时间（Unix时间戳）
            end_time INTEGER NOT NULL,       -- 结束时间（Unix时间戳）
            interval TEXT NOT NULL,          -- 时间间隔：'1m', '5m', '15m', '1h', '1d', '1w', etc.
            status TEXT NOT NULL,            -- 任务状态：'pending', 'running', 'completed', 'failed', 'cancelled'
            progress REAL NOT NULL DEFAULT 0, -- 进度（0-100%）
            error TEXT,                      -- 错误信息（如果失败）
            total_candles INTEGER,           -- 总K线数
            imported_candles INTEGER,        -- 已导入K线数
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            completed_at INTEGER,            -- 完成时间
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 导入日志表 - 记录导入过程中的详细日志
    schemas.insert(
        tables::IMPORT_LOGS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS import_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id TEXT NOT NULL,           -- 关联的导入任务ID
            log_time INTEGER NOT NULL,       -- 日志记录时间
            log_level TEXT NOT NULL,         -- 日志级别：'info', 'warning', 'error'
            message TEXT NOT NULL,           -- 日志消息
            details TEXT,                    -- 详细信息（可选）
            FOREIGN KEY (task_id) REFERENCES import_tasks(id) ON DELETE CASCADE
        )"#.to_string(),
    );
}

/// 添加索引定义
fn add_index_schemas(schemas: &mut HashMap<String, String>) {
    // 导入任务表索引
    schemas.insert(
        indexes::IDX_IMPORT_TASKS_STATUS.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON import_tasks(status, created_at)", 
                indexes::IDX_IMPORT_TASKS_STATUS),
    );
    
    schemas.insert(
        indexes::IDX_IMPORT_TASKS_SYMBOL.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON import_tasks(symbol, asset_type)", 
                indexes::IDX_IMPORT_TASKS_SYMBOL),
    );
    
    schemas.insert(
        indexes::IDX_IMPORT_TASKS_SOURCE.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON import_tasks(source, status)", 
                indexes::IDX_IMPORT_TASKS_SOURCE),
    );
    
    schemas.insert(
        indexes::IDX_IMPORT_TASKS_CREATED.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON import_tasks(user_id, created_at DESC)", 
                indexes::IDX_IMPORT_TASKS_CREATED),
    );
    
    // 导入日志表索引
    schemas.insert(
        indexes::IDX_IMPORT_LOGS_TASK_ID.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON import_logs(task_id, log_time)", 
                indexes::IDX_IMPORT_LOGS_TASK_ID),
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

/// 获取示例数据 SQL 脚本（用于开发和测试）
pub fn get_sample_data_sql() -> String {
    let mut sql = String::new();
    
    sql.push_str("BEGIN TRANSACTION;\n\n");
    
    // 添加示例导入任务
    sql.push_str(r#"-- 添加示例导入任务
INSERT OR IGNORE INTO import_tasks (
    id, user_id, asset_type, source, symbol, start_time, end_time, 
    interval, status, progress, total_candles, imported_candles, 
    created_at, updated_at, completed_at
)
VALUES 
    ('task-001', 1, 'stock', 'yahoo', 'AAPL', 
     strftime('%s', '2022-01-01'), strftime('%s', '2023-01-01'), 
     '1d', 'completed', 100.0, 365, 365, 
     strftime('%s', 'now', '-10 days'), strftime('%s', 'now', '-9 days'), strftime('%s', 'now', '-9 days')),
     
    ('task-002', 1, 'crypto', 'binance', 'BTCUSDT', 
     strftime('%s', '2023-01-01'), strftime('%s', '2023-06-01'), 
     '1h', 'completed', 100.0, 4320, 4320, 
     strftime('%s', 'now', '-5 days'), strftime('%s', 'now', '-4 days'), strftime('%s', 'now', '-4 days')),
     
    ('task-003', 1, 'stock', 'alphavantage', 'MSFT', 
     strftime('%s', '2023-01-01'), strftime('%s', 'now'), 
     '1d', 'running', 45.0, 365, 164, 
     strftime('%s', 'now', '-1 day'), strftime('%s', 'now'), NULL),
     
    ('task-004', 1, 'forex', 'oanda', 'EUR_USD', 
     strftime('%s', '2022-01-01'), strftime('%s', '2022-03-01'), 
     '15m', 'failed', 23.5, 8640, 2030, 
     strftime('%s', 'now', '-8 days'), strftime('%s', 'now', '-8 days'), NULL);"#);
    sql.push_str("\n\n");
    
    // 添加示例导入日志
    sql.push_str(r#"-- 添加示例导入日志
INSERT OR IGNORE INTO import_logs (task_id, log_time, log_level, message, details)
VALUES 
    ('task-001', strftime('%s', 'now', '-10 days'), 'info', '开始导入苹果股票数据', NULL),
    ('task-001', strftime('%s', 'now', '-9 days'), 'info', '导入完成', '{"total_records": 365, "skipped": 0}'),
    
    ('task-002', strftime('%s', 'now', '-5 days'), 'info', '开始导入比特币数据', NULL),
    ('task-002', strftime('%s', 'now', '-5 days', '+1 hour'), 'info', '已导入1000条记录', '{"progress": 23.15}'),
    ('task-002', strftime('%s', 'now', '-4 days'), 'info', '导入完成', '{"total_records": 4320, "skipped": 5}'),
    
    ('task-003', strftime('%s', 'now', '-1 day'), 'info', '开始导入微软股票数据', NULL),
    ('task-003', strftime('%s', 'now', '-12 hours'), 'warning', '数据源响应缓慢', '{"latency": 2500}'),
    ('task-003', strftime('%s', 'now', '-6 hours'), 'info', '继续导入', '{"progress": 45.0}'),
    
    ('task-004', strftime('%s', 'now', '-8 days'), 'info', '开始导入欧元/美元汇率数据', NULL),
    ('task-004', strftime('%s', 'now', '-8 days', '+1 hour'), 'error', '数据源连接失败', 
     '{"error_code": "CONNECTION_TIMEOUT", "details": "无法连接到数据源，请检查网络或API密钥"}');"#);
    sql.push_str("\n\n");
    
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
        assert!(schemas.contains_key(&tables::IMPORT_TASKS.to_string()));
        assert!(schemas.contains_key(&tables::IMPORT_LOGS.to_string()));
        
        // 验证所有索引都已定义
        assert!(schemas.contains_key(&indexes::IDX_IMPORT_TASKS_STATUS.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_IMPORT_TASKS_SYMBOL.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_IMPORT_TASKS_SOURCE.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_IMPORT_TASKS_CREATED.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_IMPORT_LOGS_TASK_ID.to_string()));
    }
    
    #[test]
    fn test_get_migration_sql() {
        let sql = get_migration_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含所有表
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS import_tasks"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS import_logs"));
        
        // 验证 SQL 脚本包含所有索引
        assert!(sql.contains("CREATE INDEX IF NOT EXISTS idx_import_tasks_status"));
        assert!(sql.contains("CREATE INDEX IF NOT EXISTS idx_import_tasks_symbol"));
    }
    
    #[test]
    fn test_get_sample_data_sql() {
        let sql = get_sample_data_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含示例数据
        assert!(sql.contains("INSERT OR IGNORE INTO import_tasks"));
        assert!(sql.contains("INSERT OR IGNORE INTO import_logs"));
        
        // 验证特定数据存在
        assert!(sql.contains("'yahoo'"));
        assert!(sql.contains("'AAPL'"));
        assert!(sql.contains("'running'"));
        assert!(sql.contains("'completed'"));
    }
}