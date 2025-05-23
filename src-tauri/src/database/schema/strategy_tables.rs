use std::collections::HashMap;

/// 策略相关表名常量
pub mod tables {
    pub const STRATEGIES: &str = "strategies";
    pub const STRATEGY_VERSIONS: &str = "strategy_versions";
    pub const STRATEGY_TAGS: &str = "strategy_tags";
    pub const STRATEGY_TAG_RELATIONS: &str = "strategy_tag_relations";
    pub const STRATEGY_FAVORITES: &str = "strategy_favorites";
    pub const STRATEGY_RATINGS: &str = "strategy_ratings";
    pub const STRATEGY_APPLICATIONS: &str = "strategy_applications";
}

/// 索引名常量
pub mod indexes {
    pub const IDX_STRATEGIES_USER: &str = "idx_strategies_user";
    pub const IDX_STRATEGIES_PUBLIC: &str = "idx_strategies_public";
    pub const IDX_STRATEGIES_TYPE: &str = "idx_strategies_type";
    pub const IDX_STRATEGY_VERSIONS: &str = "idx_strategy_versions";
    pub const IDX_STRATEGY_FAVORITES_USER: &str = "idx_strategy_favorites_user";
    pub const IDX_STRATEGY_FAVORITES_STRATEGY: &str = "idx_strategy_favorites_strategy";
    pub const IDX_STRATEGY_RATINGS_STRATEGY: &str = "idx_strategy_ratings_strategy";
    pub const IDX_STRATEGY_APPLICATIONS_USER: &str = "idx_strategy_applications_user";
    pub const IDX_STRATEGY_APPLICATIONS_ASSET: &str = "idx_strategy_applications_asset";
    pub const IDX_STRATEGY_TAG_RELATIONS_TAG: &str = "idx_strategy_tag_relations_tag";
}

/// 获取策略相关表的结构定义
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
    // 策略表 - 存储用户创建的投资策略
    schemas.insert(
        tables::STRATEGIES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 创建者ID
            name TEXT NOT NULL,              -- 策略名称
            description TEXT,                -- 策略描述
            strategy_type TEXT NOT NULL,     -- 策略类型：'value_averaging', 'dollar_cost_averaging', 'momentum', 'custom', etc.
            parameters TEXT NOT NULL,        -- JSON格式的策略参数
            is_public BOOLEAN NOT NULL DEFAULT 0, -- 是否公开分享
            is_active BOOLEAN NOT NULL DEFAULT 1, -- 是否激活
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            UNIQUE (user_id, name)           -- 同一用户不能有重名策略
        )"#.to_string(),
    );
    
    // 策略版本表 - 用于跟踪策略的历史版本
    schemas.insert(
        tables::STRATEGY_VERSIONS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategy_versions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            strategy_id INTEGER NOT NULL,    -- 关联的策略ID
            version INTEGER NOT NULL,        -- 版本号
            parameters TEXT NOT NULL,        -- 该版本的策略参数
            description TEXT,                -- 版本说明
            created_at INTEGER NOT NULL,     -- 版本创建时间
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            UNIQUE (strategy_id, version)    -- 策略版本唯一
        )"#.to_string(),
    );
    
    // 策略标签表 - 用于给策略添加标签
    schemas.insert(
        tables::STRATEGY_TAGS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategy_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE        -- 标签名称
        )"#.to_string(),
    );
    
    // 策略-标签关联表 - 多对多关系
    schemas.insert(
        tables::STRATEGY_TAG_RELATIONS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategy_tag_relations (
            strategy_id INTEGER NOT NULL,    -- 策略ID
            tag_id INTEGER NOT NULL,         -- 标签ID
            PRIMARY KEY (strategy_id, tag_id),
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES strategy_tags (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 策略收藏表 - 用户可以收藏其他用户的公开策略
    schemas.insert(
        tables::STRATEGY_FAVORITES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategy_favorites (
            user_id INTEGER NOT NULL,        -- 收藏者ID
            strategy_id INTEGER NOT NULL,    -- 被收藏的策略ID
            created_at INTEGER NOT NULL,     -- 收藏时间
            PRIMARY KEY (user_id, strategy_id),
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 策略评分表 - 用户可以对公开策略进行评分
    schemas.insert(
        tables::STRATEGY_RATINGS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategy_ratings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 评分者ID
            strategy_id INTEGER NOT NULL,    -- 被评分的策略ID
            rating INTEGER NOT NULL CHECK(rating BETWEEN 1 AND 5), -- 评分（1-5星）
            comment TEXT,                    -- 评论内容
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            UNIQUE (user_id, strategy_id)    -- 每个用户对每个策略只能评分一次
        )"#.to_string(),
    );
    
    // 策略应用表 - 记录策略应用于哪些资产
    schemas.insert(
        tables::STRATEGY_APPLICATIONS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategy_applications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 用户ID
            strategy_id INTEGER NOT NULL,    -- 策略ID
            asset_id INTEGER NOT NULL,       -- 资产ID
            is_active BOOLEAN NOT NULL DEFAULT 1, -- 是否激活
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            UNIQUE (strategy_id, asset_id)   -- 一个策略只能应用到一个资产一次
        )"#.to_string(),
    );
}

/// 添加索引定义
fn add_index_schemas(schemas: &mut HashMap<String, String>) {
    // 策略表索引
    schemas.insert(
        indexes::IDX_STRATEGIES_USER.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategies(user_id)", 
                indexes::IDX_STRATEGIES_USER),
    );
    
    schemas.insert(
        indexes::IDX_STRATEGIES_PUBLIC.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategies(is_public, is_active)", 
                indexes::IDX_STRATEGIES_PUBLIC),
    );
    
    schemas.insert(
        indexes::IDX_STRATEGIES_TYPE.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategies(strategy_type, is_public) WHERE is_public = 1", 
                indexes::IDX_STRATEGIES_TYPE),
    );
    
    // 策略版本表索引
    schemas.insert(
        indexes::IDX_STRATEGY_VERSIONS.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategy_versions(strategy_id, version)", 
                indexes::IDX_STRATEGY_VERSIONS),
    );
    
    // 策略收藏表索引
    schemas.insert(
        indexes::IDX_STRATEGY_FAVORITES_USER.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategy_favorites(user_id)", 
                indexes::IDX_STRATEGY_FAVORITES_USER),
    );
    
    schemas.insert(
        indexes::IDX_STRATEGY_FAVORITES_STRATEGY.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategy_favorites(strategy_id)", 
                indexes::IDX_STRATEGY_FAVORITES_STRATEGY),
    );
    
    // 策略评分表索引
    schemas.insert(
        indexes::IDX_STRATEGY_RATINGS_STRATEGY.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategy_ratings(strategy_id, rating)", 
                indexes::IDX_STRATEGY_RATINGS_STRATEGY),
    );
    
    // 策略应用表索引
    schemas.insert(
        indexes::IDX_STRATEGY_APPLICATIONS_USER.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategy_applications(user_id)", 
                indexes::IDX_STRATEGY_APPLICATIONS_USER),
    );
    
    schemas.insert(
        indexes::IDX_STRATEGY_APPLICATIONS_ASSET.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategy_applications(asset_id) WHERE is_active = 1", 
                indexes::IDX_STRATEGY_APPLICATIONS_ASSET),
    );
    
    // 策略标签关系表索引
    schemas.insert(
        indexes::IDX_STRATEGY_TAG_RELATIONS_TAG.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON strategy_tag_relations(tag_id)", 
                indexes::IDX_STRATEGY_TAG_RELATIONS_TAG),
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
    
    // 添加默认标签
    sql.push_str(r#"-- 添加默认策略标签
INSERT OR IGNORE INTO strategy_tags (name) VALUES 
    ('价值投资'),
    ('成长投资'),
    ('定投策略'),
    ('技术分析'),
    ('趋势跟踪'),
    ('动量策略'),
    ('均值回归'),
    ('波动率策略'),
    ('套利策略'),
    ('对冲策略');"#);
    sql.push_str("\n\n");
    
    // 提交事务
    sql.push_str("COMMIT;");
    
    sql
}

/// 获取示例数据 SQL 脚本（用于开发和测试）
pub fn get_sample_data_sql() -> String {
    let mut sql = String::new();
    
    sql.push_str("BEGIN TRANSACTION;\n\n");
    
    // 添加示例策略
    sql.push_str(r#"-- 添加示例策略
INSERT OR IGNORE INTO strategies (user_id, name, description, strategy_type, parameters, is_public, is_active, created_at, updated_at)
VALUES 
    (1, '定期定额投资', '每月固定金额投资特定资产', 'dollar_cost_averaging', 
     '{"amount": 1000, "frequency": "monthly", "day_of_month": 15}', 
     1, 1, strftime('%s', 'now', '-60 days'), strftime('%s', 'now')),
     
    (1, '价值平均策略', '根据目标价值调整投资金额', 'value_averaging', 
     '{"target_value_increase": 500, "frequency": "monthly", "max_adjustment": 2000}', 
     1, 1, strftime('%s', 'now', '-45 days'), strftime('%s', 'now')),
     
    (1, '动量跟踪策略', '追踪价格动量进行买卖决策', 'momentum', 
     '{"lookback_period": 20, "threshold": 0.05, "position_size": 0.1}', 
     0, 1, strftime('%s', 'now', '-30 days'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加策略版本
    sql.push_str(r#"-- 添加策略版本
INSERT OR IGNORE INTO strategy_versions (strategy_id, version, parameters, description, created_at)
VALUES 
    (1, 1, '{"amount": 500, "frequency": "monthly", "day_of_month": 15}', 
     '初始版本', strftime('%s', 'now', '-60 days')),
    (1, 2, '{"amount": 1000, "frequency": "monthly", "day_of_month": 15}', 
     '增加投资金额', strftime('%s', 'now', '-30 days'));"#);
    sql.push_str("\n\n");
    
    // 添加策略标签关系
    sql.push_str(r#"-- 添加策略标签关系
INSERT OR IGNORE INTO strategy_tag_relations (strategy_id, tag_id)
VALUES 
    (1, 3), -- 定期定额投资 - 定投策略
    (2, 1), -- 价值平均策略 - 价值投资
    (2, 3), -- 价值平均策略 - 定投策略
    (3, 5), -- 动量跟踪策略 - 趋势跟踪
    (3, 6); -- 动量跟踪策略 - 动量策略"#);
    sql.push_str("\n\n");
    
    // 添加策略收藏
    sql.push_str(r#"-- 添加策略收藏（假设有用户2和用户3）
INSERT OR IGNORE INTO strategy_favorites (user_id, strategy_id, created_at)
VALUES 
    (2, 1, strftime('%s', 'now', '-20 days')),
    (3, 1, strftime('%s', 'now', '-15 days')),
    (2, 2, strftime('%s', 'now', '-10 days'));"#);
    sql.push_str("\n\n");
    
    // 添加策略评分
    sql.push_str(r#"-- 添加策略评分
INSERT OR IGNORE INTO strategy_ratings (user_id, strategy_id, rating, comment, created_at, updated_at)
VALUES 
    (2, 1, 5, '非常好用的定投策略，简单易懂', strftime('%s', 'now', '-18 days'), strftime('%s', 'now', '-18 days')),
    (3, 1, 4, '不错的入门策略', strftime('%s', 'now', '-12 days'), strftime('%s', 'now', '-12 days')),
    (2, 2, 4, '价值平均法很适合波动大的市场', strftime('%s', 'now', '-8 days'), strftime('%s', 'now', '-8 days'));"#);
    sql.push_str("\n\n");
    
    // 添加策略应用
    sql.push_str(r#"-- 添加策略应用
INSERT OR IGNORE INTO strategy_applications (user_id, strategy_id, asset_id, is_active, created_at, updated_at)
VALUES 
    (1, 1, 1, 1, strftime('%s', 'now', '-55 days'), strftime('%s', 'now')),
    (1, 2, 3, 1, strftime('%s', 'now', '-40 days'), strftime('%s', 'now')),
    (1, 3, 2, 1, strftime('%s', 'now', '-25 days'), strftime('%s', 'now'));"#);
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
        assert!(schemas.contains_key(&tables::STRATEGIES.to_string()));
        assert!(schemas.contains_key(&tables::STRATEGY_VERSIONS.to_string()));
        assert!(schemas.contains_key(&tables::STRATEGY_TAGS.to_string()));
        assert!(schemas.contains_key(&tables::STRATEGY_TAG_RELATIONS.to_string()));
        assert!(schemas.contains_key(&tables::STRATEGY_FAVORITES.to_string()));
        assert!(schemas.contains_key(&tables::STRATEGY_RATINGS.to_string()));
        assert!(schemas.contains_key(&tables::STRATEGY_APPLICATIONS.to_string()));
        
        // 验证所有索引都已定义
        assert!(schemas.contains_key(&indexes::IDX_STRATEGIES_USER.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGIES_PUBLIC.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGIES_TYPE.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGY_VERSIONS.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGY_FAVORITES_USER.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGY_FAVORITES_STRATEGY.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGY_RATINGS_STRATEGY.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGY_APPLICATIONS_USER.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGY_APPLICATIONS_ASSET.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_STRATEGY_TAG_RELATIONS_TAG.to_string()));
    }
    
    #[test]
    fn test_get_migration_sql() {
        let sql = get_migration_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含所有表
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS strategies"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS strategy_versions"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS strategy_tags"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS strategy_tag_relations"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS strategy_favorites"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS strategy_ratings"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS strategy_applications"));
        
        // 验证 SQL 脚本包含默认标签
        assert!(sql.contains("INSERT OR IGNORE INTO strategy_tags"));
        assert!(sql.contains("价值投资"));
        assert!(sql.contains("定投策略"));
    }
    
    #[test]
    fn test_get_sample_data_sql() {
        let sql = get_sample_data_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含示例数据
        assert!(sql.contains("INSERT OR IGNORE INTO strategies"));
        assert!(sql.contains("INSERT OR IGNORE INTO strategy_versions"));
        assert!(sql.contains("INSERT OR IGNORE INTO strategy_tag_relations"));
        assert!(sql.contains("INSERT OR IGNORE INTO strategy_favorites"));
        assert!(sql.contains("INSERT OR IGNORE INTO strategy_ratings"));
        assert!(sql.contains("INSERT OR IGNORE INTO strategy_applications"));
    }
}