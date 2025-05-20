use std::collections::HashMap;

/// 获取导入任务相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();
    
    // 导入任务表
    schemas.insert(
        "import_tasks".to_string(),
        "CREATE TABLE IF NOT EXISTS import_tasks (
            id TEXT PRIMARY KEY,
            asset_type TEXT NOT NULL,
            source TEXT NOT NULL,
            symbol TEXT NOT NULL,
            start_time INTEGER NOT NULL,
            end_time INTEGER NOT NULL,
            interval TEXT NOT NULL,
            status TEXT NOT NULL,
            progress REAL NOT NULL DEFAULT 0,
            error TEXT,
            total_candles INTEGER,
            imported_candles INTEGER,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            completed_at INTEGER
        )".to_string(),
    );
    
    // 添加索引以提高查询性能
    schemas.insert(
        "idx_import_tasks_status".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_import_tasks_status 
         ON import_tasks(status, created_at)".to_string(),
    );
    
    schemas
}
