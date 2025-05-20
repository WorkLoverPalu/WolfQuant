use chrono::Utc;
use log::{info, error};
use serde_json::{json, Value};
use crate::error::auth::AuthError;
use crate::database::get_connection_from_pool;
use crate::models::strategy::{
    Strategy, StrategyType, StrategyVersion, StrategyTag, StrategyRating,
    StrategyApplication, CreateStrategyRequest, UpdateStrategyRequest
};
use crate::models::candle::Candle;
use crate::models::trading::OrderSignal;
use std::collections::HashMap;

/// 策略服务，负责管理交易策略
pub struct StrategyService;

impl StrategyService {
    /// 创建新的策略服务实例
    pub fn new() -> Self {
        Self {}
    }
    
    /// 创建新策略
    pub fn create_strategy(
        &self,
        user_id: i64,
        request: CreateStrategyRequest,
    ) -> Result<i64, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let now = Utc::now().timestamp();
        
        // 验证策略参数是否有效
        self.validate_strategy_parameters(&request.strategy_type, &request.parameters)?;
        
        // 插入策略记录
        let strategy_id = conn.execute(
            "INSERT INTO strategies (
                user_id, name, description, strategy_type, parameters,
                is_public, is_active, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                user_id,
                request.name,
                request.description,
                request.strategy_type,
                request.parameters,
                request.is_public,
                true, // 默认激活
                now,
                now
            ],
        ).map_err(|e| format!("Failed to create strategy: {}", e))?;
        
        let strategy_id = conn.last_insert_rowid();
        
        // 创建初始版本
        conn.execute(
            "INSERT INTO strategy_versions (
                strategy_id, version, parameters, description, created_at
            ) VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![
                strategy_id,
                1, // 初始版本为1
                request.parameters,
                request.description,
                now
            ],
        ).map_err(|e| format!("Failed to create strategy version: {}", e))?;
        
        info!("Created new strategy: {} (ID: {})", request.name, strategy_id);
        
        Ok(strategy_id)
    }
    
    /// 更新策略
    pub fn update_strategy(
        &self,
        user_id: i64,
        request: UpdateStrategyRequest,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // 检查策略是否存在且属于该用户
        let strategy = self.get_strategy(request.id)?;
        if strategy.user_id != user_id {
            return Err("You don't have permission to update this strategy".to_string());
        }
        
        // 验证策略参数是否有效
        self.validate_strategy_parameters(&strategy.strategy_type.to_str(), &request.parameters)?;
        
        let now = Utc::now().timestamp();
        
        // 开始事务
        let tx = conn.transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        
        // 更新策略
        tx.execute(
            "UPDATE strategies SET
                name = ?, description = ?, parameters = ?,
                is_public = ?, updated_at = ?
             WHERE id = ? AND user_id = ?",
            rusqlite::params![
                request.name,
                request.description,
                request.parameters,
                request.is_public,
                now,
                request.id,
                user_id
            ],
        ).map_err(|e| format!("Failed to update strategy: {}", e))?;
        
        // 获取当前最高版本号
        let current_version: i32 = tx.query_row(
            "SELECT MAX(version) FROM strategy_versions WHERE strategy_id = ?",
            rusqlite::params![request.id],
            |row| row.get(0),
        ).map_err(|e| format!("Failed to get current version: {}", e))?;
        
        // 创建新版本
        tx.execute(
            "INSERT INTO strategy_versions (
                strategy_id, version, parameters, description, created_at
            ) VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![
                request.id,
                current_version + 1,
                request.parameters,
                request.description,
                now
            ],
        ).map_err(|e| format!("Failed to create strategy version: {}", e))?;
        
        // 提交事务
        tx.commit().map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
        info!("Updated strategy: {} (ID: {})", request.name, request.id);
        
        Ok(())
    }
    
    /// 删除策略
    pub fn delete_strategy(
        &self,
        user_id: i64,
        strategy_id: i64,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // 检查策略是否存在且属于该用户
        let strategy = self.get_strategy(strategy_id)?;
        if strategy.user_id != user_id {
            return Err("You don't have permission to delete this strategy".to_string());
        }
        
        // 删除策略（级联删除会自动删除相关记录）
        conn.execute(
            "DELETE FROM strategies WHERE id = ? AND user_id = ?",
            rusqlite::params![strategy_id, user_id],
        ).map_err(|e| format!("Failed to delete strategy: {}", e))?;
        
        info!("Deleted strategy: {} (ID: {})", strategy.name, strategy_id);
        
        Ok(())
    }
    
    /// 获取策略
    pub fn get_strategy(
        &self,
        strategy_id: i64,
    ) -> Result<Strategy, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let strategy = conn.query_row(
            "SELECT id, user_id, name, description, strategy_type, parameters,
                    is_public, is_active, created_at, updated_at
             FROM strategies WHERE id = ?",
            rusqlite::params![strategy_id],
            |row| {
                Ok(Strategy {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    strategy_type: StrategyType::from_str(&row.get::<_, String>(4)?),
                    parameters: row.get(5)?,
                    is_public: row.get(6)?,
                    is_active: row.get(7)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(8)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(9)?, 0),
                })
            },
        ).map_err(|e| format!("Failed to get strategy: {}", e))?;
        
        Ok(strategy)
    }
    
    /// 获取用户的所有策略
    pub fn get_user_strategies(
        &self,
        user_id: i64,
    ) -> Result<Vec<Strategy>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, description, strategy_type, parameters,
                    is_public, is_active, created_at, updated_at
             FROM strategies WHERE user_id = ? ORDER BY updated_at DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let strategy_iter = stmt.query_map(
            rusqlite::params![user_id],
            |row| {
                Ok(Strategy {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    strategy_type: StrategyType::from_str(&row.get::<_, String>(4)?),
                    parameters: row.get(5)?,
                    is_public: row.get(6)?,
                    is_active: row.get(7)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(8)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(9)?, 0),
                })
            },
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut strategies = Vec::new();
        for strategy_result in strategy_iter {
            match strategy_result {
                Ok(strategy) => strategies.push(strategy),
                Err(e) => return Err(format!("Failed to process strategy row: {}", e)),
            }
        }
        
        Ok(strategies)
    }
    
    /// 获取公开策略
    pub fn get_public_strategies(
        &self,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<Strategy>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let offset = (page - 1) * page_size;
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, description, strategy_type, parameters,
                    is_public, is_active, created_at, updated_at
             FROM strategies 
             WHERE is_public = 1 AND is_active = 1
             ORDER BY updated_at DESC
             LIMIT ? OFFSET ?"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let strategy_iter = stmt.query_map(
            rusqlite::params![page_size as i64, offset as i64],
            |row| {
                Ok(Strategy {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    strategy_type: StrategyType::from_str(&row.get::<_, String>(4)?),
                    parameters: row.get(5)?,
                    is_public: row.get(6)?,
                    is_active: row.get(7)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(8)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(9)?, 0),
                })
            },
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut strategies = Vec::new();
        for strategy_result in strategy_iter {
            match strategy_result {
                Ok(strategy) => strategies.push(strategy),
                Err(e) => return Err(format!("Failed to process strategy row: {}", e)),
            }
        }
        
        Ok(strategies)
    }
    
    /// 获取策略版本历史
    pub fn get_strategy_versions(
        &self,
        strategy_id: i64,
    ) -> Result<Vec<StrategyVersion>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, strategy_id, version, parameters, description, created_at
             FROM strategy_versions
             WHERE strategy_id = ?
             ORDER BY version DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let version_iter = stmt.query_map(
            rusqlite::params![strategy_id],
            |row| {
                Ok(StrategyVersion {
                    id: row.get(0)?,
                    strategy_id: row.get(1)?,
                    version: row.get(2)?,
                    parameters: row.get(3)?,
                    description: row.get(4)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(5)?, 0),
                })
            },
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut versions = Vec::new();
        for version_result in version_iter {
            match version_result {
                Ok(version) => versions.push(version),
                Err(e) => return Err(format!("Failed to process version row: {}", e)),
            }
        }
        
        Ok(versions)
    }
    
    /// 添加策略标签
    pub fn add_strategy_tag(
        &self,
        strategy_id: i64,
        tag_name: &str,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // 开始事务
        let tx = conn.transaction()
            .map_err(|e| format!("Failed to start transaction: {}", e))?;
        
        // 获取或创建标签
        let tag_id = match tx.query_row(
            "SELECT id FROM strategy_tags WHERE name = ?",
            rusqlite::params![tag_name],
            |row| row.get::<_, i64>(0),
        ) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // 创建新标签
                tx.execute(
                    "INSERT INTO strategy_tags (name) VALUES (?)",
                    rusqlite::params![tag_name],
                ).map_err(|e| format!("Failed to create tag: {}", e))?;
                
                tx.last_insert_rowid()
            },
            Err(e) => return Err(format!("Failed to get tag: {}", e)),
        };
        
        // 添加标签关联
        tx.execute(
            "INSERT OR IGNORE INTO strategy_tag_relations (strategy_id, tag_id) VALUES (?, ?)",
            rusqlite::params![strategy_id, tag_id],
        ).map_err(|e| format!("Failed to add tag relation: {}", e))?;
        
        // 提交事务
        tx.commit().map_err(|e| format!("Failed to commit transaction: {}", e))?;
        
        Ok(())
    }
    
    /// 移除策略标签
    pub fn remove_strategy_tag(
        &self,
        strategy_id: i64,
        tag_name: &str,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        // 获取标签ID
        let tag_id: Result<i64, rusqlite::Error> = conn.query_row(
            "SELECT id FROM strategy_tags WHERE name = ?",
            rusqlite::params![tag_name],
            |row| row.get(0),
        );
        
        match tag_id {
            Ok(id) => {
                // 移除标签关联
                conn.execute(
                    "DELETE FROM strategy_tag_relations WHERE strategy_id = ? AND tag_id = ?",
                    rusqlite::params![strategy_id, id],
                ).map_err(|e| format!("Failed to remove tag relation: {}", e))?;
                
                Ok(())
            },
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // 标签不存在，视为成功
                Ok(())
            },
            Err(e) => Err(format!("Failed to get tag: {}", e)),
        }
    }
    
    /// 获取策略标签
    pub fn get_strategy_tags(
        &self,
        strategy_id: i64,
    ) -> Result<Vec<StrategyTag>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name
             FROM strategy_tags t
             JOIN strategy_tag_relations r ON t.id = r.tag_id
             WHERE r.strategy_id = ?
             ORDER BY t.name"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let tag_iter = stmt.query_map(
            rusqlite::params![strategy_id],
            |row| {
                Ok(StrategyTag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            },
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut tags = Vec::new();
        for tag_result in tag_iter {
            match tag_result {
                Ok(tag) => tags.push(tag),
                Err(e) => return Err(format!("Failed to process tag row: {}", e)),
            }
        }
        
        Ok(tags)
    }
    
    /// 收藏策略
    pub fn favorite_strategy(
        &self,
        user_id: i64,
        strategy_id: i64,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let now = Utc::now().timestamp();
        
        conn.execute(
            "INSERT OR REPLACE INTO strategy_favorites (user_id, strategy_id, created_at)
             VALUES (?, ?, ?)",
            rusqlite::params![user_id, strategy_id, now],
        ).map_err(|e| format!("Failed to favorite strategy: {}", e))?;
        
        Ok(())
    }
    
    /// 取消收藏策略
    pub fn unfavorite_strategy(
        &self,
        user_id: i64,
        strategy_id: i64,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        conn.execute(
            "DELETE FROM strategy_favorites WHERE user_id = ? AND strategy_id = ?",
            rusqlite::params![user_id, strategy_id],
        ).map_err(|e| format!("Failed to unfavorite strategy: {}", e))?;
        
        Ok(())
    }
    
    /// 获取用户收藏的策略
    pub fn get_user_favorite_strategies(
        &self,
        user_id: i64,
    ) -> Result<Vec<Strategy>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT s.id, s.user_id, s.name, s.description, s.strategy_type, s.parameters,
                    s.is_public, s.is_active, s.created_at, s.updated_at
             FROM strategies s
             JOIN strategy_favorites f ON s.id = f.strategy_id
             WHERE f.user_id = ? AND s.is_active = 1
             ORDER BY f.created_at DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let strategy_iter = stmt.query_map(
            rusqlite::params![user_id],
            |row| {
                Ok(Strategy {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    strategy_type: StrategyType::from_str(&row.get::<_, String>(4)?),
                    parameters: row.get(5)?,
                    is_public: row.get(6)?,
                    is_active: row.get(7)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(8)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(9)?, 0),
                })
            },
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut strategies = Vec::new();
        for strategy_result in strategy_iter {
            match strategy_result {
                Ok(strategy) => strategies.push(strategy),
                Err(e) => return Err(format!("Failed to process strategy row: {}", e)),
            }
        }
        
        Ok(strategies)
    }
    
    /// 对策略进行评分
    pub fn rate_strategy(
        &self,
        user_id: i64,
        strategy_id: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<(), String> {
        if rating < 1 || rating > 5 {
            return Err("Rating must be between 1 and 5".to_string());
        }
        
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let now = Utc::now().timestamp();
        
        conn.execute(
            "INSERT OR REPLACE INTO strategy_ratings (
                user_id, strategy_id, rating, comment, created_at, updated_at
             ) VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![user_id, strategy_id, rating, comment, now, now],
        ).map_err(|e| format!("Failed to rate strategy: {}", e))?;
        
        Ok(())
    }
    
    /// 获取策略评分
    pub fn get_strategy_ratings(
        &self,
        strategy_id: i64,
    ) -> Result<Vec<StrategyRating>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT id, user_id, strategy_id, rating, comment, created_at, updated_at
             FROM strategy_ratings
             WHERE strategy_id = ?
             ORDER BY updated_at DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let rating_iter = stmt.query_map(
            rusqlite::params![strategy_id],
            |row| {
                Ok(StrategyRating {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    strategy_id: row.get(2)?,
                    rating: row.get(3)?,
                    comment: row.get(4)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(5)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(6)?, 0),
                })
            },
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut ratings = Vec::new();
        for rating_result in rating_iter {
            match rating_result {
                Ok(rating) => ratings.push(rating),
                Err(e) => return Err(format!("Failed to process rating row: {}", e)),
            }
        }
        
        Ok(ratings)
    }
    
    /// 获取策略平均评分
    pub fn get_strategy_average_rating(
        &self,
        strategy_id: i64,
    ) -> Result<Option<f64>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let avg_rating: Result<f64, rusqlite::Error> = conn.query_row(
            "SELECT AVG(rating) FROM strategy_ratings WHERE strategy_id = ?",
            rusqlite::params![strategy_id],
            |row| row.get(0),
        );
        
        match avg_rating {
            Ok(rating) => Ok(Some(rating)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to get average rating: {}", e)),
        }
    }
    
    /// 应用策略到资产
    pub fn apply_strategy_to_asset(
        &self,
        user_id: i64,
        strategy_id: i64,
        asset_id: i64,
    ) -> Result<i64, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let now = Utc::now().timestamp();
        
        // 检查策略是否存在且属于该用户或是公开的
        let strategy = self.get_strategy(strategy_id)?;
        if strategy.user_id != user_id && !strategy.is_public {
            return Err("You don't have permission to use this strategy".to_string());
        }
        
        // 检查资产是否存在且属于该用户
        let asset_exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM assets WHERE id = ? AND user_id = ?)",
            rusqlite::params![asset_id, user_id],
            |row| row.get(0),
        ).map_err(|e| format!("Failed to check asset: {}", e))?;
        
        if !asset_exists {
            return Err("Asset not found or you don't have permission".to_string());
        }
        
        // 插入或更新应用记录
        conn.execute(
            "INSERT INTO strategy_applications (
                user_id, strategy_id, asset_id, is_active, created_at, updated_at
             ) VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(strategy_id, asset_id) DO UPDATE SET
                is_active = excluded.is_active,
                updated_at = excluded.updated_at",
            rusqlite::params![user_id, strategy_id, asset_id, true, now, now],
        ).map_err(|e| format!("Failed to apply strategy: {}", e))?;
        
        let application_id = conn.last_insert_rowid();
        
        Ok(application_id)
    }
    
    /// 停用应用到资产的策略
    pub fn deactivate_strategy_application(
        &self,
        user_id: i64,
        application_id: i64,
    ) -> Result<(), String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let now = Utc::now().timestamp();
        
        // 更新应用状态
        let rows_affected = conn.execute(
            "UPDATE strategy_applications
             SET is_active = 0, updated_at = ?
             WHERE id = ? AND user_id = ?",
            rusqlite::params![now, application_id, user_id],
        ).map_err(|e| format!("Failed to deactivate strategy application: {}", e))?;
        
        if rows_affected == 0 {
            return Err("Strategy application not found or you don't have permission".to_string());
        }
        
        Ok(())
    }
    
    /// 获取资产应用的策略
    pub fn get_asset_strategies(
        &self,
        user_id: i64,
        asset_id: i64,
    ) -> Result<Vec<StrategyApplication>, String> {
        let conn = get_connection_from_pool()
            .map_err(|e| format!("Failed to get database connection: {}", e))?;
        
        let mut stmt = conn.prepare(
            "SELECT a.id, a.user_id, a.strategy_id, a.asset_id, a.is_active, a.created_at, a.updated_at
             FROM strategy_applications a
             WHERE a.user_id = ? AND a.asset_id = ?
             ORDER BY a.updated_at DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let application_iter = stmt.query_map(
            rusqlite::params![user_id, asset_id],
            |row| {
                Ok(StrategyApplication {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    strategy_id: row.get(2)?,
                    asset_id: row.get(3)?,
                    is_active: row.get(4)?,
                    created_at: Utc.timestamp(row.get::<_, i64>(5)?, 0),
                    updated_at: Utc.timestamp(row.get::<_, i64>(6)?, 0),
                })
            },
        ).map_err(|e| format!("Failed to execute query: {}", e))?;
        
        let mut applications = Vec::new();
        for application_result in application_iter {
            match application_result {
                Ok(application) => applications.push(application),
                Err(e) => return Err(format!("Failed to process application row: {}", e)),
            }
        }
        
        Ok(applications)
    }
    
    /// 创建策略实例
    pub fn create_strategy_instance(
        &self,
        strategy_type: StrategyType,
        parameters_json: &str,
    ) -> Result<Box<dyn crate::models::strategy::Strategy>, String> {
        // 解析参数
        let parameters: Value = serde_json::from_str(parameters_json)
            .map_err(|e| format!("Failed to parse strategy parameters: {}", e))?;
        
        // 根据策略类型创建实例
        match strategy_type {
            StrategyType::MovingAverageCrossover => {
                // 创建移动平均线交叉策略
                let fast_period = parameters["fastPeriod"].as_u64()
                    .ok_or_else(|| "Missing fastPeriod parameter".to_string())? as usize;
                let slow_period = parameters["slowPeriod"].as_u64()
                    .ok_or_else(|| "Missing slowPeriod parameter".to_string())? as usize;
                
                // 这里应该返回具体的策略实现
                // 为了示例，我们返回一个空实现
                Ok(Box::new(DummyStrategy::new(
                    "Moving Average Crossover",
                    Some("A strategy based on moving average crossovers"),
                    parameters,
                )))
            },
            StrategyType::BollingerBands => {
                // 创建布林带策略
                let period = parameters["period"].as_u64()
                    .ok_or_else(|| "Missing period parameter".to_string())? as usize;
                let std_dev = parameters["stdDev"].as_f64()
                    .ok_or_else(|| "Missing stdDev parameter".to_string())?;
                
                Ok(Box::new(DummyStrategy::new(
                    "Bollinger Bands",
                    Some("A strategy based on Bollinger Bands"),
                    parameters,
                )))
            },
            StrategyType::RSI => {
                // 创建RSI策略
                let period = parameters["period"].as_u64()
                    .ok_or_else(|| "Missing period parameter".to_string())? as usize;
                let overbought = parameters["overbought"].as_f64()
                    .ok_or_else(|| "Missing overbought parameter".to_string())?;
                let oversold = parameters["oversold"].as_f64()
                    .ok_or_else(|| "Missing oversold parameter".to_string())?;
                
                Ok(Box::new(DummyStrategy::new(
                    "RSI",
                    Some("A strategy based on Relative Strength Index"),
                    parameters,
                )))
            },
            StrategyType::MACD => {
                // 创建MACD策略
                let fast_period = parameters["fastPeriod"].as_u64()
                    .ok_or_else(|| "Missing fastPeriod parameter".to_string())? as usize;
                let slow_period = parameters["slowPeriod"].as_u64()
                    .ok_or_else(|| "Missing slowPeriod parameter".to_string())? as usize;
                let signal_period = parameters["signalPeriod"].as_u64()
                    .ok_or_else(|| "Missing signalPeriod parameter".to_string())? as usize;
                
                Ok(Box::new(DummyStrategy::new(
                    "MACD",
                    Some("A strategy based on Moving Average Convergence Divergence"),
                    parameters,
                )))
            },
            StrategyType::Custom => {
                // 创建自定义策略
                let script = parameters["script"].as_str()
                    .ok_or_else(|| "Missing script parameter".to_string())?;
                
                Ok(Box::new(DummyStrategy::new(
                    "Custom",
                    Some("A custom strategy with user-defined logic"),
                    parameters,
                )))
            },
        }
    }
    
    /// 验证策略参数
    fn validate_strategy_parameters(
        &self,
        strategy_type: &str,
        parameters_json: &str,
    ) -> Result<(), String> {
        // 解析参数
        let parameters: Value = serde_json::from_str(parameters_json)
            .map_err(|e| format!("Invalid JSON parameters: {}", e))?;
        
        // 根据策略类型验证参数
        match strategy_type {
            "MovingAverageCrossover" => {
                // 验证移动平均线交叉策略参数
                let fast_period = parameters["fastPeriod"].as_u64()
                    .ok_or_else(|| "Missing fastPeriod parameter".to_string())?;
                let slow_period = parameters["slowPeriod"].as_u64()
                    .ok_or_else(|| "Missing slowPeriod parameter".to_string())?;
                
                if fast_period >= slow_period {
                    return Err("fastPeriod must be less than slowPeriod".to_string());
                }
                
                if fast_period < 1 {
                    return Err("fastPeriod must be at least 1".to_string());
                }
                
                if slow_period < 2 {
                    return Err("slowPeriod must be at least 2".to_string());
                }
            },
            "BollingerBands" => {
                // 验证布林带策略参数
                let period = parameters["period"].as_u64()
                    .ok_or_else(|| "Missing period parameter".to_string())?;
                let std_dev = parameters["stdDev"].as_f64()
                    .ok_or_else(|| "Missing stdDev parameter".to_string())?;
                
                if period < 2 {
                    return Err("period must be at least 2".to_string());
                }
                
                if std_dev <= 0.0 {
                    return Err("stdDev must be positive".to_string());
                }
            },
            "RSI" => {
                // 验证RSI策略参数
                let period = parameters["period"].as_u64()
                    .ok_or_else(|| "Missing period parameter".to_string())?;
                let overbought = parameters["overbought"].as_f64()
                    .ok_or_else(|| "Missing overbought parameter".to_string())?;
                let oversold = parameters["oversold"].as_f64()
                    .ok_or_else(|| "Missing oversold parameter".to_string())?;
                
                if period < 2 {
                    return Err("period must be at least 2".to_string());
                }
                
                if overbought <= oversold {
                    return Err("overbought must be greater than oversold".to_string());
                }
                
                if oversold < 0.0 || oversold > 100.0 {
                    return Err("oversold must be between 0 and 100".to_string());
                }
                
                if overbought < 0.0 || overbought > 100.0 {
                    return Err("overbought must be between 0 and 100".to_string());
                }
            },
            "MACD" => {
                // 验证MACD策略参数
                let fast_period = parameters["fastPeriod"].as_u64()
                    .ok_or_else(|| "Missing fastPeriod parameter".to_string())?;
                let slow_period = parameters["slowPeriod"].as_u64()
                    .ok_or_else(|| "Missing slowPeriod parameter".to_string())?;
                let signal_period = parameters["signalPeriod"].as_u64()
                    .ok_or_else(|| "Missing signalPeriod parameter".to_string())?;
                
                if fast_period >= slow_period {
                    return Err("fastPeriod must be less than slowPeriod".to_string());
                }
                
                if fast_period < 1 {
                    return Err("fastPeriod must be at least 1".to_string());
                }
                
                if slow_period < 2 {
                    return Err("slowPeriod must be at least 2".to_string());
                }
                
                if signal_period < 1 {
                    return Err("signalPeriod must be at least 1".to_string());
                }
            },
            "Custom" => {
                // 验证自定义策略参数
                let script = parameters["script"].as_str()
                    .ok_or_else(|| "Missing script parameter".to_string())?;
                
                if script.trim().is_empty() {
                    return Err("script cannot be empty".to_string());
                }
            },
            _ => {
                return Err(format!("Unknown strategy type: {}", strategy_type));
            }
        }
        
        Ok(())
    }
}

/// 示例策略实现（仅用于演示）
struct DummyStrategy {
    name: String,
    description: Option<String>,
    params: Value,
}

impl DummyStrategy {
    fn new(name: &str, description: Option<&str>, params: Value) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            params,
        }
    }
}

impl crate::models::strategy::Strategy for DummyStrategy {
    fn init(&self) -> Result<(), String> {
        Ok(())
    }
    
    fn update(&self, _candle: &Candle) -> Result<(), String> {
        Ok(())
    }
    
    fn check_signal(&self, _candle: &Candle) -> Result<Option<OrderSignal>, String> {
        Ok(None)
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    
    fn parameters(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        if let Some(obj) = self.params.as_object() {
            for (k, v) in obj {
                map.insert(k.clone(), v.clone());
            }
        }
        map
    }
}
