use std::collections::HashMap;

/// 回测相关表名常量
pub mod tables {
    pub const BACKTEST_RESULTS: &str = "backtest_results";
    pub const EQUITY_CURVES: &str = "equity_curves";
    pub const BACKTEST_TRADES: &str = "backtest_trades";
    pub const BACKTEST_POSITIONS: &str = "backtest_positions";
    pub const BACKTEST_METRICS: &str = "backtest_metrics";
    pub const BACKTEST_CONFIGS: &str = "backtest_configs";
}

/// 索引名常量
pub mod indexes {
    pub const IDX_BACKTEST_RESULTS_USER: &str = "idx_backtest_results_user";
    pub const IDX_BACKTEST_RESULTS_STRATEGY: &str = "idx_backtest_results_strategy";
    pub const IDX_BACKTEST_RESULTS_CREATED: &str = "idx_backtest_results_created";
    pub const IDX_EQUITY_CURVES_BACKTEST: &str = "idx_equity_curves_backtest";
    pub const IDX_BACKTEST_TRADES_BACKTEST: &str = "idx_backtest_trades_backtest";
    pub const IDX_BACKTEST_TRADES_SYMBOL: &str = "idx_backtest_trades_symbol";
    pub const IDX_BACKTEST_POSITIONS_BACKTEST: &str = "idx_backtest_positions_backtest";
    pub const IDX_BACKTEST_METRICS_BACKTEST: &str = "idx_backtest_metrics_backtest";
}

/// 获取回测相关表的结构定义
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
    // 回测配置表 - 存储回测的配置参数
    schemas.insert(
        tables::BACKTEST_CONFIGS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS backtest_configs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 用户ID
            name TEXT NOT NULL,              -- 配置名称
            start_date INTEGER NOT NULL,     -- 回测开始日期
            end_date INTEGER NOT NULL,       -- 回测结束日期
            initial_capital REAL NOT NULL,   -- 初始资金
            fee_rate REAL NOT NULL DEFAULT 0.001, -- 手续费率
            slippage REAL NOT NULL DEFAULT 0.0001, -- 滑点
            benchmark TEXT,                  -- 基准指数
            risk_free_rate REAL DEFAULT 0.02, -- 无风险利率
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 回测结果表 - 存储回测的主要结果
    schemas.insert(
        tables::BACKTEST_RESULTS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS backtest_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,        -- 用户ID
            strategy_id INTEGER NOT NULL,    -- 策略ID
            config_id INTEGER NOT NULL,      -- 配置ID
            name TEXT NOT NULL,              -- 回测名称
            status TEXT NOT NULL DEFAULT 'pending', -- 状态：'pending', 'running', 'completed', 'failed'
            start_date INTEGER NOT NULL,     -- 回测开始日期
            end_date INTEGER NOT NULL,       -- 回测结束日期
            initial_capital REAL NOT NULL,   -- 初始资金
            final_capital REAL,              -- 最终资金
            total_return REAL,               -- 总收益率
            annual_return REAL,              -- 年化收益率
            sharpe_ratio REAL,               -- 夏普比率
            sortino_ratio REAL,              -- 索提诺比率
            max_drawdown REAL,               -- 最大回撤
            max_drawdown_duration INTEGER,   -- 最大回撤持续时间（天）
            volatility REAL,                 -- 波动率
            win_rate REAL,                   -- 胜率
            profit_factor REAL,              -- 盈利因子
            total_trades INTEGER,            -- 总交易次数
            winning_trades INTEGER,          -- 盈利交易次数
            losing_trades INTEGER,           -- 亏损交易次数
            avg_win REAL,                    -- 平均盈利
            avg_loss REAL,                   -- 平均亏损
            largest_win REAL,                -- 最大盈利
            largest_loss REAL,               -- 最大亏损
            avg_trade_duration REAL,         -- 平均持仓时间（小时）
            error_message TEXT,              -- 错误信息（如果失败）
            created_at INTEGER NOT NULL,     -- 创建时间
            completed_at INTEGER,            -- 完成时间
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE,
            FOREIGN KEY (config_id) REFERENCES backtest_configs (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 权益曲线表 - 存储回测过程中的权益变化
    schemas.insert(
        tables::EQUITY_CURVES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS equity_curves (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            backtest_id INTEGER NOT NULL,    -- 回测ID
            timestamp INTEGER NOT NULL,      -- 时间戳
            equity REAL NOT NULL,            -- 权益
            cash REAL NOT NULL,              -- 现金
            position_value REAL NOT NULL,    -- 持仓价值
            drawdown REAL NOT NULL DEFAULT 0, -- 回撤
            benchmark_value REAL,            -- 基准价值
            FOREIGN KEY (backtest_id) REFERENCES backtest_results (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 回测交易记录表 - 存储回测过程中的所有交易
    schemas.insert(
        tables::BACKTEST_TRADES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS backtest_trades (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            backtest_id INTEGER NOT NULL,    -- 回测ID
            symbol TEXT NOT NULL,            -- 资产代码
            side TEXT NOT NULL,              -- 交易方向：'buy', 'sell'
            quantity REAL NOT NULL,          -- 交易数量
            price REAL NOT NULL,             -- 交易价格
            fee REAL NOT NULL DEFAULT 0,     -- 手续费
            slippage REAL NOT NULL DEFAULT 0, -- 滑点
            timestamp INTEGER NOT NULL,      -- 交易时间
            signal_type TEXT,                -- 信号类型：'entry', 'exit', 'stop_loss', 'take_profit'
            pnl REAL,                        -- 盈亏（对于卖出交易）
            cumulative_pnl REAL,             -- 累计盈亏
            FOREIGN KEY (backtest_id) REFERENCES backtest_results (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 回测持仓表 - 存储回测过程中的持仓变化
    schemas.insert(
        tables::BACKTEST_POSITIONS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS backtest_positions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            backtest_id INTEGER NOT NULL,    -- 回测ID
            symbol TEXT NOT NULL,            -- 资产代码
            timestamp INTEGER NOT NULL,      -- 时间戳
            quantity REAL NOT NULL,          -- 持仓数量
            avg_price REAL NOT NULL,         -- 平均成本价
            market_price REAL NOT NULL,      -- 市场价格
            market_value REAL NOT NULL,      -- 市场价值
            unrealized_pnl REAL NOT NULL,    -- 未实现盈亏
            FOREIGN KEY (backtest_id) REFERENCES backtest_results (id) ON DELETE CASCADE
        )"#.to_string(),
    );
    
    // 回测指标表 - 存储详细的回测指标
    schemas.insert(
        tables::BACKTEST_METRICS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS backtest_metrics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            backtest_id INTEGER NOT NULL,    -- 回测ID
            metric_name TEXT NOT NULL,       -- 指标名称
            metric_value REAL NOT NULL,      -- 指标值
            metric_description TEXT,         -- 指标描述
            category TEXT NOT NULL,          -- 指标分类：'return', 'risk', 'trade', 'ratio'
            created_at INTEGER NOT NULL,     -- 创建时间
            FOREIGN KEY (backtest_id) REFERENCES backtest_results (id) ON DELETE CASCADE,
            UNIQUE(backtest_id, metric_name)
        )"#.to_string(),
    );
}

/// 添加索引定义
fn add_index_schemas(schemas: &mut HashMap<String, String>) {
    // 回测结果表索引
    schemas.insert(
        indexes::IDX_BACKTEST_RESULTS_USER.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON backtest_results(user_id, created_at DESC)", 
                indexes::IDX_BACKTEST_RESULTS_USER),
    );
    
    schemas.insert(
        indexes::IDX_BACKTEST_RESULTS_STRATEGY.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON backtest_results(strategy_id, created_at DESC)", 
                indexes::IDX_BACKTEST_RESULTS_STRATEGY),
    );
    
    schemas.insert(
        indexes::IDX_BACKTEST_RESULTS_CREATED.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON backtest_results(status, created_at)", 
                indexes::IDX_BACKTEST_RESULTS_CREATED),
    );
    
    // 权益曲线表索引
    schemas.insert(
        indexes::IDX_EQUITY_CURVES_BACKTEST.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON equity_curves(backtest_id, timestamp)", 
                indexes::IDX_EQUITY_CURVES_BACKTEST),
    );
    
    // 回测交易记录表索引
    schemas.insert(
        indexes::IDX_BACKTEST_TRADES_BACKTEST.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON backtest_trades(backtest_id, timestamp)", 
                indexes::IDX_BACKTEST_TRADES_BACKTEST),
    );
    
    schemas.insert(
        indexes::IDX_BACKTEST_TRADES_SYMBOL.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON backtest_trades(backtest_id, symbol)", 
                indexes::IDX_BACKTEST_TRADES_SYMBOL),
    );
    
    // 回测持仓表索引
    schemas.insert(
        indexes::IDX_BACKTEST_POSITIONS_BACKTEST.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON backtest_positions(backtest_id, symbol, timestamp)", 
                indexes::IDX_BACKTEST_POSITIONS_BACKTEST),
    );
    
    // 回测指标表索引
    schemas.insert(
        indexes::IDX_BACKTEST_METRICS_BACKTEST.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON backtest_metrics(backtest_id, category)", 
                indexes::IDX_BACKTEST_METRICS_BACKTEST),
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
    
    // 添加示例回测配置
    sql.push_str(r#"-- 添加示例回测配置
INSERT OR IGNORE INTO backtest_configs (
    user_id, name, start_date, end_date, initial_capital, 
    fee_rate, slippage, benchmark, risk_free_rate, created_at, updated_at
)
VALUES 
    (1, '默认配置', strftime('%s', '2022-01-01'), strftime('%s', '2023-01-01'), 
     100000.0, 0.001, 0.0001, 'SPY', 0.02, strftime('%s', 'now'), strftime('%s', 'now')),
     
    (1, '低手续费配置', strftime('%s', '2022-01-01'), strftime('%s', '2023-01-01'), 
     50000.0, 0.0005, 0.00005, 'QQQ', 0.02, strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例回测结果
    sql.push_str(r#"-- 添加示例回测结果
INSERT OR IGNORE INTO backtest_results (
    user_id, strategy_id, config_id, name, status, start_date, end_date,
    initial_capital, final_capital, total_return, annual_return, sharpe_ratio,
    sortino_ratio, max_drawdown, max_drawdown_duration, volatility, win_rate,
    profit_factor, total_trades, winning_trades, losing_trades, avg_win, avg_loss,
    largest_win, largest_loss, avg_trade_duration, created_at, completed_at
)
VALUES 
    (1, 1, 1, '定投策略回测', 'completed', 
     strftime('%s', '2022-01-01'), strftime('%s', '2023-01-01'),
     100000.0, 115000.0, 0.15, 0.15, 1.25, 1.45, -0.08, 45, 0.18, 0.65,
     1.35, 24, 16, 8, 2500.0, -1200.0, 8500.0, -3200.0, 168.5,
     strftime('%s', 'now', '-5 days'), strftime('%s', 'now', '-5 days')),
     
    (1, 2, 1, '价值平均策略回测', 'completed',
     strftime('%s', '2022-01-01'), strftime('%s', '2023-01-01'),
     100000.0, 122000.0, 0.22, 0.22, 1.45, 1.68, -0.06, 32, 0.16, 0.72,
     1.58, 18, 13, 5, 3200.0, -1800.0, 12000.0, -4500.0, 240.3,
     strftime('%s', 'now', '-3 days'), strftime('%s', 'now', '-3 days')),
     
    (1, 3, 2, '动量策略回测', 'running',
     strftime('%s', '2022-01-01'), strftime('%s', '2023-01-01'),
     50000.0, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
     NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
     strftime('%s', 'now', '-1 day'), NULL);"#);
    sql.push_str("\n\n");
    
    // 添加示例权益曲线数据
    sql.push_str(r#"-- 添加示例权益曲线数据
INSERT OR IGNORE INTO equity_curves (
    backtest_id, timestamp, equity, cash, position_value, drawdown, benchmark_value
)
VALUES 
    (1, strftime('%s', '2022-01-01'), 100000.0, 95000.0, 5000.0, 0.0, 100000.0),
    (1, strftime('%s', '2022-02-01'), 102000.0, 90000.0, 12000.0, 0.0, 101500.0),
    (1, strftime('%s', '2022-03-01'), 98000.0, 85000.0, 13000.0, -0.04, 99800.0),
    (1, strftime('%s', '2022-04-01'), 105000.0, 80000.0, 25000.0, 0.0, 103200.0),
    (1, strftime('%s', '2022-05-01'), 110000.0, 75000.0, 35000.0, 0.0, 107500.0),
    
    (2, strftime('%s', '2022-01-01'), 100000.0, 92000.0, 8000.0, 0.0, 100000.0),
    (2, strftime('%s', '2022-02-01'), 105000.0, 87000.0, 18000.0, 0.0, 101500.0),
    (2, strftime('%s', '2022-03-01'), 108000.0, 82000.0, 26000.0, 0.0, 99800.0),
    (2, strftime('%s', '2022-04-01'), 115000.0, 77000.0, 38000.0, 0.0, 103200.0),
    (2, strftime('%s', '2022-05-01'), 122000.0, 72000.0, 50000.0, 0.0, 107500.0);"#);
    sql.push_str("\n\n");
    
    // 添加示例交易记录
    sql.push_str(r#"-- 添加示例交易记录
INSERT OR IGNORE INTO backtest_trades (
    backtest_id, symbol, side, quantity, price, fee, slippage, 
    timestamp, signal_type, pnl, cumulative_pnl
)
VALUES 
    (1, 'AAPL', 'buy', 30, 150.0, 4.5, 0.15, strftime('%s', '2022-01-15'), 'entry', NULL, 0.0),
    (1, 'AAPL', 'sell', 15, 160.0, 2.4, 0.08, strftime('%s', '2022-02-15'), 'exit', 142.5, 142.5),
    (1, 'MSFT', 'buy', 20, 250.0, 5.0, 0.25, strftime('%s', '2022-02-01'), 'entry', NULL, 142.5),
    (1, 'MSFT', 'sell', 20, 270.0, 5.4, 0.27, strftime('%s', '2022-03-15'), 'exit', 389.1, 531.6),
    
    (2, 'AAPL', 'buy', 40, 145.0, 5.8, 0.145, strftime('%s', '2022-01-10'), 'entry', NULL, 0.0),
    (2, 'AAPL', 'sell', 20, 165.0, 3.3, 0.165, strftime('%s', '2022-02-20'), 'exit', 396.4, 396.4),
    (2, 'GOOGL', 'buy', 5, 2800.0, 14.0, 2.8, strftime('%s', '2022-02-05'), 'entry', NULL, 396.4),
    (2, 'GOOGL', 'sell', 5, 3100.0, 15.5, 3.1, strftime('%s', '2022-04-10'), 'exit', 1465.9, 1862.3);"#);
    sql.push_str("\n\n");
    
    // 添加示例指标数据
    sql.push_str(r#"-- 添加示例指标数据
INSERT OR IGNORE INTO backtest_metrics (
    backtest_id, metric_name, metric_value, metric_description, category, created_at
)
VALUES 
    (1, 'calmar_ratio', 1.875, '卡尔玛比率', 'ratio', strftime('%s', 'now')),
    (1, 'information_ratio', 0.85, '信息比率', 'ratio', strftime('%s', 'now')),
    (1, 'beta', 0.92, '贝塔系数', 'risk', strftime('%s', 'now')),
    (1, 'alpha', 0.03, '阿尔法系数', 'return', strftime('%s', 'now')),
    (1, 'var_95', -0.025, '95%风险价值', 'risk', strftime('%s', 'now')),
    
    (2, 'calmar_ratio', 3.67, '卡尔玛比率', 'ratio', strftime('%s', 'now')),
    (2, 'information_ratio', 1.12, '信息比率', 'ratio', strftime('%s', 'now')),
    (2, 'beta', 0.88, '贝塔系数', 'risk', strftime('%s', 'now')),
    (2, 'alpha', 0.05, '阿尔法系数', 'return', strftime('%s', 'now')),
    (2, 'var_95', -0.018, '95%风险价值', 'risk', strftime('%s', 'now'));"#);
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
        assert!(schemas.contains_key(&tables::BACKTEST_RESULTS.to_string()));
        assert!(schemas.contains_key(&tables::EQUITY_CURVES.to_string()));
        assert!(schemas.contains_key(&tables::BACKTEST_TRADES.to_string()));
        assert!(schemas.contains_key(&tables::BACKTEST_POSITIONS.to_string()));
        assert!(schemas.contains_key(&tables::BACKTEST_METRICS.to_string()));
        assert!(schemas.contains_key(&tables::BACKTEST_CONFIGS.to_string()));
        
        // 验证所有索引都已定义
        assert!(schemas.contains_key(&indexes::IDX_BACKTEST_RESULTS_USER.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_BACKTEST_RESULTS_STRATEGY.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_EQUITY_CURVES_BACKTEST.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_BACKTEST_TRADES_BACKTEST.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_BACKTEST_POSITIONS_BACKTEST.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_BACKTEST_METRICS_BACKTEST.to_string()));
    }
    
    #[test]
    fn test_get_migration_sql() {
        let sql = get_migration_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含所有表
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS backtest_results"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS equity_curves"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS backtest_trades"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS backtest_positions"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS backtest_metrics"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS backtest_configs"));
    }
    
    #[test]
    fn test_get_sample_data_sql() {
        let sql = get_sample_data_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含示例数据
        assert!(sql.contains("INSERT OR IGNORE INTO backtest_configs"));
        assert!(sql.contains("INSERT OR IGNORE INTO backtest_results"));
        assert!(sql.contains("INSERT OR IGNORE INTO equity_curves"));
        assert!(sql.contains("INSERT OR IGNORE INTO backtest_trades"));
        assert!(sql.contains("INSERT OR IGNORE INTO backtest_metrics"));
        
        // 验证特定数据存在
        assert!(sql.contains("'completed'"));
        assert!(sql.contains("'running'"));
        assert!(sql.contains("'AAPL'"));
        assert!(sql.contains("'calmar_ratio'"));
    }
}