/**
 * 交易相关功能
 */
use crate::database::get_db_connection;
use crate::error::AuthError;
use crate::models::Transaction;
use chrono::Utc;
use log::{error, info};
use rusqlite::params;

pub fn create_transaction(
    user_id: &str,
    asset_id: i64,
    transaction_type: &str,
    amount: f64,
    price: f64,
    transaction_date: i64,
    notes: Option<&str>,
) -> Result<Transaction, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 验证交易类型
    if transaction_type != "BUY" && transaction_type != "SELL" {
        return Err(AuthError::InvalidCredentials("交易类型无效，必须为 BUY 或 SELL".to_string()));
    }
    
    // 检查资产是否存在且属于该用户
    let asset_exists: bool = conn.query_row(
        "SELECT 1 FROM assets WHERE id = ?1 AND user_id = ?2",
        params![asset_id, user_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !asset_exists {
        return Err(AuthError::InvalidCredentials("资产不存在或无权限".to_string()));
    }
    
    // 如果是卖出交易，检查持仓是否足够
    if transaction_type == "SELL" {
        let current_holdings: f64 = conn.query_row(
            "SELECT COALESCE(SUM(CASE WHEN transaction_type = 'BUY' THEN amount ELSE -amount END), 0)
             FROM transactions
             WHERE asset_id = ?1 AND transaction_date <= ?2",
            params![asset_id, transaction_date],
            |row| row.get(0),
        ).unwrap_or(0.0);
        
        if current_holdings < amount {
            return Err(AuthError::InvalidCredentials(format!(
                "持仓不足，当前持仓: {}, 卖出数量: {}", 
                current_holdings, amount
            )));
        }
    }
    
    // 计算总成本
    let total_cost = amount * price;
    
    // 创建交易记录
    conn.execute(
        "INSERT INTO transactions (user_id, asset_id, transaction_type, amount, price, total_cost, transaction_date, notes, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            user_id,
            asset_id,
            transaction_type,
            amount,
            price,
            total_cost,
            transaction_date,
            notes,
            now
        ],
    )?;
    
    let transaction_id = conn.last_insert_rowid();
    
    // 获取资产信息
    let (asset_name, asset_code): (String, String) = conn.query_row(
        "SELECT name, code FROM assets WHERE id = ?1",
        params![asset_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    
    let transaction = Transaction {
        id: transaction_id,
        user_id: user_id.to_string(),
        asset_id,
        asset_name,
        asset_code,
        transaction_type: transaction_type.to_string(),
        amount,
        price,
        total_cost,
        transaction_date,
        notes: notes.map(|s| s.to_string()),
        created_at: now,
    };
    
    // 更新资产当前价格
    conn.execute(
        "UPDATE assets SET current_price = ?1, last_updated = ?2, updated_at = ?3 WHERE id = ?4",
        params![price, now, now, asset_id],
    )?;
    
    // 添加历史价格记录
    let date_exists: bool = conn.query_row(
        "SELECT 1 FROM price_history WHERE asset_id = ?1 AND date = ?2",
        params![asset_id, transaction_date],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !date_exists {
        conn.execute(
            "INSERT INTO price_history (asset_id, date, close_price, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![asset_id, transaction_date, price, now],
        )?;
    }
    
    info!("Transaction created: {} {} of {} for user: {}", 
          transaction_type, amount, asset_name, user_id);
    Ok(transaction)
}

pub fn update_transaction(
    id: i64,
    user_id: &str,
    transaction_type: &str,
    amount: f64,
    price: f64,
    transaction_date: i64,
    notes: Option<&str>,
) -> Result<Transaction, AuthError> {
    let conn = get_db_connection()?;
    
    // 验证交易类型
    if transaction_type != "BUY" && transaction_type != "SELL" {
        return Err(AuthError::InvalidCredentials("交易类型无效，必须为 BUY 或 SELL".to_string()));
    }
    
    // 检查交易记录是否存在且属于该用户
    let (asset_id, old_transaction_type, old_amount): (i64, String, f64) = conn.query_row(
        "SELECT asset_id, transaction_type, amount FROM transactions WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    ).map_err(|_| AuthError::InvalidCredentials("交易记录不存在或无权限".to_string()))?;
    
    // 如果是卖出交易，检查持仓是否足够
    if transaction_type == "SELL" {
        // 计算当前持仓（不包括当前交易）
        let mut current_holdings: f64 = conn.query_row(
            "SELECT COALESCE(SUM(CASE WHEN transaction_type = 'BUY' THEN amount ELSE -amount END), 0)
             FROM transactions
             WHERE asset_id = ?1 AND id != ?2 AND transaction_date <= ?3",
            params![asset_id, id, transaction_date],
            |row| row.get(0),
        ).unwrap_or(0.0);
        
        // 如果原交易是买入，则不需要加回原交易数量
        // 如果原交易是卖出，则需要加回原交易数量（因为我们已经排除了当前交易）
        if old_transaction_type == "SELL" {
            current_holdings += old_amount;
        }
        
        if current_holdings < amount {
            return Err(AuthError::InvalidCredentials(format!(
                "持仓不足，当前持仓: {}, 卖出数量: {}", 
                current_holdings, amount
            )));
        }
    }
    
    // 计算总成本
    let total_cost = amount * price;
    
    // 更新交易记录
    conn.execute(
        "UPDATE transactions 
         SET transaction_type = ?1, amount = ?2, price = ?3, total_cost = ?4, transaction_date = ?5, notes = ?6
         WHERE id = ?7",
        params![
            transaction_type,
            amount,
            price,
            total_cost,
            transaction_date,
            notes,
            id
        ],
    )?;
    
    // 获取资产信息
    let (asset_name, asset_code): (String, String) = conn.query_row(
        "SELECT name, code FROM assets WHERE id = ?1",
        params![asset_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    
    let created_at: i64 = conn.query_row(
        "SELECT created_at FROM transactions WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    
    let transaction = Transaction {
        id,
        user_id: user_id.to_string(),
        asset_id,
        asset_name,
        asset_code,
        transaction_type: transaction_type.to_string(),
        amount,
        price,
        total_cost,
        transaction_date,
        notes: notes.map(|s| s.to_string()),
        created_at,
    };
    
    // 添加历史价格记录
    let date_exists: bool = conn.query_row(
        "SELECT 1 FROM price_history WHERE asset_id = ?1 AND date = ?2",
        params![asset_id, transaction_date],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !date_exists {
        conn.execute(
            "INSERT INTO price_history (asset_id, date, close_price, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![asset_id, transaction_date, price, Utc::now().timestamp()],
        )?;
    }
    
    info!("Transaction updated: {} for user: {}", id, user_id);
    Ok(transaction)
}

pub fn delete_transaction(id: i64, user_id: &str) -> Result<(), AuthError> {
    let conn = get_db_connection()?;
    
    // 检查交易记录是否存在且属于该用户
    let transaction_exists: bool = conn.query_row(
        "SELECT 1 FROM transactions WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !transaction_exists {
        return Err(AuthError::InvalidCredentials("交易记录不存在或无权限".to_string()));
    }
    
    // 删除交易记录
    conn.execute(
        "DELETE FROM transactions WHERE id = ?1",
        params![id],
    )?;
    
    info!("Transaction deleted: {} for user: {}", id, user_id);
    Ok(())
}

pub fn get_user_transactions(
    user_id: &str,
    asset_id: Option<i64>,
    start_date: Option<i64>,
    end_date: Option<i64>,
) -> Result<Vec<Transaction>, AuthError> {
    let conn = get_db_connection()?;
    
    // 构建查询条件
    let mut conditions = vec!["t.user_id = ?1".to_string()];
    let mut params: Vec<&dyn rusqlite::ToSql> = vec![&user_id];
    
    if let Some(a_id) = asset_id {
        conditions.push("t.asset_id = ?".to_string());
        params.push(&a_id);
    }
    
    if let Some(s_date) = start_date {
        conditions.push("t.transaction_date >= ?".to_string());
        params.push(&s_date);
    }
    
    if let Some(e_date) = end_date {
        conditions.push("t.transaction_date <= ?".to_string());
        params.push(&e_date);
    }
    
    let condition_str = conditions.join(" AND ");
    
    // 构建查询语句
    let query = format!(
        "SELECT t.id, t.user_id, t.asset_id, a.name, a.code, t.transaction_type, 
                t.amount, t.price, t.total_cost, t.transaction_date, t.notes, t.created_at
         FROM transactions t
         JOIN assets a ON t.asset_id = a.id
         WHERE {}
         ORDER BY t.transaction_date DESC, t.id DESC", condition_str);
    
    let mut stmt = conn.prepare(&query)?;
    
    let transactions = stmt.query_map(rusqlite::params_from_iter(params), |row| {
        Ok(Transaction {
            id: row.get(0)?,
            user_id: row.get(1)?,
            asset_id: row.get(2)?,
            asset_name: row.get(3)?,
            asset_code: row.get(4)?,
            transaction_type: row.get(5)?,
            amount: row.get(6)?,
            price: row.get(7)?,
            total_cost: row.get(8)?,
            transaction_date: row.get(9)?,
            notes: row.get(10)?,
            created_at: row.get(11)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| {
        error!("Failed to fetch user transactions: {}", e);
        AuthError::DatabaseError(format!("获取用户交易记录失败: {}", e))
    })?;
    
    Ok(transactions)
}