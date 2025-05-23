use std::collections::HashMap;

/// 交易相关表名常量
pub mod tables {
    pub const TRANSACTIONS: &str = "transactions";
    pub const INVESTMENT_PLANS: &str = "investment_plans";
    pub const INVESTMENT_STRATEGIES: &str = "investment_strategies";
    pub const STRATEGY_APPLICATIONS: &str = "strategy_applications";
    pub const TRADE_ALERTS: &str = "trade_alerts";
}

/// 索引名常量
pub mod indexes {
    pub const IDX_TRANSACTIONS_USER_ID: &str = "idx_transactions_user_id";
    pub const IDX_TRANSACTIONS_ASSET_ID: &str = "idx_transactions_asset_id";
    pub const IDX_TRANSACTIONS_DATE: &str = "idx_transactions_date";
    pub const IDX_INVESTMENT_PLANS_USER_ID: &str = "idx_investment_plans_user_id";
    pub const IDX_INVESTMENT_PLANS_NEXT_EXECUTION: &str = "idx_investment_plans_next_execution";
    pub const IDX_STRATEGIES_USER_ID: &str = "idx_strategies_user_id";
    pub const IDX_STRATEGY_APPLICATIONS_USER_ID: &str = "idx_strategy_applications_user_id";
    pub const IDX_STRATEGY_APPLICATIONS_ASSET_ID: &str = "idx_strategy_applications_asset_id";
    pub const IDX_TRADE_ALERTS_USER_ID: &str = "idx_trade_alerts_user_id";
    pub const IDX_TRADE_ALERTS_IS_READ: &str = "idx_trade_alerts_is_read";
}

/// 获取交易相关表的结构定义
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
    // 交易记录表 - 存储用户的所有交易记录
    schemas.insert(
        tables::TRANSACTIONS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            transaction_type TEXT NOT NULL,  -- 'buy', 'sell', 'dividend', 'split', 'fee', etc.
            amount REAL NOT NULL,            -- 交易数量
            price REAL NOT NULL,             -- 交易价格
            total_cost REAL NOT NULL,        -- 总成本（包括手续费）
            transaction_date INTEGER NOT NULL, -- 交易日期（Unix 时间戳）
            notes TEXT,                      -- 交易备注
            created_at INTEGER NOT NULL,     -- 记录创建时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE
        )"#
        .to_string(),
    );

    // 定投计划表 - 存储用户的定期投资计划
    schemas.insert(
        tables::INVESTMENT_PLANS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS investment_plans (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            name TEXT NOT NULL,              -- 计划名称
            frequency TEXT NOT NULL,         -- 'daily', 'weekly', 'monthly', 'quarterly', 'yearly'
            day_of_week INTEGER,             -- 每周几执行（1-7，周一到周日）
            day_of_month INTEGER,            -- 每月几号执行（1-31）
            amount REAL NOT NULL,            -- 每次投资金额
            is_active BOOLEAN NOT NULL DEFAULT 1, -- 计划是否激活
            last_executed INTEGER,           -- 上次执行时间
            next_execution INTEGER,          -- 下次执行时间
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE
        )"#
        .to_string(),
    );

    // 投资策略表 - 存储用户定义的投资策略
    schemas.insert(
        tables::INVESTMENT_STRATEGIES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS investment_strategies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,              -- 策略名称
            description TEXT,                -- 策略描述
            strategy_type TEXT NOT NULL,     -- 'value_averaging', 'dollar_cost_averaging', 'custom', etc.
            parameters TEXT NOT NULL,        -- JSON 格式的策略参数
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )"#.to_string(),
    );

    // 策略应用表 - 将策略应用到特定资产
    schemas.insert(
        tables::STRATEGY_APPLICATIONS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS strategy_applications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            strategy_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT 1, -- 策略应用是否激活
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES investment_strategies (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            UNIQUE (strategy_id, asset_id)   -- 一个策略只能应用到一个资产一次
        )"#
        .to_string(),
    );

    // 交易提醒表 - 存储系统生成的交易提醒
    schemas.insert(
        tables::TRADE_ALERTS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS trade_alerts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            asset_id INTEGER NOT NULL,
            strategy_id INTEGER,             -- 可能为空，如果提醒不是由策略生成的
            alert_type TEXT NOT NULL,        -- 'buy', 'sell', 'price_target', 'strategy', etc.
            message TEXT NOT NULL,           -- 提醒消息
            is_read BOOLEAN NOT NULL DEFAULT 0, -- 提醒是否已读
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (asset_id) REFERENCES assets (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES investment_strategies (id) ON DELETE SET NULL
        )"#
        .to_string(),
    );
}

/// 添加索引定义
fn add_index_schemas(schemas: &mut HashMap<String, String>) {
    // 交易记录表索引
    schemas.insert(
        indexes::IDX_TRANSACTIONS_USER_ID.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON transactions(user_id)",
            indexes::IDX_TRANSACTIONS_USER_ID
        ),
    );
    schemas.insert(
        indexes::IDX_TRANSACTIONS_ASSET_ID.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON transactions(asset_id)",
            indexes::IDX_TRANSACTIONS_ASSET_ID
        ),
    );
    schemas.insert(
        indexes::IDX_TRANSACTIONS_DATE.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON transactions(transaction_date)",
            indexes::IDX_TRANSACTIONS_DATE
        ),
    );

    // 定投计划表索引
    schemas.insert(
        indexes::IDX_INVESTMENT_PLANS_USER_ID.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON investment_plans(user_id)",
            indexes::IDX_INVESTMENT_PLANS_USER_ID
        ),
    );
    schemas.insert(
        indexes::IDX_INVESTMENT_PLANS_NEXT_EXECUTION.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON investment_plans(next_execution) WHERE is_active = 1",
            indexes::IDX_INVESTMENT_PLANS_NEXT_EXECUTION
        ),
    );

    // 投资策略表索引
    schemas.insert(
        indexes::IDX_STRATEGIES_USER_ID.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON investment_strategies(user_id)",
            indexes::IDX_STRATEGIES_USER_ID
        ),
    );

    // 策略应用表索引
    schemas.insert(
        indexes::IDX_STRATEGY_APPLICATIONS_USER_ID.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON strategy_applications(user_id)",
            indexes::IDX_STRATEGY_APPLICATIONS_USER_ID
        ),
    );
    schemas.insert(
        indexes::IDX_STRATEGY_APPLICATIONS_ASSET_ID.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON strategy_applications(asset_id) WHERE is_active = 1",
            indexes::IDX_STRATEGY_APPLICATIONS_ASSET_ID
        ),
    );

    // 交易提醒表索引
    schemas.insert(
        indexes::IDX_TRADE_ALERTS_USER_ID.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON trade_alerts(user_id)",
            indexes::IDX_TRADE_ALERTS_USER_ID
        ),
    );
    schemas.insert(
        indexes::IDX_TRADE_ALERTS_IS_READ.to_string(),
        format!(
            "CREATE INDEX IF NOT EXISTS {} ON trade_alerts(is_read, created_at) WHERE is_read = 0",
            indexes::IDX_TRADE_ALERTS_IS_READ
        ),
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

    // 添加示例资产
    sql.push_str(
        r#"INSERT OR IGNORE INTO assets (symbol, name, asset_type, created_at, updated_at)
    VALUES 
        ('AAPL', '苹果公司', 'stock', strftime('%s', 'now'), strftime('%s', 'now')),
        ('MSFT', '微软公司', 'stock', strftime('%s', 'now'), strftime('%s', 'now')),
        ('BTC', '比特币', 'crypto', strftime('%s', 'now'), strftime('%s', 'now')),
        ('ETH', '以太坊', 'crypto', strftime('%s', 'now'), strftime('%s', 'now'));"#,
    );
    sql.push_str("\n\n");

    // 添加示例交易记录（假设用户 ID 为 1）
    sql.push_str(r#"INSERT OR IGNORE INTO transactions (user_id, asset_id, transaction_type, amount, price, total_cost, transaction_date, notes, created_at)
    VALUES 
        (1, 1, 'buy', 10, 150.0, 1500.0, strftime('%s', 'now', '-30 days'), '首次购买苹果股票', strftime('%s', 'now')),
        (1, 1, 'buy', 5, 155.0, 775.0, strftime('%s', 'now', '-15 days'), '定投苹果股票', strftime('%s', 'now')),
        (1, 2, 'buy', 20, 250.0, 5000.0, strftime('%s', 'now', '-20 days'), '首次购买微软股票', strftime('%s', 'now')),
        (1, 3, 'buy', 0.5, 30000.0, 15000.0, strftime('%s', 'now', '-10 days'), '首次购买比特币', strftime('%s', 'now'));"#);
    sql.push_str("\n\n");

    // 添加示例定投计划
    sql.push_str(r#"INSERT OR IGNORE INTO investment_plans (user_id, asset_id, name, frequency, day_of_week, amount, is_active, next_execution, created_at, updated_at)
    VALUES 
        (1, 1, '苹果股票周定投', 'weekly', 1, 500.0, 1, strftime('%s', 'now', '+7 days'), strftime('%s', 'now'), strftime('%s', 'now')),
        (1, 3, '比特币月定投', 'monthly', NULL, 1000.0, 1, strftime('%s', 'now', '+30 days'), strftime('%s', 'now'), strftime('%s', 'now'));"#);
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
        assert!(schemas.contains_key(&tables::TRANSACTIONS.to_string()));
        assert!(schemas.contains_key(&tables::INVESTMENT_PLANS.to_string()));
        assert!(schemas.contains_key(&tables::INVESTMENT_STRATEGIES.to_string()));
        assert!(schemas.contains_key(&tables::STRATEGY_APPLICATIONS.to_string()));
        assert!(schemas.contains_key(&tables::TRADE_ALERTS.to_string()));

        // 验证所有索引都已定义
        assert!(schemas.contains_key(&indexes::IDX_TRANSACTIONS_USER_ID.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_TRANSACTIONS_ASSET_ID.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_TRANSACTIONS_DATE.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_INVESTMENT_PLANS_USER_ID.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_INVESTMENT_PLANS_NEXT_EXECUTION.to_string()));
    }

    #[test]
    fn test_get_migration_sql() {
        let sql = get_migration_sql();

        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));

        // 验证 SQL 脚本包含所有表
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS transactions"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS investment_plans"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS investment_strategies"));
    }

    #[test]
    fn test_get_sample_data_sql() {
        let sql = get_sample_data_sql();

        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));

        // 验证 SQL 脚本包含示例数据
        assert!(sql.contains("INSERT OR IGNORE INTO assets"));
        assert!(sql.contains("INSERT OR IGNORE INTO transactions"));
        assert!(sql.contains("INSERT OR IGNORE INTO investment_plans"));
    }
}
