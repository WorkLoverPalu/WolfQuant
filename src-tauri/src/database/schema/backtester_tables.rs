use std::collections::HashMap;

/// 获取回测相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();
    
    // 回测结果表
    schemas.insert(
        "backtest_results".to_string(),
        "CREATE TABLE IF NOT EXISTS backtest_results (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            strategy_id INTEGER NOT NULL,
            initial_capital REAL NOT NULL,
            fee_rate REAL NOT NULL,
            slippage REAL NOT NULL,
            total_return REAL NOT NULL,
            annual_return REAL NOT NULL,
            sharpe_ratio REAL NOT NULL,
            max_drawdown REAL NOT NULL,
            win_rate REAL NOT NULL,
            profit_factor REAL NOT NULL,
            total_trades INTEGER NOT NULL,
            winning_trades INTEGER NOT NULL,
            losing_trades INTEGER NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
            FOREIGN KEY (strategy_id) REFERENCES strategies (id) ON DELETE CASCADE
        )".to_string(),
    );
    
    // 权益曲线表
    schemas.insert(
        "equity_curves".to_string(),
        "CREATE TABLE IF NOT EXISTS equity_curves (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            backtest_id INTEGER NOT NULL,
            timestamp INTEGER NOT NULL,
            equity REAL NOT NULL,
            FOREIGN KEY (backtest_id) REFERENCES backtest_results (id) ON DELETE CASCADE
        )".to_string(),
    );
    
    // 回测交易记录表
    schemas.insert(
        "backtest_trades".to_string(),
        "CREATE TABLE IF NOT EXISTS backtest_trades (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            backtest_id INTEGER NOT NULL,
            symbol TEXT NOT NULL,
            side TEXT NOT NULL,
            quantity REAL NOT NULL,
            price REAL NOT NULL,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY (backtest_id) REFERENCES backtest_results (id) ON DELETE CASCADE
        )".to_string(),
    );
    
    // 添加索引以提高查询性能
    schemas.insert(
        "idx_backtest_results_strategy".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_backtest_results_strategy 
         ON backtest_results(strategy_id)".to_string(),
    );
    
    schemas.insert(
        "idx_equity_curves_backtest".to_string(),
        "CREATE INDEX IF NOT EXISTS idx_equity_curves_backtest 
         ON equity_curves(backtest_id, timestamp)".to_string(),
    );
    
    schemas
}
