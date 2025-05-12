/**
 * 定投计划
 */

use crate::database::get_db_connection;
use crate::error::AuthError;
use crate::models::InvestmentPlan;
use chrono::{Datelike, Duration, NaiveDateTime, TimeZone, Utc, Weekday};
use log::{error, info};
use rusqlite::params;

pub fn create_investment_plan(
    user_id: &str,
    asset_id: i64,
    name: &str,
    frequency: &str,
    day_of_week: Option<i64>,
    day_of_month: Option<i64>,
    amount: f64,
) -> Result<InvestmentPlan, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 验证频率
    match frequency {
        "DAILY" => {
            if day_of_week.is_some() || day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials("每日定投不需要指定星期几或每月几号".to_string()));
            }
        },
        "WEEKLY" => {
            if day_of_week.is_none() || day_of_week.unwrap() < 1 || day_of_week.unwrap() > 7 {
                return Err(AuthError::InvalidCredentials("每周定投需要指定星期几（1-7）".to_string()));
            }
            if day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials("每周定投不需要指定每月几号".to_string()));
            }
        },
        "BIWEEKLY" => {
            if day_of_week.is_none() || day_of_week.unwrap() < 1 || day_of_week.unwrap() > 7 {
                return Err(AuthError::InvalidCredentials("每两周定投需要指定星期几（1-7）".to_string()));
            }
            if day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials("每两周定投不需要指定每月几号".to_string()));
            }
        },
        "MONTHLY" => {
            if day_of_month.is_none() || day_of_month.unwrap() < 1 || day_of_month.unwrap() > 31 {
                return Err(AuthError::InvalidCredentials("每月定投需要指定每月几号（1-31）".to_string()));
            }
            if day_of_week.is_some() {
                return Err(AuthError::InvalidCredentials("每月定投不需要指定星期几".to_string()));
            }
        },
        _ => return Err(AuthError::InvalidCredentials("无效的定投频率，支持的频率：DAILY, WEEKLY, BIWEEKLY, MONTHLY".to_string())),
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
    
    // 计算下次执行时间
    let next_execution = calculate_next_execution(frequency, day_of_week, day_of_month)?;
    
    // 创建定投计划
    conn.execute(
        "INSERT INTO investment_plans (
            user_id, asset_id, name, frequency, day_of_week, day_of_month, 
            amount, is_active, next_execution, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            user_id,
            asset_id,
            name,
            frequency,
            day_of_week,
            day_of_month,
            amount,
            true,
            next_execution,
            now,
            now
        ],
    )?;
    
    let plan_id = conn.last_insert_rowid();
    
    // 获取资产信息
    let (asset_name, asset_code): (String, String) = conn.query_row(
        "SELECT name, code FROM assets WHERE id = ?1",
        params![asset_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    
    let plan = InvestmentPlan {
        id: plan_id,
        user_id: user_id.to_string(),
        asset_id,
        asset_name,
        asset_code,
        name: name.to_string(),
        frequency: frequency.to_string(),
        day_of_week,
        day_of_month,
        amount,
        is_active: true,
        last_executed: None,
        next_execution: Some(next_execution),
        created_at: now,
        updated_at: now,
    };
    
    info!("Investment plan created: {} for asset: {} by user: {}", name, asset_name, user_id);
    Ok(plan)
}

pub fn update_investment_plan(
    id: i64,
    user_id: &str,
    name: &str,
    frequency: &str,
    day_of_week: Option<i64>,
    day_of_month: Option<i64>,
    amount: f64,
    is_active: bool,
) -> Result<InvestmentPlan, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 验证频率
    match frequency {
        "DAILY" => {
            if day_of_week.is_some() || day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials("每日定投不需要指定星期几或每月几号".to_string()));
            }
        },
        "WEEKLY" => {
            if day_of_week.is_none() || day_of_week.unwrap() < 1 || day_of_week.unwrap() > 7 {
                return Err(AuthError::InvalidCredentials("每周定投需要指定星期几（1-7）".to_string()));
            }
            if day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials("每周定投不需要指定每月几号".to_string()));
            }
        },
        "BIWEEKLY" => {
            if day_of_week.is_none() || day_of_week.unwrap() < 1 || day_of_week.unwrap() > 7 {
                return Err(AuthError::InvalidCredentials("每两周定投需要指定星期几（1-7）".to_string()));
            }
            if day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials("每两周定投不需要指定每月几号".to_string()));
            }
        },
        "MONTHLY" => {
            if day_of_month.is_none() || day_of_month.unwrap() < 1 || day_of_month.unwrap() > 31 {
                return Err(AuthError::InvalidCredentials("每月定投需要指定每月几号（1-31）".to_string()));
            }
            if day_of_week.is_some() {
                return Err(AuthError::InvalidCredentials("每月定投不需要指定星期几".to_string()));
            }
        },
        _ => return Err(AuthError::InvalidCredentials("无效的定投频率，支持的频率：DAILY, WEEKLY, BIWEEKLY, MONTHLY".to_string())),
    }
    
    // 检查定投计划是否存在且属于该用户
    let (asset_id, last_executed): (i64, Option<i64>) = conn.query_row(
        "SELECT asset_id, last_executed FROM investment_plans WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|_| AuthError::InvalidCredentials("定投计划不存在或无权限".to_string()))?;
    
    // 计算下次执行时间
    let next_execution = if is_active {
        Some(calculate_next_execution(frequency, day_of_week, day_of_month)?)
    } else {
        None
    };
    
    // 更新定投计划
    conn.execute(
        "UPDATE investment_plans 
         SET name = ?1, frequency = ?2, day_of_week = ?3, day_of_month = ?4, 
             amount = ?5, is_active = ?6, next_execution = ?7, updated_at = ?8
         WHERE id = ?9",
        params![
            name,
            frequency,
            day_of_week,
            day_of_month,
            amount,
            is_active,
            next_execution,
            now,
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
        "SELECT created_at FROM investment_plans WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    
    let plan = InvestmentPlan {
        id,
        user_id: user_id.to_string(),
        asset_id,
        asset_name,
        asset_code,
        name: name.to_string(),
        frequency: frequency.to_string(),
        day_of_week,
        day_of_month,
        amount,
        is_active,
        last_executed,
        next_execution,
        created_at,
        updated_at: now,
    };
    
    info!("Investment plan updated: {} for user: {}", name, user_id);
    Ok(plan)
}

pub fn delete_investment_plan(id: i64, user_id: &str) -> Result<(), AuthError> {
    let conn = get_db_connection()?;
    
    // 检查定投计划是否存在且属于该用户
    let plan_exists: bool = conn.query_row(
        "SELECT 1 FROM investment_plans WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if !plan_exists {
        return Err(AuthError::InvalidCredentials("定投计划不存在或无权限".to_string()));
    }
    
    // 删除定投计划
    conn.execute(
        "DELETE FROM investment_plans WHERE id = ?1",
        params![id],
    )?;
    
    info!("Investment plan deleted: {} for user: {}", id, user_id);
    Ok(())
}

pub fn get_user_investment_plans(user_id: &str, asset_id: Option<i64>) -> Result<Vec<InvestmentPlan>, AuthError> {
    let conn = get_db_connection()?;
    
    let query = if let Some(a_id) = asset_id {
        conn.prepare(
            "SELECT p.id, p.user_id, p.asset_id, a.name, a.code, p.name, p.frequency, 
                    p.day_of_week, p.day_of_month, p.amount, p.is_active, 
                    p.last_executed, p.next_execution, p.created_at, p.updated_at
             FROM investment_plans p
             JOIN assets a ON p.asset_id = a.id
             WHERE p.user_id = ?1 AND p.asset_id = ?2
             ORDER BY p.name"
        )?
    } else {
        conn.prepare(
            "SELECT p.id, p.user_id, p.asset_id, a.name, a.code, p.name, p.frequency, 
                    p.day_of_week, p.day_of_month, p.amount, p.is_active, 
                    p.last_executed, p.next_execution, p.created_at, p.updated_at
             FROM investment_plans p
             JOIN assets a ON p.asset_id = a.id
             WHERE p.user_id = ?1
             ORDER BY p.name"
        )?
    };
    
    let plans = if let Some(a_id) = asset_id {
        query.query_map(params![user_id, a_id], |row| {
            Ok(InvestmentPlan {
                id: row.get(0)?,
                user_id: row.get(1)?,
                asset_id: row.get(2)?,
                asset_name: row.get(3)?,
                asset_code: row.get(4)?,
                name: row.get(5)?,
                frequency: row.get(6)?,
                day_of_week: row.get(7)?,
                day_of_month: row.get(8)?,
                amount: row.get(9)?,
                is_active: row.get(10)?,
                last_executed: row.get(11)?,
                next_execution: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            error!("Failed to fetch user investment plans: {}", e);
            AuthError::DatabaseError(format!("获取用户定投计划失败: {}", e))
        })?
    } else {
        query.query_map(params![user_id], |row| {
            Ok(InvestmentPlan {
                id: row.get(0)?,
                user_id: row.get(1)?,
                asset_id: row.get(2)?,
                asset_name: row.get(3)?,
                asset_code: row.get(4)?,
                name: row.get(5)?,
                frequency: row.get(6)?,
                day_of_week: row.get(7)?,
                day_of_month: row.get(8)?,
                amount: row.get(9)?,
                is_active: row.get(10)?,
                last_executed: row.get(11)?,
                next_execution: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            error!("Failed to fetch user investment plans: {}", e);
            AuthError::DatabaseError(format!("获取用户定投计划失败: {}", e))
        })?
    };
    
    Ok(plans)
}

pub fn execute_due_investment_plans() -> Result<usize, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 获取所有到期的定投计划
    let mut stmt = conn.prepare(
        "SELECT p.id, p.user_id, p.asset_id, p.amount, p.frequency, p.day_of_week, p.day_of_month
         FROM investment_plans p
         WHERE p.is_active = 1 AND p.next_execution <= ?1"
    )?;
    
    let plans = stmt.query_map(params![now], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, Option<i64>>(5)?,
            row.get::<_, Option<i64>>(6)?,
        ))
    })?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| {
        error!("Failed to fetch due investment plans: {}", e);
        AuthError::DatabaseError(format!("获取到期定投计划失败: {}", e))
    })?;
    
    let mut executed_count = 0;
    
    for (plan_id, user_id, asset_id, amount, frequency, day_of_week, day_of_month) in plans {
        // 获取资产当前价格
        let current_price: Option<f64> = conn.query_row(
            "SELECT current_price FROM assets WHERE id = ?1",
            params![asset_id],
            |row| row.get(0),
        ).ok();
        
        if let Some(price) = current_price {
            // 创建交易记录
            let tx = conn.transaction()?;
            
            tx.execute(
                "INSERT INTO transactions (
                    user_id, asset_id, transaction_type, amount, price, 
                    total_cost, transaction_date, notes, created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    user_id,
                    asset_id,
                    "BUY",
                    amount / price, // 购买数量 = 金额 / 价格
                    price,
                    amount,
                    now,
                    format!("自动定投 (计划ID: {})", plan_id),
                    now
                ],
            )?;
            
            // 计算下次执行时间
            let next_execution = calculate_next_execution(&frequency, day_of_week, day_of_month)?;
            
            // 更新定投计划
            tx.execute(
                "UPDATE investment_plans 
                 SET last_executed = ?1, next_execution = ?2, updated_at = ?3
                 WHERE id = ?4",
                params![now, next_execution, now, plan_id],
            )?;
            
            tx.commit()?;
            executed_count += 1;
            
            info!("Executed investment plan: {} for asset: {}", plan_id, asset_id);
        } else {
            error!("Failed to execute investment plan: {} for asset: {} - No price available", 
                   plan_id, asset_id);
        }
    }
    
    Ok(executed_count)
}

fn calculate_next_execution(
    frequency: &str,
    day_of_week: Option<i64>,
    day_of_month: Option<i64>,
) -> Result<i64, AuthError> {
    let now = Utc::now();
    let today = now.date_naive();
    
    match frequency {
        "DAILY" => {
            // 明天同一时间
            let next = now + Duration::days(1);
            Ok(next.timestamp())
        },
        "WEEKLY" => {
            if let Some(dow) = day_of_week {
                // 将 1-7 转换为 Weekday (其中 1 = Mon, 7 = Sun)
                let target_weekday = match dow {
                    1 => Weekday::Mon,
                    2 => Weekday::Tue,
                    3 => Weekday::Wed,
                    4 => Weekday::Thu,
                    5 => Weekday::Fri,
                    6 => Weekday::Sat,
                    7 => Weekday::Sun,
                    _ => return Err(AuthError::InvalidCredentials("无效的星期几".to_string())),
                };
                
                let current_weekday = now.weekday();
                let days_until_target = (7 + target_weekday.num_days_from_monday() - current_weekday.num_days_from_monday()) % 7;
                
                // 如果今天是目标日期但已经过了执行时间，则设置为下周
                let days_to_add = if days_until_target == 0 { 7 } else { days_until_target };
                
                let next = now + Duration::days(days_to_add as i64);
                Ok(next.timestamp())
            } else {
                Err(AuthError::InvalidCredentials("每周定投需要指定星期几".to_string()))
            }
        },
        "BIWEEKLY" => {
            if let Some(dow) = day_of_week {
                // 与每周定投类似，但间隔为两周
                let target_weekday = match dow {
                    1 => Weekday::Mon,
                    2 => Weekday::Tue,
                    3 => Weekday::Wed,
                    4 => Weekday::Thu,
                    5 => Weekday::Fri,
                    6 => Weekday::Sat,
                    7 => Weekday::Sun,
                    _ => return Err(AuthError::InvalidCredentials("无效的星期几".to_string())),
                };
                
                let current_weekday = now.weekday();
                let days_until_target = (7 + target_weekday.num_days_from_monday() - current_weekday.num_days_from_monday()) % 7;
                
                // 如果今天是目标日期但已经过了执行时间，则设置为两周后
                let days_to_add = if days_until_target == 0 { 14 } else { days_until_target + 7 };
                
                let next = now + Duration::days(days_to_add as i64);
                Ok(next.timestamp())
            } else {
                Err(AuthError::InvalidCredentials("每两周定投需要指定星期几".to_string()))
            }
        },
        "MONTHLY" => {
            if let Some(dom) = day_of_month {
                if dom < 1 || dom > 31 {
                    return Err(AuthError::InvalidCredentials("无效的每月日期".to_string()));
                }
                
                let current_day = today.day() as i64;
                let current_month = today.month();
                let current_year = today.year();
                
                // 如果当前日期小于目标日期，则设置为本月目标日期
                // 否则设置为下月目标日期
                let (target_year, target_month, target_day) = if current_day < dom {
                    (current_year, current_month, dom as u32)
                } else {
                    // 计算下个月
                    let next_month = if current_month == 12 {
                        (current_year + 1, 1)
                    } else {
                        (current_year, current_month + 1)
                    };
                    
                    (next_month.0, next_month.1, dom as u32)
                };
                
                // 处理月末日期问题（例如2月没有30日）
                let days_in_month = match target_month {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                    4 | 6 | 9 | 11 => 30,
                    2 => {
                        // 闰年2月有29天，平年28天
                        if (target_year % 4 == 0 && target_year % 100 != 0) || target_year % 400 == 0 {
                            29
                        } else {
                            28
                        }
                    },
                    _ => return Err(AuthError::InternalError("无效的月份".to_string())),
                };
                
                let actual_target_day = std::cmp::min(target_day, days_in_month);
                
                // 创建目标日期时间
                if let Some(target_date) = NaiveDateTime::new(
                    chrono::NaiveDate::from_ymd_opt(target_year, target_month, actual_target_day).ok_or_else(|| 
                        AuthError::InternalError("无效的日期".to_string())
                    )?,
                    now.time().naive_utc(),
                ).and_local_timezone(Utc).single() {
                    Ok(target_date.timestamp())
                } else {
                    Err(AuthError::InternalError("无法创建目标日期".to_string()))
                }
            } else {
                Err(AuthError::InvalidCredentials("每月定投需要指定每月几号".to_string()))
            }
        },
        _ => Err(AuthError::InvalidCredentials("无效的定投频率".to_string())),
    }
}