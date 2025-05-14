/**
 * 数据更新模块
 */
use crate::database::get_connection_from_pool;
use crate::error::auth::AuthError;
use crate::models::{AssetSummary, PortfolioSummary, PriceHistory, TradeAlert};
use chrono::{Duration, NaiveDate, Utc};
use log::{error, info};
use rusqlite::params;
use serde_json::Value;
use std::collections::HashMap;

pub fn update_asset_price(asset_id: i64, price: f64, date: i64) -> Result<(), AuthError> {
    let mut conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 更新资产当前价格
    conn.execute(
        "UPDATE assets SET current_price = ?1, last_updated = ?2, updated_at = ?3 WHERE id = ?4",
        params![price, now, now, asset_id],
    )?;

    // 检查是否已存在该日期的价格记录
    let price_exists: bool = conn
        .query_row(
            "SELECT 1 FROM price_history WHERE asset_id = ?1 AND date = ?2",
            params![asset_id, date],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if price_exists {
        // 更新价格记录
        conn.execute(
            "UPDATE price_history SET close_price = ?1 WHERE asset_id = ?2 AND date = ?3",
            params![price, asset_id, date],
        )?;
    } else {
        // 创建新价格记录
        conn.execute(
            "INSERT INTO price_history (asset_id, date, close_price, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![asset_id, date, price, now],
        )?;
    }

    info!("Asset price updated: {} at {}", asset_id, date);
    Ok(())
}

pub fn update_asset_price_batch(asset_prices: &[(i64, f64, i64)]) -> Result<usize, AuthError> {
    let mut conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 开始事务
    let tx = conn.transaction()?;

    let mut updated_count = 0;

    for (asset_id, price, date) in asset_prices {
        // 更新资产当前价格
        tx.execute(
            "UPDATE assets SET current_price = ?1, last_updated = ?2, updated_at = ?3 WHERE id = ?4",
            params![price, now, now, asset_id],
        )?;

        // 检查是否已存在该日期的价格记录
        let price_exists: bool = tx
            .query_row(
                "SELECT 1 FROM price_history WHERE asset_id = ?1 AND date = ?2",
                params![asset_id, date],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if price_exists {
            // 更新价格记录
            tx.execute(
                "UPDATE price_history SET close_price = ?1 WHERE asset_id = ?2 AND date = ?3",
                params![price, asset_id, date],
            )?;
        } else {
            // 创建新价格记录
            tx.execute(
                "INSERT INTO price_history (asset_id, date, close_price, created_at)
                 VALUES (?1, ?2, ?3, ?4)",
                params![asset_id, date, price, now],
            )?;
        }

        updated_count += 1;
    }

    // 提交事务
    tx.commit()?;

    info!("Batch updated {} asset prices", updated_count);
    Ok(updated_count)
}

pub fn get_asset_price_history(
    asset_id: i64,
    start_date: Option<i64>,
    end_date: Option<i64>,
) -> Result<Vec<PriceHistory>, AuthError> {
    let mut conn = get_connection_from_pool()?;

    let mut query = match (start_date, end_date) {
        (Some(s_date), Some(e_date)) => {
            conn.prepare(
                "SELECT id, asset_id, date, open_price, close_price, high_price, low_price, volume, created_at
                 FROM price_history
                 WHERE asset_id = ?1 AND date >= ?2 AND date <= ?3
                 ORDER BY date"
            )?
        },
        (Some(s_date), None) => {
            conn.prepare(
                "SELECT id, asset_id, date, open_price, close_price, high_price, low_price, volume, created_at
                 FROM price_history
                 WHERE asset_id = ?1 AND date >= ?2
                 ORDER BY date"
            )?
        },
        (None, Some(e_date)) => {
            conn.prepare(
                "SELECT id, asset_id, date, open_price, close_price, high_price, low_price, volume, created_at
                 FROM price_history
                 WHERE asset_id = ?1 AND date <= ?2
                 ORDER BY date"
            )?
        },
        (None, None) => {
            conn.prepare(
                "SELECT id, asset_id, date, open_price, close_price, high_price, low_price, volume, created_at
                 FROM price_history
                 WHERE asset_id = ?1
                 ORDER BY date"
            )?
        }
    };

    let price_history = match (start_date, end_date) {
        (Some(s_date), Some(e_date)) => query
            .query_map(params![asset_id, s_date, e_date], |row| {
                Ok(PriceHistory {
                    id: row.get(0)?,
                    asset_id: row.get(1)?,
                    date: row.get(2)?,
                    open_price: row.get(3)?,
                    close_price: row.get(4)?,
                    high_price: row.get(5)?,
                    low_price: row.get(6)?,
                    volume: row.get(7)?,
                    created_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch asset price history: {}", e);
                AuthError::DatabaseError(format!("获取资产价格历史失败: {}", e))
            })?,
        (Some(s_date), None) => query
            .query_map(params![asset_id, s_date], |row| {
                Ok(PriceHistory {
                    id: row.get(0)?,
                    asset_id: row.get(1)?,
                    date: row.get(2)?,
                    open_price: row.get(3)?,
                    close_price: row.get(4)?,
                    high_price: row.get(5)?,
                    low_price: row.get(6)?,
                    volume: row.get(7)?,
                    created_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch asset price history: {}", e);
                AuthError::DatabaseError(format!("获取资产价格历史失败: {}", e))
            })?,
        (None, Some(e_date)) => query
            .query_map(params![asset_id, e_date], |row| {
                Ok(PriceHistory {
                    id: row.get(0)?,
                    asset_id: row.get(1)?,
                    date: row.get(2)?,
                    open_price: row.get(3)?,
                    close_price: row.get(4)?,
                    high_price: row.get(5)?,
                    low_price: row.get(6)?,
                    volume: row.get(7)?,
                    created_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch asset price history: {}", e);
                AuthError::DatabaseError(format!("获取资产价格历史失败: {}", e))
            })?,
        (None, None) => query
            .query_map(params![asset_id], |row| {
                Ok(PriceHistory {
                    id: row.get(0)?,
                    asset_id: row.get(1)?,
                    date: row.get(2)?,
                    open_price: row.get(3)?,
                    close_price: row.get(4)?,
                    high_price: row.get(5)?,
                    low_price: row.get(6)?,
                    volume: row.get(7)?,
                    created_at: row.get(8)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch asset price history: {}", e);
                AuthError::DatabaseError(format!("获取资产价格历史失败: {}", e))
            })?,
    };

    Ok(price_history)
}

pub fn create_trade_alert(
    user_id: i64,
    asset_id: i64,
    strategy_id: Option<i64>,
    alert_type: &str,
    message: &str,
) -> Result<TradeAlert, AuthError> {
    let conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 检查资产是否存在且属于该用户
    let asset_exists: bool = conn
        .query_row(
            "SELECT 1 FROM assets WHERE id = ?1 AND user_id = ?2",
            params![asset_id, user_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !asset_exists {
        return Err(AuthError::InvalidCredentials(
            "资产不存在或无权限".to_string(),
        ));
    }

    // 如果指定了策略，检查策略是否存在且属于该用户
    if let Some(s_id) = strategy_id {
        let strategy_exists: bool = conn
            .query_row(
                "SELECT 1 FROM investment_strategies WHERE id = ?1 AND user_id = ?2",
                params![s_id, user_id],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !strategy_exists {
            return Err(AuthError::InvalidCredentials(
                "策略不存在或无权限".to_string(),
            ));
        }
    }

    // 创建交易提醒
    conn.execute(
        "INSERT INTO trade_alerts (user_id, asset_id, strategy_id, alert_type, message, is_read, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            user_id,
            asset_id,
            strategy_id,
            alert_type,
            message,
            false,
            now
        ],
    )?;

    let alert_id = conn.last_insert_rowid();

    // 获取资产信息
    let (asset_name, asset_code): (String, String) = conn.query_row(
        "SELECT name, code FROM assets WHERE id = ?1",
        params![asset_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    // 获取策略信息
    let strategy_name = if let Some(s_id) = strategy_id {
        conn.query_row(
            "SELECT name FROM investment_strategies WHERE id = ?1",
            params![s_id],
            |row| row.get(0),
        )
        .ok()
    } else {
        None
    };

    let alert = TradeAlert {
        id: alert_id,
        user_id: user_id,
        asset_id,
        asset_name: asset_name.clone(),
        asset_code,
        strategy_id,
        strategy_name,
        alert_type: alert_type.to_string(),
        message: message.to_string(),
        is_read: false,
        created_at: now,
    };

    info!(
        "Trade alert created: {} for asset: {}",
        alert_type, asset_name
    );
    Ok(alert)
}

pub fn mark_alert_read(id: i64, user_id: i64) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;

    // 检查提醒是否存在且属于该用户
    let alert_exists: bool = conn
        .query_row(
            "SELECT 1 FROM trade_alerts WHERE id = ?1 AND user_id = ?2",
            params![id, user_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !alert_exists {
        return Err(AuthError::InvalidCredentials(
            "交易提醒不存在或无权限".to_string(),
        ));
    }

    // 标记为已读
    conn.execute(
        "UPDATE trade_alerts SET is_read = 1 WHERE id = ?1",
        params![id],
    )?;

    info!("Trade alert marked as read: {} for user: {}", id, user_id);
    Ok(())
}

pub fn get_user_trade_alerts(
    user_id: i64,
    is_read: Option<bool>,
    limit: Option<i64>,
) -> Result<Vec<TradeAlert>, AuthError> {
    let conn = get_connection_from_pool()?;

    let mut query = match (is_read, limit) {
        (Some(read), Some(lim)) => conn.prepare(
            "SELECT a.id, a.user_id, a.asset_id, ast.name, ast.code, a.strategy_id, 
                        s.name, a.alert_type, a.message, a.is_read, a.created_at
                 FROM trade_alerts a
                 JOIN assets ast ON a.asset_id = ast.id
                 LEFT JOIN investment_strategies s ON a.strategy_id = s.id
                 WHERE a.user_id = ?1 AND a.is_read = ?2
                 ORDER BY a.created_at DESC
                 LIMIT ?3",
        )?,
        (Some(read), None) => conn.prepare(
            "SELECT a.id, a.user_id, a.asset_id, ast.name, ast.code, a.strategy_id, 
                        s.name, a.alert_type, a.message, a.is_read, a.created_at
                 FROM trade_alerts a
                 JOIN assets ast ON a.asset_id = ast.id
                 LEFT JOIN investment_strategies s ON a.strategy_id = s.id
                 WHERE a.user_id = ?1 AND a.is_read = ?2
                 ORDER BY a.created_at DESC",
        )?,
        (None, Some(lim)) => conn.prepare(
            "SELECT a.id, a.user_id, a.asset_id, ast.name, ast.code, a.strategy_id, 
                        s.name, a.alert_type, a.message, a.is_read, a.created_at
                 FROM trade_alerts a
                 JOIN assets ast ON a.asset_id = ast.id
                 LEFT JOIN investment_strategies s ON a.strategy_id = s.id
                 WHERE a.user_id = ?1
                 ORDER BY a.created_at DESC
                 LIMIT ?2",
        )?,
        (None, None) => conn.prepare(
            "SELECT a.id, a.user_id, a.asset_id, ast.name, ast.code, a.strategy_id, 
                        s.name, a.alert_type, a.message, a.is_read, a.created_at
                 FROM trade_alerts a
                 JOIN assets ast ON a.asset_id = ast.id
                 LEFT JOIN investment_strategies s ON a.strategy_id = s.id
                 WHERE a.user_id = ?1
                 ORDER BY a.created_at DESC",
        )?,
    };

    let alerts = match (is_read, limit) {
        (Some(read), Some(lim)) => query
            .query_map(params![user_id, read, lim], |row| {
                Ok(TradeAlert {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    asset_id: row.get(2)?,
                    asset_name: row.get(3)?,
                    asset_code: row.get(4)?,
                    strategy_id: row.get(5)?,
                    strategy_name: row.get(6)?,
                    alert_type: row.get(7)?,
                    message: row.get(8)?,
                    is_read: row.get(9)?,
                    created_at: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user trade alerts: {}", e);
                AuthError::DatabaseError(format!("获取用户交易提醒失败: {}", e))
            })?,
        (Some(read), None) => query
            .query_map(params![user_id, read], |row| {
                Ok(TradeAlert {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    asset_id: row.get(2)?,
                    asset_name: row.get(3)?,
                    asset_code: row.get(4)?,
                    strategy_id: row.get(5)?,
                    strategy_name: row.get(6)?,
                    alert_type: row.get(7)?,
                    message: row.get(8)?,
                    is_read: row.get(9)?,
                    created_at: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user trade alerts: {}", e);
                AuthError::DatabaseError(format!("获取用户交易提醒失败: {}", e))
            })?,
        (None, Some(lim)) => query
            .query_map(params![user_id, lim], |row| {
                Ok(TradeAlert {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    asset_id: row.get(2)?,
                    asset_name: row.get(3)?,
                    asset_code: row.get(4)?,
                    strategy_id: row.get(5)?,
                    strategy_name: row.get(6)?,
                    alert_type: row.get(7)?,
                    message: row.get(8)?,
                    is_read: row.get(9)?,
                    created_at: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user trade alerts: {}", e);
                AuthError::DatabaseError(format!("获取用户交易提醒失败: {}", e))
            })?,
        (None, None) => query
            .query_map(params![user_id], |row| {
                Ok(TradeAlert {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    asset_id: row.get(2)?,
                    asset_name: row.get(3)?,
                    asset_code: row.get(4)?,
                    strategy_id: row.get(5)?,
                    strategy_name: row.get(6)?,
                    alert_type: row.get(7)?,
                    message: row.get(8)?,
                    is_read: row.get(9)?,
                    created_at: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user trade alerts: {}", e);
                AuthError::DatabaseError(format!("获取用户交易提醒失败: {}", e))
            })?,
    };

    Ok(alerts)
}

pub fn get_portfolio_summary(user_id: i64) -> Result<PortfolioSummary, AuthError> {
    let conn = get_connection_from_pool()?;

    // 获取用户所有资产
    let mut stmt = conn.prepare(
        "SELECT a.id, a.asset_type_id, t.name, a.current_price,
                (SELECT SUM(CASE WHEN transaction_type = 'BUY' THEN amount ELSE -amount END) 
                 FROM transactions WHERE asset_id = a.id) as total_amount,
                (SELECT SUM(CASE WHEN transaction_type = 'BUY' THEN total_cost ELSE -total_cost END) 
                 FROM transactions WHERE asset_id = a.id) as total_cost
         FROM assets a
         JOIN asset_types t ON a.asset_type_id = t.id
         WHERE a.user_id = ?1"
    )?;

    let assets = stmt
        .query_map(params![user_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<f64>>(3)?,
                row.get::<_, Option<f64>>(4)?,
                row.get::<_, Option<f64>>(5)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            error!("Failed to fetch user assets: {}", e);
            AuthError::DatabaseError(format!("获取用户资产失败: {}", e))
        })?;

    // 按资产类型分组计算
    let mut asset_summaries = Vec::new();
    let mut asset_type_map: HashMap<String, (f64, f64, f64, f64)> = HashMap::new();

    let yesterday = (Utc::now() - Duration::days(1)).timestamp();

    let mut total_value = 0.0;
    let mut total_cost = 0.0;
    let mut total_daily_profit = 0.0;

    for (asset_id, _, asset_type, current_price, total_amount, total_cost_asset) in assets {
        if let (Some(price), Some(amount), Some(cost)) =
            (current_price, total_amount, total_cost_asset)
        {
            if amount <= 0.0 {
                continue; // 跳过没有持仓的资产
            }

            let current_value = price * amount;
            let profit = current_value - cost;
            let profit_percent = if cost > 0.0 {
                profit / cost * 100.0
            } else {
                0.0
            };

            // 获取昨日价格
            let yesterday_price: Option<f64> = conn
                .query_row(
                    "SELECT close_price FROM price_history 
                 WHERE asset_id = ?1 AND date <= ?2 
                 ORDER BY date DESC LIMIT 1",
                    params![asset_id, yesterday],
                    |row| row.get(0),
                )
                .ok();

            let daily_profit = if let Some(prev_price) = yesterday_price {
                (price - prev_price) * amount
            } else {
                0.0
            };

            let daily_profit_percent = if let Some(prev_price) = yesterday_price {
                if prev_price > 0.0 {
                    (price - prev_price) / prev_price * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            };

            // 更新资产类型统计
            let entry = asset_type_map
                .entry(asset_type.clone())
                .or_insert((0.0, 0.0, 0.0, 0.0));
            entry.0 += current_value;
            entry.1 += cost;
            entry.2 += profit;
            entry.3 += daily_profit;

            // 更新总计
            total_value += current_value;
            total_cost += cost;
            total_daily_profit += daily_profit;
        }
    }

    // 生成资产类型摘要
    for (asset_type, (value, cost, profit, daily_profit)) in asset_type_map {
        let profit_percent = if cost > 0.0 {
            profit / cost * 100.0
        } else {
            0.0
        };
        let daily_profit_percent = if value - daily_profit > 0.0 {
            daily_profit / (value - daily_profit) * 100.0
        } else {
            0.0
        };

        asset_summaries.push(AssetSummary {
            asset_type,
            total_value: value,
            total_cost: cost,
            total_profit: profit,
            total_profit_percent: profit_percent,
            daily_profit,
            daily_profit_percent,
        });
    }

    // 计算总体摘要
    let total_profit = total_value - total_cost;
    let total_profit_percent = if total_cost > 0.0 {
        total_profit / total_cost * 100.0
    } else {
        0.0
    };
    let daily_profit_percent = if total_value - total_daily_profit > 0.0 {
        total_daily_profit / (total_value - total_daily_profit) * 100.0
    } else {
        0.0
    };

    let summary = PortfolioSummary {
        total_value,
        total_cost,
        total_profit,
        total_profit_percent,
        daily_profit: total_daily_profit,
        daily_profit_percent,
        asset_summaries,
    };

    Ok(summary)
}
