use std::collections::HashMap;

/// 获取交易相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();
    
    // 交易记录表
    schemas.insert(
        "transactions".to_string(),
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
        )".to_string(),
    );
    
    // 定投计划表
    schemas.insert(
        "investment_plans".to_string(),
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
        )".to_string(),
    );
    
    // 投资策略表
    schemas.insert(
        "investment_strategies".to_string(),
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
        )".to_string(),
    );
    
    // 策略应用表
    schemas.insert(
        "strategy_applications".to_string(),
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
        )".to_string(),
    );
    
    // 交易提醒表
    schemas.insert(
        "trade_alerts".to_string(),
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
        )".to_string(),
    );
    
    schemas
}
