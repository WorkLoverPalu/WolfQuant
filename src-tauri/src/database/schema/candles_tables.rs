use std::collections::HashMap;

/// 获取k线相关表的结构定义
pub fn get_schemas() -> HashMap<String, String> {
    let mut schemas = HashMap::new();

    // 资产类型表
    schemas.insert(
        "candles".to_string(),
        "CREATE TABLE IF NOT EXISTS candles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                symbol TEXT NOT NULL,
                source TEXT NOT NULL,
                timestamp DATETIME NOT NULL,
                open REAL NOT NULL,
                high REAL NOT NULL,
                low REAL NOT NULL,
                close REAL NOT NULL,
                volume REAL NOT NULL,
                UNIQUE(symbol, source, timestamp)
            )"
        .to_string(),
    );

    schemas
}
