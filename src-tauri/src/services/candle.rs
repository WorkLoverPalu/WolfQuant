use chrono::{DateTime, Utc};
use log::error;
use rusqlite::{params, Result as SqliteResult};
use crate::error::auth::AuthError;
use crate::database::get_connection_from_pool;
use crate::models::candle::{Candle, CandleModel};

pub struct CandleService;

impl CandleService {
    pub fn save_candle(&self, candle: &Candle) -> Result<i64, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let result = conn.execute(
            "INSERT INTO candles (symbol, source, timestamp, open, high, low, close, volume)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(symbol, source, timestamp) DO UPDATE SET
             open = excluded.open,
             high = excluded.high,
             low = excluded.low,
             close = excluded.close,
             volume = excluded.volume",
            params![
                &candle.symbol,
                &candle.source,
                candle.timestamp,
                candle.open,
                candle.high,
                candle.low,
                candle.close,
                candle.volume
            ],
        ).map_err(|e| format!("Failed to save candle: {}", e))?;

        // 获取最后插入的行ID
        let last_id = conn.last_insert_rowid();
        
        Ok(last_id)
    }

    pub fn save_candles(&self, candles: &[Candle]) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // 使用事务来批量插入
        let tx = conn.transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        
        for candle in candles {
            tx.execute(
                "INSERT INTO candles (symbol, source, timestamp, open, high, low, close, volume)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(symbol, source, timestamp) DO UPDATE SET
                 open = excluded.open,
                 high = excluded.high,
                 low = excluded.low,
                 close = excluded.close,
                 volume = excluded.volume",
                params![
                    &candle.symbol,
                    &candle.source,
                    candle.timestamp,
                    candle.open,
                    candle.high,
                    candle.low,
                    candle.close,
                    candle.volume
                ],
            ).map_err(|e| format!("Failed to save candle in batch: {}", e))?;
        }
        
        tx.commit().map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
        Ok(())
    }

    pub fn get_candles(
        &self,
        symbol: &str,
        source: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<Candle>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let start_timestamp = start_time.timestamp();
        let end_timestamp = end_time.timestamp();
        
        let mut stmt = conn.prepare(
            "SELECT id, symbol, source, timestamp, open, high, low, close, volume FROM candles
             WHERE symbol = ?1 AND source = ?2 AND timestamp BETWEEN ?3 AND ?4
             ORDER BY timestamp ASC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let candle_iter = stmt.query_map(
            params![symbol, source, start_timestamp, end_timestamp],
            |row| {
                Ok(CandleModel {
                    id: row.get(0)?,
                    symbol: row.get(1)?,
                    source: row.get(2)?,
                    timestamp: row.get(3)?,
                    open: row.get(4)?,
                    high: row.get(5)?,
                    low: row.get(6)?,
                    close: row.get(7)?,
                    volume: row.get(8)?,
                })
            }
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut candles = Vec::new();
        for candle_result in candle_iter {
            match candle_result {
                Ok(model) => candles.push(Candle::from(model)),
                Err(e) => return Err(format!("Failed to process candle row: {}", e)),
            }
        }
        
        Ok(candles)
    }

    pub fn get_latest_candle(
        &self,
        symbol: &str,
        source: &str,
    ) -> Result<Option<Candle>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, symbol, source, timestamp, open, high, low, close, volume FROM candles
             WHERE symbol = ?1 AND source = ?2
             ORDER BY timestamp DESC
             LIMIT 1"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let candle_result = stmt.query_row(
            params![symbol, source],
            |row| {
                Ok(CandleModel {
                    id: row.get(0)?,
                    symbol: row.get(1)?,
                    source: row.get(2)?,
                    timestamp: row.get(3)?,
                    open: row.get(4)?,
                    high: row.get(5)?,
                    low: row.get(6)?,
                    close: row.get(7)?,
                    volume: row.get(8)?,
                })
            }
        );
        
        match candle_result {
            Ok(model) => Ok(Some(Candle::from(model))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get latest candle: {}", e)),
        }
    }
}
