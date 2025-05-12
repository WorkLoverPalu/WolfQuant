/**
 * 资产管理功能
 */
use crate::database::get_db_connection;
use crate::error::auth::AuthError;
use crate::models::{Asset, AssetType, UserGroup};
use chrono::Utc;
use log::{error, info};
use rusqlite::{params, Connection};

pub fn get_asset_types() -> Result<Vec<AssetType>, AuthError> {
    let conn = get_db_connection()?;
    
    let mut stmt = conn.prepare(
        "SELECT id, name, description FROM asset_types ORDER BY id"
    )?;
    
    let asset_types = stmt.query_map([], |row| {
        Ok(AssetType {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| {
        error!("Failed to fetch asset types: {}", e);
        AuthError::DatabaseError(format!("获取资产类型失败: {}", e))
    })?;
    
    Ok(asset_types)
}

pub fn create_user_group(
    user_id: &str,
    name: &str,
    asset_type_id: i64,
    description: Option<&str>,
) -> Result<UserGroup, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 检查资产类型是否存在
    let asset_type_exists: bool = conn.query_row(
        "SELECT 1 FROM asset_types WHERE id = ?1",
        params![asset_type_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !asset_type_exists {
        return Err(AuthError::InvalidCredentials("资产类型不存在".to_string()));
    }
    
    // 检查是否已存在同名分组
    let group_exists: bool = conn.query_row(
        "SELECT 1 FROM user_groups WHERE user_id = ?1 AND name = ?2 AND asset_type_id = ?3",
        params![user_id, name, asset_type_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if group_exists {
        return Err(AuthError::InvalidCredentials("已存在同名分组".to_string()));
    }
    
    // 创建分组
    conn.execute(
        "INSERT INTO user_groups (user_id, name, asset_type_id, description, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            user_id,
            name,
            asset_type_id,
            description,
            now,
            now
        ],
    )?;
    
    let group_id = conn.last_insert_rowid();
    
    // 获取资产类型名称
    let asset_type_name: String = conn.query_row(
        "SELECT name FROM asset_types WHERE id = ?1",
        params![asset_type_id],
        |row| row.get(0),
    )?;
    
    let group = UserGroup {
        id: group_id,
        user_id: user_id.to_string(),
        name: name.to_string(),
        asset_type_id,
        asset_type_name,
        description: description.map(|s| s.to_string()),
        created_at: now,
        updated_at: now,
    };
    
    info!("User group created: {} for user: {}", name, user_id);
    Ok(group)
}

pub fn update_user_group(
    id: i64,
    user_id: &str,
    name: &str,
    description: Option<&str>,
) -> Result<UserGroup, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 检查分组是否存在且属于该用户
    let group_exists: bool = conn.query_row(
        "SELECT 1 FROM user_groups WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !group_exists {
        return Err(AuthError::InvalidCredentials("分组不存在或无权限".to_string()));
    }
    
    // 检查是否已存在同名分组
    let asset_type_id: i64 = conn.query_row(
        "SELECT asset_type_id FROM user_groups WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    
    let same_name_exists: bool = conn.query_row(
        "SELECT 1 FROM user_groups 
         WHERE user_id = ?1 AND name = ?2 AND asset_type_id = ?3 AND id != ?4",
        params![user_id, name, asset_type_id, id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if same_name_exists {
        return Err(AuthError::InvalidCredentials("已存在同名分组".to_string()));
    }
    
    // 更新分组
    conn.execute(
        "UPDATE user_groups SET name = ?1, description = ?2, updated_at = ?3 WHERE id = ?4",
        params![name, description, now, id],
    )?;
    
    // 获取资产类型名称
    let asset_type_name: String = conn.query_row(
        "SELECT name FROM asset_types WHERE id = ?1",
        params![asset_type_id],
        |row| row.get(0),
    )?;
    
    let group = UserGroup {
        id,
        user_id: user_id.to_string(),
        name: name.to_string(),
        asset_type_id,
        asset_type_name,
        description: description.map(|s| s.to_string()),
        created_at: conn.query_row(
            "SELECT created_at FROM user_groups WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?,
        updated_at: now,
    };
    
    info!("User group updated: {} for user: {}", name, user_id);
    Ok(group)
}

pub fn delete_user_group(id: i64, user_id: &str) -> Result<(), AuthError> {
    let conn = get_db_connection()?;
    
    // 检查分组是否存在且属于该用户
    let group_exists: bool = conn.query_row(
        "SELECT 1 FROM user_groups WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !group_exists {
        return Err(AuthError::InvalidCredentials("分组不存在或无权限".to_string()));
    }
    
    // 将该分组下的资产移出分组
    conn.execute(
        "UPDATE assets SET group_id = NULL, updated_at = ?1 WHERE group_id = ?2",
        params![Utc::now().timestamp(), id],
    )?;
    
    // 删除分组
    conn.execute(
        "DELETE FROM user_groups WHERE id = ?1",
        params![id],
    )?;
    
    info!("User group deleted: {} for user: {}", id, user_id);
    Ok(())
}

pub fn get_user_groups(user_id: &str, asset_type_id: Option<i64>) -> Result<Vec<UserGroup>, AuthError> {
    let conn = get_db_connection()?;
    
    let mut query = match asset_type_id {
        Some(type_id) => {
            conn.prepare(
                "SELECT g.id, g.user_id, g.name, g.asset_type_id, t.name, g.description, g.created_at, g.updated_at
                 FROM user_groups g
                 JOIN asset_types t ON g.asset_type_id = t.id
                 WHERE g.user_id = ?1 AND g.asset_type_id = ?2
                 ORDER BY g.name"
            )?
        },
        None => {
            conn.prepare(
                "SELECT g.id, g.user_id, g.name, g.asset_type_id, t.name, g.description, g.created_at, g.updated_at
                 FROM user_groups g
                 JOIN asset_types t ON g.asset_type_id = t.id
                 WHERE g.user_id = ?1
                 ORDER BY g.name"
            )?
        }
    };
    
    let groups = match asset_type_id {
        Some(type_id) => {
            query.query_map(params![user_id, type_id], |row| {
                Ok(UserGroup {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    asset_type_id: row.get(3)?,
                    asset_type_name: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user groups: {}", e);
                AuthError::DatabaseError(format!("获取用户分组失败: {}", e))
            })?
        },
        None => {
            query.query_map(params![user_id], |row| {
                Ok(UserGroup {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    name: row.get(2)?,
                    asset_type_id: row.get(3)?,
                    asset_type_name: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user groups: {}", e);
                AuthError::DatabaseError(format!("获取用户分组失败: {}", e))
            })?
        }
    };
    
    Ok(groups)
}

pub fn create_asset(
    user_id: &str,
    group_id: Option<i64>,
    asset_type_id: i64,
    code: &str,
    name: &str,
    current_price: Option<f64>,
) -> Result<Asset, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 检查资产类型是否存在
    let asset_type_exists: bool = conn.query_row(
        "SELECT 1 FROM asset_types WHERE id = ?1",
        params![asset_type_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !asset_type_exists {
        return Err(AuthError::InvalidCredentials("资产类型不存在".to_string()));
    }
    
    // 如果指定了分组，检查分组是否存在且属于该用户
    if let Some(gid) = group_id {
        let group_valid: bool = conn.query_row(
            "SELECT 1 FROM user_groups 
             WHERE id = ?1 AND user_id = ?2 AND asset_type_id = ?3",
            params![gid, user_id, asset_type_id],
            |_| Ok(true),
        ).unwrap_or(false);
        
        if !group_valid {
            return Err(AuthError::InvalidCredentials("分组不存在、无权限或资产类型不匹配".to_string()));
        }
    }
    
    // 检查是否已存在相同代码的资产
    let asset_exists: bool = conn.query_row(
        "SELECT 1 FROM assets 
         WHERE user_id = ?1 AND asset_type_id = ?2 AND code = ?3",
        params![user_id, asset_type_id, code],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if asset_exists {
        return Err(AuthError::InvalidCredentials("已存在相同代码的资产".to_string()));
    }
    
    // 创建资产
    conn.execute(
        "INSERT INTO assets (user_id, group_id, asset_type_id, code, name, current_price, last_updated, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            user_id,
            group_id,
            asset_type_id,
            code,
            name,
            current_price,
            current_price.map(|_| now),
            now,
            now
        ],
    )?;
    
    let asset_id = conn.last_insert_rowid();
    
    // 获取资产类型名称
    let asset_type_name: String = conn.query_row(
        "SELECT name FROM asset_types WHERE id = ?1",
        params![asset_type_id],
        |row| row.get(0),
    )?;
    
    // 获取分组名称
    let group_name: Option<String> = if let Some(gid) = group_id {
        conn.query_row(
            "SELECT name FROM user_groups WHERE id = ?1",
            params![gid],
            |row| row.get(0),
        ).ok()
    } else {
        None
    };
    
    let asset = Asset {
        id: asset_id,
        user_id: user_id.to_string(),
        group_id,
        group_name,
        asset_type_id,
        asset_type_name,
        code: code.to_string(),
        name: name.to_string(),
        current_price,
        last_updated: current_price.map(|_| now),
        created_at: now,
        updated_at: now,
        total_amount: None,
        total_cost: None,
        daily_change: None,
        daily_change_percent: None,
        total_profit: None,
        total_profit_percent: None,
    };
    
    info!("Asset created: {} ({}) for user: {}", name, code, user_id);
    Ok(asset)
}

pub fn update_asset(
    id: i64,
    user_id: &str,
    group_id: Option<i64>,
    name: &str,
    current_price: Option<f64>,
) -> Result<Asset, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 检查资产是否存在且属于该用户
    let asset_exists: bool = conn.query_row(
        "SELECT 1 FROM assets WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !asset_exists {
        return Err(AuthError::InvalidCredentials("资产不存在或无权限".to_string()));
    }
    
    // 获取资产类型
    let asset_type_id: i64 = conn.query_row(
        "SELECT asset_type_id FROM assets WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    
    // 如果指定了分组，检查分组是否存在且属于该用户
    if let Some(gid) = group_id {
        let group_valid: bool = conn.query_row(
            "SELECT 1 FROM user_groups 
             WHERE id = ?1 AND user_id = ?2 AND asset_type_id = ?3",
            params![gid, user_id, asset_type_id],
            |_| Ok(true),
        ).unwrap_or(false);
        
        if !group_valid {
            return Err(AuthError::InvalidCredentials("分组不存在、无权限或资产类型不匹配".to_string()));
        }
    }
    
    // 更新资产
    conn.execute(
        "UPDATE assets 
         SET group_id = ?1, name = ?2, current_price = ?3, last_updated = ?4, updated_at = ?5 
         WHERE id = ?6",
        params![
            group_id,
            name,
            current_price,
            current_price.map(|_| now),
            now,
            id
        ],
    )?;
    
    // 获取资产信息
    let code: String = conn.query_row(
        "SELECT code FROM assets WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    
    // 获取资产类型名称
    let asset_type_name: String = conn.query_row(
        "SELECT name FROM asset_types WHERE id = ?1",
        params![asset_type_id],
        |row| row.get(0),
    )?;
    
    // 获取分组名称
    let group_name: Option<String> = if let Some(gid) = group_id {
        conn.query_row(
            "SELECT name FROM user_groups WHERE id = ?1",
            params![gid],
            |row| row.get(0),
        ).ok()
    } else {
        None
    };
    
    let created_at: i64 = conn.query_row(
        "SELECT created_at FROM assets WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    
    let asset = Asset {
        id,
        user_id: user_id.to_string(),
        group_id,
        group_name,
        asset_type_id,
        asset_type_name,
        code,
        name: name.to_string(),
        current_price,
        last_updated: current_price.map(|_| now),
        created_at,
        updated_at: now,
        total_amount: None,
        total_cost: None,
        daily_change: None,
        daily_change_percent: None,
        total_profit: None,
        total_profit_percent: None,
    };
    
    info!("Asset updated: {} for user: {}", name, user_id);
    Ok(asset)
}

pub fn delete_asset(id: i64, user_id: &str) -> Result<(), AuthError> {
    let mut conn = get_db_connection()?;
    
    // 检查资产是否存在且属于该用户
    let asset_exists: bool = conn.query_row(
        "SELECT 1 FROM assets WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !asset_exists {
        return Err(AuthError::InvalidCredentials("资产不存在或无权限".to_string()));
    }
    
    // 开始事务
    let tx = conn.transaction()?;
    
    // 删除相关的交易记录
    tx.execute(
        "DELETE FROM transactions WHERE asset_id = ?1",
        params![id],
    )?;
    
    // 删除相关的定投计划
    tx.execute(
        "DELETE FROM investment_plans WHERE asset_id = ?1",
        params![id],
    )?;
    
    // 删除相关的策略应用
    tx.execute(
        "DELETE FROM strategy_applications WHERE asset_id = ?1",
        params![id],
    )?;
    
    // 删除相关的历史价格
    tx.execute(
        "DELETE FROM price_history WHERE asset_id = ?1",
        params![id],
    )?;
    
    // 删除相关的交易提醒
    tx.execute(
        "DELETE FROM trade_alerts WHERE asset_id = ?1",
        params![id],
    )?;
    
    // 删除资产
    tx.execute(
        "DELETE FROM assets WHERE id = ?1",
        params![id],
    )?;
    
    // 提交事务
    tx.commit()?;
    
    info!("Asset deleted: {} for user: {}", id, user_id);
    Ok(())
}

pub fn get_user_assets(
    user_id: &str,
    asset_type_id: Option<i64>,
    group_id: Option<i64>,
) -> Result<Vec<Asset>, AuthError> {
    let conn = get_db_connection()?;
    
    // 构建查询条件
    let mut conditions = vec!["a.user_id = ?1".to_string()];
    let mut params: Vec<&dyn rusqlite::ToSql> = vec![&user_id];
    let mut boxed_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(type_id) = asset_type_id {
        conditions.push("a.asset_type_id = ?".to_string());
        let type_id_box = Box::new(type_id);
        boxed_params.push(type_id_box);
        params.push(boxed_params.last().unwrap().as_ref());
    }
    
    if let Some(g_id) = group_id {
        conditions.push("a.group_id = ?".to_string());
        let g_id_box = Box::new(g_id);
        boxed_params.push(g_id_box);
        params.push(boxed_params.last().unwrap().as_ref());
    }
    
    let condition_str = conditions.join(" AND ");
    
    // 构建查询语句
    let query = format!(
        "SELECT 
            a.id, a.user_id, a.group_id, g.name, a.asset_type_id, t.name, 
            a.code, a.name, a.current_price, a.last_updated, a.created_at, a.updated_at,
            (SELECT SUM(CASE WHEN transaction_type = 'BUY' THEN amount ELSE -amount END) 
             FROM transactions WHERE asset_id = a.id) as total_amount,
            (SELECT SUM(CASE WHEN transaction_type = 'BUY' THEN total_cost ELSE -total_cost END) 
             FROM transactions WHERE asset_id = a.id) as total_cost
         FROM assets a
         JOIN asset_types t ON a.asset_type_id = t.id
         LEFT JOIN user_groups g ON a.group_id = g.id
         WHERE {}
         ORDER BY a.name", condition_str);
    
    let mut stmt = conn.prepare(&query)?;
    
    let assets = stmt.query_map(rusqlite::params_from_iter(params), |row| {
        let id: i64 = row.get(0)?;
        let current_price: Option<f64> = row.get(8)?;
        let total_amount: Option<f64> = row.get(12)?;
        let total_cost: Option<f64> = row.get(13)?;
        
        // 计算盈亏
        let (daily_change, daily_change_percent, total_profit, total_profit_percent) = 
            calculate_asset_performance(&conn, id, current_price, total_amount, total_cost)?;
        
        Ok(Asset {
            id,
            user_id: row.get(1)?,
            group_id: row.get(2)?,
            group_name: row.get(3)?,
            asset_type_id: row.get(4)?,
            asset_type_name: row.get(5)?,
            code: row.get(6)?,
            name: row.get(7)?,
            current_price,
            last_updated: row.get(9)?,
            created_at: row.get(10)?,
            updated_at: row.get(11)?,
            total_amount,
            total_cost,
            daily_change,
            daily_change_percent,
            total_profit,
            total_profit_percent,
        })
    })?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| {
        error!("Failed to fetch user assets: {}", e);
        AuthError::DatabaseError(format!("获取用户资产失败: {}", e))
    })?;
    
    Ok(assets)
}

fn calculate_asset_performance(
    conn: &Connection,
    asset_id: i64,
    current_price: Option<f64>,
    total_amount: Option<f64>,
    total_cost: Option<f64>,
) -> Result<(Option<f64>, Option<f64>, Option<f64>, Option<f64>), rusqlite::Error> {
    // 如果没有价格或持仓数量，则无法计算
    if current_price.is_none() || total_amount.is_none() || total_cost.is_none() {
        return Ok((None, None, None, None));
    }
    
    let price = current_price.unwrap();
    let amount = total_amount.unwrap();
    let cost = total_cost.unwrap();
    
    // 如果没有持仓，则无法计算
    if amount <= 0.0 {
        return Ok((None, None, None, None));
    }
    
    // 获取昨日收盘价
    let yesterday = chrono::Utc::now().date_naive().pred().and_hms_opt(0, 0, 0).unwrap().timestamp();
    
    let yesterday_price: Option<f64> = conn.query_row(
        "SELECT close_price FROM price_history 
         WHERE asset_id = ?1 AND date <= ?2 
         ORDER BY date DESC LIMIT 1",
        params![asset_id, yesterday],
        |row| row.get(0),
    ).ok();
    
    // 计算日涨跌幅
    let (daily_change, daily_change_percent) = if let Some(prev_price) = yesterday_price {
        let change = price - prev_price;
        let change_percent = if prev_price > 0.0 { change / prev_price * 100.0 } else { 0.0 };
        (Some(change * amount), Some(change_percent))
    } else {
        (None, None)
    };
    
    // 计算总盈亏
    let current_value = price * amount;
    let total_profit = current_value - cost;
    let total_profit_percent = if cost > 0.0 { total_profit / cost * 100.0 } else { 0.0 };
    
    Ok((
        daily_change,
        daily_change_percent,
        Some(total_profit),
        Some(total_profit_percent),
    ))
}