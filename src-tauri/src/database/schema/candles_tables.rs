use std::collections::HashMap;

/// K线相关表名常量
pub mod tables {
    pub const CANDLES: &str = "candles";
    pub const TIMEFRAMES: &str = "timeframes";
    pub const CANDLE_INDICATORS: &str = "candle_indicators";
}

/// 索引名常量
pub mod indexes {
    pub const IDX_CANDLES_SYMBOL: &str = "idx_candles_symbol";
    pub const IDX_CANDLES_TIMESTAMP: &str = "idx_candles_timestamp";
    pub const IDX_CANDLES_SYMBOL_TIMEFRAME: &str = "idx_candles_symbol_timeframe";
    pub const IDX_CANDLES_SOURCE: &str = "idx_candles_source";
    pub const IDX_CANDLE_INDICATORS_CANDLE_ID: &str = "idx_candle_indicators_candle_id";
    pub const IDX_CANDLE_INDICATORS_NAME: &str = "idx_candle_indicators_name";
}

/// 获取K线相关表的结构定义
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
    // 时间周期表 - 存储支持的K线时间周期
    schemas.insert(
        tables::TIMEFRAMES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS timeframes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,       -- 时间周期名称：'1m', '5m', '15m', '30m', '1h', '4h', '1d', '1w', '1M'
            seconds INTEGER NOT NULL,        -- 对应的秒数：60, 300, 900, 1800, 3600, 14400, 86400, 604800, 2592000
            description TEXT,                -- 描述
            is_enabled BOOLEAN NOT NULL DEFAULT 1 -- 是否启用
        )"#.to_string(),
    );
    
    // K线数据表 - 存储各种资产的K线数据
    schemas.insert(
        tables::CANDLES.to_string(),
        r#"CREATE TABLE IF NOT EXISTS candles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            symbol TEXT NOT NULL,            -- 资产代码
            source TEXT NOT NULL,            -- 数据源：'yahoo', 'binance', 'alphavantage', etc.
            timeframe_id INTEGER NOT NULL,   -- 时间周期ID
            timestamp INTEGER NOT NULL,      -- 时间戳（Unix时间戳，秒）
            open REAL NOT NULL,              -- 开盘价
            high REAL NOT NULL,              -- 最高价
            low REAL NOT NULL,               -- 最低价
            close REAL NOT NULL,             -- 收盘价
            volume REAL NOT NULL,            -- 成交量
            quote_volume REAL,               -- 成交额（对于加密货币）
            trades INTEGER,                  -- 成交笔数
            is_complete BOOLEAN NOT NULL DEFAULT 1, -- K线是否完成（最后一根可能未完成）
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (timeframe_id) REFERENCES timeframes(id),
            UNIQUE(symbol, source, timeframe_id, timestamp)
        )"#.to_string(),
    );
    
    // K线指标表 - 存储预计算的技术指标
    schemas.insert(
        tables::CANDLE_INDICATORS.to_string(),
        r#"CREATE TABLE IF NOT EXISTS candle_indicators (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            candle_id INTEGER NOT NULL,      -- 关联的K线ID
            name TEXT NOT NULL,              -- 指标名称：'ma', 'ema', 'rsi', 'macd', etc.
            parameters TEXT NOT NULL,        -- JSON格式的指标参数
            value REAL NOT NULL,             -- 指标值
            created_at INTEGER NOT NULL,     -- 创建时间
            updated_at INTEGER NOT NULL,     -- 更新时间
            FOREIGN KEY (candle_id) REFERENCES candles(id) ON DELETE CASCADE,
            UNIQUE(candle_id, name, parameters)
        )"#.to_string(),
    );
}

/// 添加索引定义
fn add_index_schemas(schemas: &mut HashMap<String, String>) {
    // K线表索引
    schemas.insert(
        indexes::IDX_CANDLES_SYMBOL.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON candles(symbol, source)", 
                indexes::IDX_CANDLES_SYMBOL),
    );
    
    schemas.insert(
        indexes::IDX_CANDLES_TIMESTAMP.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON candles(timestamp)", 
                indexes::IDX_CANDLES_TIMESTAMP),
    );
    
    schemas.insert(
        indexes::IDX_CANDLES_SYMBOL_TIMEFRAME.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON candles(symbol, timeframe_id, timestamp DESC)", 
                indexes::IDX_CANDLES_SYMBOL_TIMEFRAME),
    );
    
    schemas.insert(
        indexes::IDX_CANDLES_SOURCE.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON candles(source, symbol, timeframe_id)", 
                indexes::IDX_CANDLES_SOURCE),
    );
    
    // K线指标表索引
    schemas.insert(
        indexes::IDX_CANDLE_INDICATORS_CANDLE_ID.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON candle_indicators(candle_id)", 
                indexes::IDX_CANDLE_INDICATORS_CANDLE_ID),
    );
    
    schemas.insert(
        indexes::IDX_CANDLE_INDICATORS_NAME.to_string(),
        format!("CREATE INDEX IF NOT EXISTS {} ON candle_indicators(name, parameters)", 
                indexes::IDX_CANDLE_INDICATORS_NAME),
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
    
    // 添加默认时间周期
    sql.push_str(r#"-- 添加默认时间周期
INSERT OR IGNORE INTO timeframes (name, seconds, description, is_enabled) VALUES 
    ('1m', 60, '1分钟', 1),
    ('5m', 300, '5分钟', 1),
    ('15m', 900, '15分钟', 1),
    ('30m', 1800, '30分钟', 1),
    ('1h', 3600, '1小时', 1),
    ('4h', 14400, '4小时', 1),
    ('1d', 86400, '日线', 1),
    ('1w', 604800, '周线', 1),
    ('1M', 2592000, '月线', 1);"#);
    sql.push_str("\n\n");
    
    // 提交事务
    sql.push_str("COMMIT;");
    
    sql
}

/// 获取示例数据 SQL 脚本（用于开发和测试）
pub fn get_sample_data_sql() -> String {
    let mut sql = String::new();
    
    sql.push_str("BEGIN TRANSACTION;\n\n");
    
    // 添加示例K线数据 - 日线
    sql.push_str(r#"-- 添加示例日线数据（苹果股票）
INSERT OR IGNORE INTO candles (
    symbol, source, timeframe_id, timestamp, 
    open, high, low, close, volume, 
    is_complete, created_at, updated_at
)
VALUES 
    ('AAPL', 'yahoo', 7, strftime('%s', '2023-01-03'), 
     130.28, 130.90, 124.17, 125.07, 112117500, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('AAPL', 'yahoo', 7, strftime('%s', '2023-01-04'), 
     126.89, 128.66, 125.08, 126.36, 89113600, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('AAPL', 'yahoo', 7, strftime('%s', '2023-01-05'), 
     127.13, 127.77, 124.76, 125.02, 80829300, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('AAPL', 'yahoo', 7, strftime('%s', '2023-01-06'), 
     126.01, 130.29, 124.89, 129.62, 87686600, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('AAPL', 'yahoo', 7, strftime('%s', '2023-01-09'), 
     130.47, 133.41, 129.89, 130.15, 70790800, 
     1, strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例K线数据 - 小时线
    sql.push_str(r#"-- 添加示例小时线数据（比特币）
INSERT OR IGNORE INTO candles (
    symbol, source, timeframe_id, timestamp, 
    open, high, low, close, volume, quote_volume, trades,
    is_complete, created_at, updated_at
)
VALUES 
    ('BTCUSDT', 'binance', 5, strftime('%s', '2023-01-03 09:00:00'), 
     16750.12, 16788.45, 16723.67, 16780.23, 1250.34, 20982345.67, 8765, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('BTCUSDT', 'binance', 5, strftime('%s', '2023-01-03 10:00:00'), 
     16780.23, 16820.56, 16760.89, 16795.45, 980.56, 16456789.23, 6543, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('BTCUSDT', 'binance', 5, strftime('%s', '2023-01-03 11:00:00'), 
     16795.45, 16830.12, 16785.34, 16810.67, 1120.78, 18765432.45, 7654, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('BTCUSDT', 'binance', 5, strftime('%s', '2023-01-03 12:00:00'), 
     16810.67, 16845.23, 16790.56, 16825.78, 1350.45, 22678901.34, 9876, 
     1, strftime('%s', 'now'), strftime('%s', 'now')),
     
    ('BTCUSDT', 'binance', 5, strftime('%s', '2023-01-03 13:00:00'), 
     16825.78, 16860.34, 16815.67, 16840.12, 1050.23, 17654321.56, 7123, 
     1, strftime('%s', 'now'), strftime('%s', 'now'));"#);
    sql.push_str("\n\n");
    
    // 添加示例指标数据
    sql.push_str(r#"-- 添加示例指标数据
INSERT OR IGNORE INTO candle_indicators (
    candle_id, name, parameters, value, created_at, updated_at
)
VALUES 
    (1, 'ma', '{"period": 5}', 127.24, strftime('%s', 'now'), strftime('%s', 'now')),
    (2, 'ma', '{"period": 5}', 126.43, strftime('%s', 'now'), strftime('%s', 'now')),
    (3, 'ma', '{"period": 5}', 126.49, strftime('%s', 'now'), strftime('%s', 'now')),
    (4, 'ma', '{"period": 5}', 127.23, strftime('%s', 'now'), strftime('%s', 'now')),
    (5, 'ma', '{"period": 5}', 128.04, strftime('%s', 'now'), strftime('%s', 'now')),
    
    (1, 'rsi', '{"period": 14}', 42.5, strftime('%s', 'now'), strftime('%s', 'now')),
    (2, 'rsi', '{"period": 14}', 45.3, strftime('%s', 'now'), strftime('%s', 'now')),
    (3, 'rsi', '{"period": 14}', 43.8, strftime('%s', 'now'), strftime('%s', 'now')),
    (4, 'rsi', '{"period": 14}', 58.2, strftime('%s', 'now'), strftime('%s', 'now')),
    (5, 'rsi', '{"period": 14}', 54.7, strftime('%s', 'now'), strftime('%s', 'now'));"#);
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
        assert!(schemas.contains_key(&tables::CANDLES.to_string()));
        assert!(schemas.contains_key(&tables::TIMEFRAMES.to_string()));
        assert!(schemas.contains_key(&tables::CANDLE_INDICATORS.to_string()));
        
        // 验证所有索引都已定义
        assert!(schemas.contains_key(&indexes::IDX_CANDLES_SYMBOL.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_CANDLES_TIMESTAMP.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_CANDLES_SYMBOL_TIMEFRAME.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_CANDLES_SOURCE.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_CANDLE_INDICATORS_CANDLE_ID.to_string()));
        assert!(schemas.contains_key(&indexes::IDX_CANDLE_INDICATORS_NAME.to_string()));
    }
    
    #[test]
    fn test_get_migration_sql() {
        let sql = get_migration_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含所有表
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS candles"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS timeframes"));
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS candle_indicators"));
        
        // 验证 SQL 脚本包含默认时间周期
        assert!(sql.contains("INSERT OR IGNORE INTO timeframes"));
        assert!(sql.contains("'1m'"));
        assert!(sql.contains("'1d'"));
    }
    
    #[test]
    fn test_get_sample_data_sql() {
        let sql = get_sample_data_sql();
        
        // 验证 SQL 脚本包含事务
        assert!(sql.starts_with("BEGIN TRANSACTION;"));
        assert!(sql.ends_with("COMMIT;"));
        
        // 验证 SQL 脚本包含示例数据
        assert!(sql.contains("INSERT OR IGNORE INTO candles"));
        assert!(sql.contains("INSERT OR IGNORE INTO candle_indicators"));
        
        // 验证特定数据存在
        assert!(sql.contains("'AAPL'"));
        assert!(sql.contains("'BTCUSDT'"));
        assert!(sql.contains("'ma'"));
        assert!(sql.contains("'rsi'"));
    }
}