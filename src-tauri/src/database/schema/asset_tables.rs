use std::collections::HashMap;

/// 资产相关表名常量
pub mod tables {
    pub const ASSET_TYPES: &str = "asset_types";
    pub const USER_GROUPS: &str = "user_groups";
    pub const ASSETS: &str = "assets";
    pub const PRICE_HISTORY: &str = "price_history";
    pub const ASSET_TAGS: &str = "asset_tags";
    pub const ASSET_TAG_RELATIONS: &str = "asset_tag_relations";
    pub const ASSET_WATCHLISTS: &str = "asset_watchlists";
    pub const WATCHLIST_ITEMS: &str = "watchlist_items";
}

/// 索引名常量
pub mod indexes {
    pub const IDX_USER_GROUPS_USER: &str = "idx_user_groups_user";
    pub const IDX_USER_GROUPS_TYPE: &str = "idx_user_groups_type";
    pub const IDX_ASSETS_USER: &str = "idx_assets_user";
    pub const IDX_ASSETS_GROUP: &str = "idx_assets_group";
    pub const IDX_ASSETS_TYPE: &str = "idx_assets_type";
    pub const IDX_ASSETS_CODE: &str = "idx_assets_code";
    pub const IDX_PRICE_HISTORY_ASSET: &str = "idx_price_history_asset";
    pub const IDX_PRICE_HISTORY_DATE: &str = "idx_price_history_date";
    pub const IDX_ASSET_TAGS_NAME: &str = "idx_asset_tags_name";
    pub const IDX_ASSET_TAG_RELATIONS_ASSET: &str = "idx_asset_tag_relations_asset";
    pub const IDX_ASSET_TAG_RELATIONS_TAG: &str = "idx_asset_tag_relations_tag";
    pub const IDX_WATCHLISTS_USER: &str = "idx_watchlists_user";
    pub const IDX_WATCHLIST_ITEMS_WATCHLIST: &str = "idx_watchlist_items_watchlist";
}

/// 获取资产相关表的结构定义
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
    // 资产类型表 - 存储不同类型的资产
    schemas.insert(
        tables::ASSET_TYPES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS asset_types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,       -- 资产类型名称：'stock', 'crypto', 'forex', 'fund', 'bond', etc.
            code TEXT UNIQUE NOT NULL,       -- 资产类型代码：'STK', 'CRY', 'FRX', 'FND', 'BND', etc.
            description TEXT,                -- 资产类型描述
            icon TEXT,                       -- 图标路径或名称
            is_enabled BOOLEAN NOT NULL DEFAULT 1, -- 是否启用
            display_order INTEGER NOT NULL DEFAULT 0, -- 显示顺序
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL      -- 更新时间
        )"#.to_string(),
    );
    
    // 用户分组表 - 用户自定义的资产分组
    schemas.insert(
        tables::USER_GROUPS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS user_groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 用户ID
            name TEXT NOT NULL,              -- 分组名称
            asset_type_id INTEGER NOT NULL,  -- 资产类型ID
            description TEXT,                -- 分组描述
            color TEXT,                      -- 分组颜色
            icon TEXT,                       -- 分组图标
            display_order INTEGER NOT NULL DEFAULT 0, -- 显示顺序
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_type_id) REFERENCES asset_types (id) ON DELETE CASCADE,
            UNIQUE (user_id, name, asset_type_id)
        )"#.to_string(),
    );
    
    // 资产表 - 存储用户的资产
    schemas.insert(
        tables::ASSETS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS assets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 用户ID
            group_id INTEGER,                -- 分组ID（可选）
            asset_type_id INTEGER NOT NULL,  -- 资产类型ID
            code TEXT NOT NULL,              -- 资产代码
            name TEXT NOT NULL,              -- 资产名称
            description TEXT,                -- 资产描述
            currency TEXT NOT NULL DEFAULT 'CNY', -- 货币单位
            current_price REAL,              -- 当前价格
            position_amount REAL DEFAULT 0,  -- 持仓数量
            position_cost REAL DEFAULT 0,    -- 持仓成本
            target_price REAL,               -- 目标价格
            stop_loss_price REAL,            -- 止损价格
            risk_level INTEGER,              -- 风险等级（1-5）
            is_favorite BOOLEAN NOT NULL DEFAULT 0, -- 是否收藏
            notes TEXT,                      -- 备注
            last_updated INTEGER,            -- 价格最后更新时间
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (group_id) REFERENCES user_groups (id) ON DELETE SET NULL,
            FOREIGN KEY (asset_type_id) REFERENCES asset_types (id) ON DELETE CASCADE,
            UNIQUE (user_id, asset_type_id, code)
        )"#.to_string(),
    );
    
    // 价格历史表 - 存储资产的历史价格
    schemas.insert(
        tables::PRICE_HISTORY.to_string(),
        r#"CREATE TABLE IF NOT EXISTS price_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            asset_id INTEGER NOT NULL,       -- 资产ID
            date INTEGER NOT NULL,           -- 日期（Unix时间戳）
            open_price REAL,                 -- 开盘价
            close_price REAL NOT NULL,       -- 收盘价
            high_price REAL,                 -- 最高价
            low_price REAL,                  -- 最低价
            volume REAL,                     -- 成交量
            turnover REAL,                   -- 成交额
            change_amount REAL,              -- 涨跌额
            change_percent REAL,             -- 涨跌幅
            source TEXT,                     -- 数据来源
            created_at INTEGER NOT NULL,     -- 创建时间
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            UNIQUE (asset_id, date)
        )"#.to_string(),
    );
    
    // 资产标签表 - 用于给资产添加标签
    schemas.insert(
        tables::ASSET_TAGS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS asset_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 用户ID
            name TEXT NOT NULL,              -- 标签名称
            color TEXT,                      -- 标签颜色
            description TEXT,                -- 标签描述
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            UNIQUE (user_id, name)
        )"#.to_string(),
    );
    
    // 资产-标签关联表 - 多对多关系
    schemas.insert(
        tables::ASSET_TAG_RELATIONS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS asset_tag_relations (
            asset_id INTEGER NOT NULL,       -- 资产ID
            tag_id INTEGER NOT NULL,         -- 标签ID
            created_at INTEGER NOT NULL,     -- 创建时间
            PRIMARY KEY (asset_id, tag_id),
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES asset_tags (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 资产观察列表表 - 用户的资产观察列表
    schemas.insert(
        tables::ASSET_WATCHLISTS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS asset_watchlists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 用户ID
            name TEXT NOT NULL,              -- 观察列表名称
            description TEXT,                -- 观察列表描述
            is_default BOOLEAN NOT NULL DEFAULT 0, -- 是否默认观察列表
            display_order INTEGER NOT NULL DEFAULT 0, -- 显示顺序
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            UNIQUE (user_id, name)
        )"#.to_string(),
    );
    
    // 观察列表项表 - 观察列表中的资产
    schemas.insert(
        tables::WATCHLIST_ITEMS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS watchlist_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            watchlist_id INTEGER NOT NULL,   -- 观察列表ID
            asset_id INTEGER NOT NULL,       -- 资产ID
            display_order INTEGER NOT NULL DEFAULT 0, -- 显示顺序
            notes TEXT,                      -- 备注
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (watchlist_id) REFERENCES asset_watchlists (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            UNIQUE (watchlist_id, asset_id)
        )"#.to_string(),
    );
}

/// 添加索引定义
fn add_index_schemas(schemas: &mut HashMap<String, String>) {
    // 用户分组表索引
    schemas.insert(
        indexes::IDX_USER_GROUPS_USER.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON user_groups(user_id, display_order)", 
                indexes::IDX_USER_GROUPS_USER),
    );
    
    schemas.insert(
        indexes::IDX_USER_GROUPS_TYPE.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON user_groups(asset_type_id, user_id)", 
                indexes::IDX_USER_GROUPS_TYPE),
    );
    
    // 资产表索引
    schemas.insert(
        indexes::IDX_ASSETS_USER.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON assets(user_id, updated_at DESC)", 
                indexes::IDX_ASSETS_USER),
    );
    
    schemas.insert(
        indexes::IDX_ASSETS_GROUP.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON assets(group_id)", 
                indexes::IDX_ASSETS_GROUP),
    );
    
    schemas.insert(
        indexes::IDX_ASSETS_TYPE.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON assets(asset_type_id, user_id)", 
                indexes::IDX_ASSETS_TYPE),
    );
    
    schemas.insert(
        indexes::IDX_ASSETS_CODE.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON assets(code, asset_type_id)", 
                indexes::IDX_ASSETS_CODE),
    );
    
    // 价格历史表索引
    schemas.insert(
        indexes::IDX_PRICE_HISTORY_ASSET.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON price_history(asset_id, date DESC)", 
                indexes::IDX_PRICE_HISTORY_ASSET),
    );
    
    schemas.insert(
        indexes::IDX_PRICE_HISTORY_DATE.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON price_history(date)", 
                indexes::IDX_PRICE_HISTORY_DATE),
    );
    
    // 资产标签表索引
    schemas.insert(
        indexes::IDX_ASSET_TAGS_NAME.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON asset_tags(user_id, name)", 
                indexes::IDX_ASSET_TAGS_NAME),
    );
    
    // 资产-标签关联表索引
    schemas.insert(
        indexes::IDX_ASSET_TAG_RELATIONS_ASSET.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON asset_tag_relations(asset_id)", 
                indexes::IDX_ASSET_TAG_RELATIONS_ASSET),
    );
    
    schemas.insert(
        indexes::IDX_ASSET_TAG_RELATIONS_TAG.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON asset_tag_relations(tag_id)", 
                indexes::IDX_ASSET_TAG_RELATIONS_TAG),
    );
    
    // 资产观察列表表索引
    schemas.insert(
        indexes::IDX_WATCHLISTS_USER.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON asset_watchlists(user_id, display_order)", 
                indexes::IDX_WATCHLISTS_USER),
    );
    
    // 观察列表项表索引
    schemas.insert(
        indexes::IDX_WATCHLIST_ITEMS_WATCHLIST.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON watchlist_items(watchlist_id, display_order)", 
                indexes::IDX_WATCHLIST_ITEMS_WATCHLIST),
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
    
    // 添加默认资产类型
    sql.push_str(r#"-- 添加默认资产类型
INSERT OR IGNORE INTO asset_types (
    name, code, description, icon, is_enabled, display_order, created_at, updated_at
)
VALUES 
    ('股票', 'STK', '股票资产', 'stock-icon', 1, 1, strftime('%s', 'now'), strftime('%s', 'now')),
    ('加密货币', 'CRY', '加密货币资产', 'crypto-icon', 1, 2, strftime('%s', 'now'), strftime('%s', 'now')),
    ('基金', 'FND', '基金资产', 'fund-icon', 1, 3, strftime('%s', 'now'), strftime('%s', 'now')),
    ('外汇', 'FRX', '外汇资产', 'forex-icon', 1, 4, strftime('%s', 'now'), strftime('%s', 'now')),
    ('债券', 'BND', '债券资产', 'bond-icon', 1, 5, strftime('%s', 'now'), strftime('%s', 'now')),
    ('期货', 'FUT', '期货资产', 'futures-icon', 1, 6, strftime('%s', 'now'), strftime('%s', 'now')),
    ('期权', 'OPT', '期权资产', 'options-icon', 1, 7, strftime('%s', 'now'), strftime('%s', 'now')),
    ('其他', 'OTH', '其他资产', 'other-icon', 1, 8, strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 提交事务
    sql.push_str("COMMIT;");
    
    sql
}

/// 获取示例数据 SQL 脚本（用于开发和测试）
pub fn get_sample_data_sql() -> String {
    let mut sql = String::new();
    
    sql.push_str("BEGIN TRANSACTION;\n\n");
    
    // 添加示例用户分组
    sql.push_str(r#"-- 添加示例用户分组（假设用户ID为1）
INSERT OR IGNORE INTO user_groups (
    user_id, name, asset_type_id, description, color, icon, display_order, created_at, updated_at
)
VALUES 
    (1, '长期持有', 1, '长期持有的股票', '#4CAF50', 'trending-up', 1, strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '短期交易', 1, '短期交易的股票', '#2196F3', 'activity', 2, strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '价值投资', 1, '价值投资的股票', '#9C27B0', 'dollar-sign', 3, strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '加密货币', 2, '加密货币投资', '#FF9800', 'bitcoin', 1, strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例资产
    sql.push_str(r#"-- 添加示例资产
INSERT OR IGNORE INTO assets (
    user_id, group_id, asset_type_id, code, name, description, currency, 
    current_price, position_amount, position_cost, target_price, stop_loss_price, 
    risk_level, is_favorite, notes, last_updated, created_at, updated_at
)
VALUES 
    (1, 1, 1, 'AAPL', '苹果公司', '美国科技公司', 'USD', 
     175.43, 10, 1650.0, 200.0, 150.0, 
     2, 1, '长期看好苹果的创新能力', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
     
    (1, 1, 1, 'MSFT', '微软公司', '美国科技公司', 'USD', 
     330.11, 5, 1500.0, 350.0, 300.0, 
     2, 0, '云业务增长强劲', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
     
    (1, 2, 1, 'TSLA', '特斯拉', '电动汽车公司', 'USD', 
     240.5, 8, 1800.0, 280.0, 200.0, 
     3, 1, '波动较大，适合短期交易', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
     
    (1, 4, 2, 'BTC', '比特币', '加密货币', 'USD', 
     43500.0, 0.5, 20000.0, 50000.0, 35000.0, 
     4, 1, '作为数字黄金长期持有', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now')),
     
    (1, 4, 2, 'ETH', '以太坊', '智能合约平台', 'USD', 
     2300.0, 2.0, 4000.0, 3000.0, 1800.0, 
     4, 0, '看好以太坊2.0升级', strftime('%s', 'now'), strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例价格历史
    sql.push_str(r#"-- 添加示例价格历史
INSERT OR IGNORE INTO price_history (
    asset_id, date, open_price, close_price, high_price, low_price, 
    volume, turnover, change_amount, change_percent, source, created_at
)
VALUES 
    (1, strftime('%s', '2023-01-03'), 130.28, 125.07, 130.90, 124.17, 
     112117500, 14323456789.0, -5.21, -4.0, 'yahoo', strftime('%s', 'now')),
    (1, strftime('%s', '2023-01-04'), 126.89, 126.36, 128.66, 125.08, 
     89113600, 11234567890.0, 1.29, 1.03, 'yahoo', strftime('%s', 'now')),
    (1, strftime('%s', '2023-01-05'), 127.13, 125.02, 127.77, 124.76, 
     80829300, 10123456789.0, -1.34, -1.06, 'yahoo', strftime('%s', 'now')),
     
    (4, strftime('%s', '2023-01-03'), 16500.0, 16780.23, 16800.0, 16400.0, 
     12500.34, 209823456.67, 280.23, 1.7, 'binance', strftime('%s', 'now')),
    (4, strftime('%s', '2023-01-04'), 16780.23, 16950.45, 17000.0, 16700.0, 
     13600.45, 229876543.21, 170.22, 1.01, 'binance', strftime('%s', 'now')),
    (4, strftime('%s', '2023-01-05'), 16950.45, 16800.67, 17100.0, 16750.0, 
     14200.78, 239876543.21, -149.78, -0.88, 'binance', strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例资产标签
    sql.push_str(r#"-- 添加示例资产标签
INSERT OR IGNORE INTO asset_tags (
    user_id, name, color, description, created_at, updated_at
)
VALUES 
    (1, '科技', '#2196F3', '科技行业相关资产', strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '高成长', '#4CAF50', '高成长性资产', strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '高风险', '#F44336', '高风险资产', strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '稳健', '#607D8B', '稳健型资产', strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '加密', '#FF9800', '加密货币相关资产', strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例资产-标签关联
    sql.push_str(r#"-- 添加示例资产-标签关联
INSERT OR IGNORE INTO asset_tag_relations (
    asset_id, tag_id, created_at
)
VALUES 
    (1, 1, strftime('%s', 'now')), -- 苹果 - 科技
    (1, 2, strftime('%s', 'now')), -- 苹果 - 高成长
    (2, 1, strftime('%s', 'now')), -- 微软 - 科技
    (2, 4, strftime('%s', 'now')), -- 微软 - 稳健
    (3, 1, strftime('%s', 'now')), -- 特斯拉 - 科技
    (3, 2, strftime('%s', 'now')), -- 特斯拉 - 高成长
    (3, 3, strftime('%s', 'now')), -- 特斯拉 - 高风险
    (4, 3, strftime('%s', 'now')), -- 比特币 - 高风险
    (4, 5, strftime('%s', 'now')), -- 比特币 - 加密
    (5, 3, strftime('%s', 'now')), -- 以太坊 - 高风险
    (5, 5, strftime('%s', 'now'))  -- 以太坊 - 加密
    ;"#);
    sql.push_str("\n\n");
    
    // 添加示例观察列表
    sql.push_str(r#"-- 添加示例观察列表
INSERT OR IGNORE INTO asset_watchlists (
    user_id, name, description, is_default, display_order, created_at, updated_at
)
VALUES 
    (1, '我的关注', '默认关注列表', 1, 1, strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '潜在投资', '潜在投资机会', 0, 2, strftime('%s', 'now'), strftime('%s', 'now')),
    (1, '行业龙头', '各行业龙头企业', 0, 3, strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例观察列表项
    sql.push_str(r#"-- 添加示例观察列表项
INSERT OR IGNORE INTO watchlist_items (
    watchlist_id, asset_id, display_order, notes, created_at, updated_at
)
VALUES 
    (1, 1, 1, '重点关注', strftime('%s', 'now'), strftime('%s', 'now')),
    (1, 3, 2, '波动较大', strftime('%s', 'now'), strftime('%s', 'now')),
    (1, 4, 3, '市场情绪指标', strftime('%s', 'now'), strftime('%s', 'now')),
    (2, 2, 1, '考虑增持', strftime('%s', 'now'), strftime('%s', 'now')),
    (3, 1, 1, '科技行业龙头', strftime('%s', 'now'), strftime('%s', 'now')),
    (3, 2, 2, '软件行业龙头', strftime('%s', 'now'), strftime('%s', 'now'));"#);
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
        assert!(schemas.contains_key(&tables::ASSET_TYPES.to_string()));
        assert!(schemas.contains_key(&tables::USER_GROUPS.to_string()));
        assert!(schemas.contains_key(&tables::ASSETS.to_string()));
        assert!(schemas.contains_key(&tables::PRICE_HISTORY.to_string()));
        assert!(schemas.contains_key(&tables::ASSET_TAGS.to_string()));
        assert!(schemas.contains_key(&tables::ASSET_TAG_RELATIONS.to_string()));
        assert!(schemas.contains_key(&tables::ASSET_WATCHLISTS.to_string()));
        assert!(schemas.contains_key(&tables::WATCHLIST_ITEMS.to_string()));
        
        // 验证所有索引都已定义
        assert!(schemas.contains_key(&indexes::IDX_USER_GROUPS_USER.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_ASSETS_USER.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_PRICE_HISTORY_ASSET.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_ASSET_TAGS_NAME.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_WATCHLISTS_USER.to_string()));
    }
    
    #[test]
    fn test_get_migration_sql() {
        let sql = get_migration_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含所有表
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS asset_types"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS user_groups"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS assets"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS price_history"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS asset_tags"));
        
        // 验证 SQL 脚本包含默认资产类型
        assert!(sql.contains("INSERT OR IGNORE INTO asset_types"));
        assert!(sql.contains("'股票'"));
        assert!(sql.contains("'加密货币'"));
    }
    
    #[test]
    fn test_get_sample_data_sql() {
        let sql = get_sample_data_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含示例数据
        assert!(sql.contains("INSERT OR IGNORE INTO user_groups"));
        assert!(sql.contains("INSERT OR IGNORE INTO assets"));
        assert!(sql.contains("INSERT OR IGNORE INTO price_history"));
        assert!(sql.contains("INSERT OR IGNORE INTO asset_tags"));
        
        // 验证特定数据存在
        assert!(sql.contains("'AAPL'"));
        assert!(sql.contains("'BTC'"));
        assert!(sql.contains("'科技'"));
        assert!(sql.contains("'我的关注'"));
    }
}