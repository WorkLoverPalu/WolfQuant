// src/db/repository.rs 中添加以下内容

use chrono::{DateTime, Duration, Utc};
use sqlx::sqlite::SqlitePool;
use std::collections::HashSet;
use std::path::Path;

use crate::db::models::{CandleModel, DatasetInfo, ImportStatus, ImportTask, ProductModel};
use crate::models::{Candle, Product};

impl Repository {
    // 初始化数据库时添加导入任务表
    async fn init_db(pool: &SqlitePool) -> Result<(), String> {
        // 创建产品表和K线表（原有代码）...
        
        // 创建导入任务表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS import_tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                asset_type TEXT NOT NULL,
                source TEXT NOT NULL,
                symbol TEXT NOT NULL,
                start_time DATETIME NOT NULL,
                end_time DATETIME NOT NULL,
                interval TEXT NOT NULL,
                status TEXT NOT NULL,
                progress REAL NOT NULL,
                error TEXT,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL,
                completed_at DATETIME
            )"
        )
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create import_tasks table: {}", e))?;
        
        // 创建K线表索引以提高查询性能
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_candles_symbol_source_timestamp 
             ON candles (symbol, source, timestamp)"
        )
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to create candles index: {}", e))?;
        
        Ok(())
    }
    
    // 创建导入任务
    pub async fn create_import_task(
        &self,
        asset_type: &str,
        source: &str,
        symbol: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: &str,
    ) -> Result<i64, String> {
        let now = Utc::now();
        
        let result = sqlx::query(
            "INSERT INTO import_tasks (
                asset_type, source, symbol, start_time, end_time, interval,
                status, progress, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(asset_type)
        .bind(source)
        .bind(symbol)
        .bind(start_time)
        .bind(end_time)
        .bind(interval)
        .bind("pending")
        .bind(0.0)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to create import task: {}", e))?;
        
        Ok(result.last_insert_rowid())
    }
    
    // 更新导入任务状态
    pub async fn update_import_task_status(
        &self,
        task_id: i64,
        status: ImportStatus,
        progress: f64,
        error: Option<&str>,
    ) -> Result<(), String> {
        let now = Utc::now();
        let status_str = match status {
            ImportStatus::Pending => "pending",
            ImportStatus::InProgress => "in_progress",
            ImportStatus::Completed => "completed",
            ImportStatus::Failed => "failed",
        };
        
        let completed_at = if status == ImportStatus::Completed || status == ImportStatus::Failed {
            Some(now)
        } else {
            None
        };
        
        sqlx::query(
            "UPDATE import_tasks 
             SET status = ?, progress = ?, error = ?, updated_at = ?, completed_at = ?
             WHERE id = ?"
        )
        .bind(status_str)
        .bind(progress)
        .bind(error)
        .bind(now)
        .bind(completed_at)
        .bind(task_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to update import task: {}", e))?;
        
        Ok(())
    }
    
    // 获取导入任务
    pub async fn get_import_task(&self, task_id: i64) -> Result<Option<ImportTask>, String> {
        let task = sqlx::query_as::<_, ImportTask>(
            "SELECT * FROM import_tasks WHERE id = ?"
        )
        .bind(task_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to get import task: {}", e))?;
        
        Ok(task)
    }
    
    // 获取所有导入任务
    pub async fn get_import_tasks(&self) -> Result<Vec<ImportTask>, String> {
        let tasks = sqlx::query_as::<_, ImportTask>(
            "SELECT * FROM import_tasks ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get import tasks: {}", e))?;
        
        Ok(tasks)
    }
    
    // 获取可用数据集信息
    pub async fn get_available_datasets(&self) -> Result<Vec<DatasetInfo>, String> {
        let datasets = sqlx::query_as::<_, DatasetInfo>(
            "SELECT 
                c.asset_type, 
                c.source, 
                c.symbol, 
                p.name,
                MIN(c.timestamp) as min_timestamp, 
                MAX(c.timestamp) as max_timestamp,
                COUNT(*) as candle_count,
                GROUP_CONCAT(DISTINCT c.interval) as intervals
             FROM candles c
             LEFT JOIN products p ON c.symbol = p.symbol AND c.source = p.source
             GROUP BY c.asset_type, c.source, c.symbol
             ORDER BY c.asset_type, c.source, c.symbol"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get available datasets: {}", e))?;
        
        Ok(datasets)
    }
    
    // 检查数据集是否存在
    pub async fn check_dataset_exists(
        &self,
        symbol: &str,
        source: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<bool, String> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM candles 
             WHERE symbol = ? AND source = ? 
             AND timestamp BETWEEN ? AND ?"
        )
        .bind(symbol)
        .bind(source)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to check dataset: {}", e))?;
        
        Ok(count.0 > 0)
    }
    
    // 扩展 Candle 模型以包含 interval 字段
    pub async fn save_candle_with_interval(
        &self,
        candle: &Candle,
        interval: &str,
    ) -> Result<i64, String> {
        let result = sqlx::query(
            "INSERT INTO candles (symbol, source, asset_type, timestamp, open, high, low, close, volume, interval)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(symbol, source, timestamp, interval) DO UPDATE SET
             open = excluded.open,
             high = excluded.high,
             low = excluded.low,
             close = excluded.close,
             volume = excluded.volume"
        )
        .bind(&candle.symbol)
        .bind(&candle.source)
        .bind(&candle.asset_type)
        .bind(candle.timestamp)
        .bind(candle.open)
        .bind(candle.high)
        .bind(candle.low)
        .bind(candle.close)
        .bind(candle.volume)
        .bind(interval)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to save candle: {}", e))?;
        
        Ok(result.last_insert_rowid())
    }
    
    // 批量保存带有间隔的K线数据
    pub async fn save_candles_with_interval(
        &self,
        candles: &[Candle],
        interval: &str,
    ) -> Result<(), String> {
        // 使用事务批量插入以提高性能
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        
        for candle in candles {
            sqlx::query(
                "INSERT INTO candles (symbol, source, asset_type, timestamp, open, high, low, close, volume, interval)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT(symbol, source, timestamp, interval) DO UPDATE SET
                 open = excluded.open,
                 high = excluded.high,
                 low = excluded.low,
                 close = excluded.close,
                 volume = excluded.volume"
            )
            .bind(&candle.symbol)
            .bind(&candle.source)
            .bind(&candle.asset_type)
            .bind(candle.timestamp)
            .bind(candle.open)
            .bind(candle.high)
            .bind(candle.low)
            .bind(candle.close)
            .bind(candle.volume)
            .bind(interval)
            .execute(&mut tx)
            .await
            .map_err(|e| format!("Failed to save candle in batch: {}", e))?;
        }
        
        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
        Ok(())
    }
    
    // 获取带有间隔的K线数据
    pub async fn get_candles_with_interval(
        &self,
        symbol: &str,
        source: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        interval: &str,
    ) -> Result<Vec<Candle>, String> {
        let models = sqlx::query_as::<_, CandleModel>(
            "SELECT * FROM candles
             WHERE symbol = ? AND source = ? AND interval = ? AND timestamp BETWEEN ? AND ?
             ORDER BY timestamp ASC"
        )
        .bind(symbol)
        .bind(source)
        .bind(interval)
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("Failed to get candles: {}", e))?;
        
        Ok(models.into_iter().map(|m| m.into()).collect())
    }
}