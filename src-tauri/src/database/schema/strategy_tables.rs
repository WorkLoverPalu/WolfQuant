use std::collections::HashMap;

/// 获取策略相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();
    
    // 策略表
    schemas.insert(
        "strategies".to_string(),
        "CREATE TABLE IF NOT EXISTS strategies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            strategy_type TEXT NOT NULL,
            parameters TEXT NOT NULL,
            is_public BOOLEAN NOT NULL DEFAULT 0,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            UNIQUE (user_id, name)
        )".to_string(),
    );
    
    // 策略版本表 - 用于跟踪策略的历史版本
    schemas.insert(
        "strategy_versions".to_string(),
        "CREATE TABLE IF NOT EXISTS strategy_versions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            strategy_id INTEGER NOT NULL,
            version INTEGER NOT NULL,
            parameters TEXT NOT NULL,
            description TEXT,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            UNIQUE (strategy_id, version)
        )".to_string(),
    );
    
    // 策略标签表 - 用于给策略添加标签
    schemas.insert(
        "strategy_tags".to_string(),
        "CREATE TABLE IF NOT EXISTS strategy_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )".to_string(),
    );
    
    // 策略-标签关联表
    schemas.insert(
        "strategy_tag_relations".to_string(),
        "CREATE TABLE IF NOT EXISTS strategy_tag_relations (
            strategy_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (strategy_id, tag_id),
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES strategy_tags (id) ON DELETE CASCADE
        )".to_string(),
    );
    
    // 策略收藏表 - 用户可以收藏其他用户的公开策略
    schemas.insert(
        "strategy_favorites".to_string(),
        "CREATE TABLE IF NOT EXISTS strategy_favorites (
            user_id INTEGER NOT NULL,
            strategy_id INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            PRIMARY KEY (user_id, strategy_id),
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE
        )".to_string(),
    );
    
    // 策略评分表 - 用户可以对公开策略进行评分
    schemas.insert(
        "strategy_ratings".to_string(),
        "CREATE TABLE IF NOT EXISTS strategy_ratings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            strategy_id INTEGER NOT NULL,
            rating INTEGER NOT NULL CHECK(rating BETWEEN 1 AND 5),
            comment TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            UNIQUE (user_id, strategy_id)
        )".to_string(),
    );
    
    // 策略应用表 - 记录策略应用于哪些资产
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
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            UNIQUE (strategy_id, asset_id)
        )".to_string(),
    );
    
    // 添加索引以提高查询性能
    schemas.insert(
        "idx_strategies_user".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_strategies_user 
         ON strategies(user_id)".to_string(),
    );
    
    schemas.insert(
        "idx_strategies_public".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_strategies_public 
         ON strategies(is_public, is_active)".to_string(),
    );
    
    schemas.insert(
        "idx_strategy_versions".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_strategy_versions 
         ON strategy_versions(strategy_id, version)".to_string(),
    );
    
    schemas
}
