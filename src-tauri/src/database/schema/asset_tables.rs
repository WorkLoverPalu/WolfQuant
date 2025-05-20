use std::collections::HashMap;

/// 获取资产相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();
    
    // 资产类型表
    schemas.insert(
        "asset_types".to_string(),
        "CREATE TABLE IF NOT EXISTS asset_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT
        )".to_string(),
    );
    
    // 用户分组表
    schemas.insert(
        "user_groups".to_string(),
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
        )".to_string(),
    );
    
    // 资产表
    schemas.insert(
        "assets".to_string(),
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
        )".to_string(),
    );
    
    // 价格历史表
    schemas.insert(
        "price_history".to_string(),
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
        )".to_string(),
    );
    
    schemas
}
