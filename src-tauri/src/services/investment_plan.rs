use crate::database::get_connection_from_pool;
use crate::error::auth::AuthError;
use crate::models::InvestmentPlan;
use chrono::{Datelike, Duration, NaiveDateTime, TimeZone, Utc, Weekday};
use log::{error, info};
use rusqlite::params;

/**
 * 创建或更新定投计划
 * 如果 plan_id 为 None，则创建新计划；否则更新现有计划
 */
pub fn save_investment_plan(
    plan_id: Option<i64>,
    user_id: i64,
    asset_id: i64,
    name: &str,
    frequency: &str,
    day_of_week: Option<i64>,
    day_of_month: Option<i64>,
    amount: f64,
    is_active: bool,
) -> Result<InvestmentPlan, AuthError> {
    let conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 验证频率
    match frequency {
        "DAILY" => {
            if day_of_week.is_some() || day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials(
                    "每日定投不需要指定星期几或每月几号".to_string(),
                ));
            }
        }
        "WEEKLY" => {
            if day_of_week.is_none() || day_of_week.unwrap() < 1 || day_of_week.unwrap() > 7 {
                return Err(AuthError::InvalidCredentials(
                    "每周定投需要指定星期几（1-7）".to_string(),
                ));
            }
            if day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials(
                    "每周定投不需要指定每月几号".to_string(),
                ));
            }
        }
        "BIWEEKLY" => {
            if day_of_week.is_none() || day_of_week.unwrap() < 1 || day_of_week.unwrap() > 7 {
                return Err(AuthError::InvalidCredentials(
                    "每两周定投需要指定星期几（1-7）".to_string(),
                ));
            }
            if day_of_month.is_some() {
                return Err(AuthError::InvalidCredentials(
                    "每两周定投不需要指定每月几号".to_string(),
                ));
            }
        }
        "MONTHLY" => {
            if day_of_month.is_none() || day_of_month.unwrap() < 1 || day_of_month.unwrap() > 31 {
                return Err(AuthError::InvalidCredentials(
                    "每月定投需要指定每月几号（1-31）".to_string(),
                ));
            }
            if day_of_week.is_some() {
                return Err(AuthError::InvalidCredentials(
                    "每月定投不需要指定星期几".to_string(),
                ));
            }
        }
        _ => {
            return Err(AuthError::InvalidCredentials(
                "无效的定投频率，支持的频率：DAILY, WEEKLY, BIWEEKLY, MONTHLY".to_string(),
            ))
        }
    }

    // 计算下次执行时间
    let next_execution = if is_active {
        Some(calculate_next_execution(
            frequency,
            day_of_week,
            day_of_month,
        )?)
    } else {
        None
    };

    let plan_id = match plan_id {
        // 更新现有计划
        Some(id) => {
            // 检查定投计划是否存在且属于该用户
            let plan_exists: bool = conn
                .query_row(
                    "SELECT 1 FROM investment_plans WHERE id = ?1 AND user_id = ?2",
                    params![id, user_id],
                    |_| Ok(true),
                )
                .unwrap_or(false);

            if !plan_exists {
                return Err(AuthError::InvalidCredentials(
                    "定投计划不存在或无权限".to_string(),
                ));
            }

            // 获取资产ID（用于后续查询）
            let db_asset_id: i64 = conn.query_row(
                "SELECT asset_id FROM investment_plans WHERE id = ?1",
                params![id],
                |row| row.get(0),
            )?;

            // 如果传入的资产ID与数据库中的不同，需要验证新资产是否存在且属于该用户
            if asset_id != db_asset_id {
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
            }

            // 更新定投计划
            conn.execute(
                "UPDATE investment_plans 
                 SET asset_id = ?1, name = ?2, frequency = ?3, day_of_week = ?4, day_of_month = ?5, 
                     amount = ?6, is_active = ?7, next_execution = ?8, updated_at = ?9
                 WHERE id = ?10",
                params![
                    asset_id,
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

            info!("Investment plan updated: {} for user: {}", name, user_id);
            id
        }
        // 创建新计划
        None => {
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
                    is_active,
                    next_execution,
                    now,
                    now
                ],
            )?;

            let new_id = conn.last_insert_rowid();
            info!("Investment plan created: {} for user: {}", name, user_id);
            new_id
        }
    };

    // 获取资产信息
    let (asset_name, asset_code): (String, String) = conn.query_row(
        "SELECT name, code FROM assets WHERE id = ?1",
        params![asset_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    // 获取计划的其他信息
    let (last_executed, created_at): (Option<i64>, i64) = conn.query_row(
        "SELECT last_executed, created_at FROM investment_plans WHERE id = ?1",
        params![plan_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    // 构建返回的计划对象
    let plan = InvestmentPlan {
        id: plan_id,
        user_id,
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

    Ok(plan)
}
/**
 * @dev 根据id删除用户的定投计划
 */
pub fn delete_investment_plan(id: i64, user_id: i64) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;

    // 检查定投计划是否存在且属于该用户
    let plan_exists: bool = conn
        .query_row(
            "SELECT 1 FROM investment_plans WHERE id = ?1 AND user_id = ?2",
            params![id, user_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !plan_exists {
        return Err(AuthError::InvalidCredentials(
            "定投计划不存在或无权限".to_string(),
        ));
    }

    // 删除定投计划
    conn.execute("DELETE FROM investment_plans WHERE id = ?1", params![id])?;

    info!("Investment plan deleted: {} for user: {}", id, user_id);
    Ok(())
}
/**
 * @dev 根据资产类型，获取用户的定投计划
 */
pub fn get_user_investment_plans(
    user_id: i64,
    asset_id: Option<i64>,
) -> Result<Vec<InvestmentPlan>, AuthError> {
    let conn = get_connection_from_pool()?;

    // 如果 asset_id 为 Some(0)，视为查询所有类型
    let effective_asset_id = match asset_id {
        Some(0) => None,
        other => other,
    };

    let mut query = if let Some(a_id) = effective_asset_id {
        conn.prepare(
            "SELECT p.id, p.user_id, p.asset_id, a.name, a.code, p.name, p.frequency, 
                    p.day_of_week, p.day_of_month, p.amount, p.is_active, 
                    p.last_executed, p.next_execution, p.created_at, p.updated_at
             FROM investment_plans p
             JOIN assets a ON p.asset_id = a.id
             WHERE p.user_id = ?1 AND p.asset_id = ?2
             ORDER BY p.name",
        )?
    } else {
        conn.prepare(
            "SELECT p.id, p.user_id, p.asset_id, a.name, a.code, p.name, p.frequency, 
                    p.day_of_week, p.day_of_month, p.amount, p.is_active, 
                    p.last_executed, p.next_execution, p.created_at, p.updated_at
             FROM investment_plans p
             JOIN assets a ON p.asset_id = a.id
             WHERE p.user_id = ?1
             ORDER BY p.name",
        )?
    };

    let plans = if let Some(a_id) = effective_asset_id {
        query
            .query_map(params![user_id, a_id], |row| {
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
        query
            .query_map(params![user_id], |row| {
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

/**
 * @dev 执行定投计划，并更新下一次定投的执行时间
 */
pub fn execute_due_investment_plans() -> Result<usize, AuthError> {
    let mut conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 获取所有到期的定投计划
    let plans = {
        let mut stmt = conn.prepare(
            "SELECT p.id, p.user_id, p.asset_id, p.amount, p.frequency, p.day_of_week, p.day_of_month
             FROM investment_plans p
             WHERE p.is_active = 1 AND p.next_execution <= ?1",
        )?;

        let result = stmt
            .query_map(params![now], |row| {
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
            });

        result?
    };

    let mut executed_count = 0;

    for (plan_id, user_id, asset_id, amount, frequency, day_of_week, day_of_month) in plans {
        // 获取资产当前价格
        let current_price: Option<f64> = conn
            .query_row(
                "SELECT current_price FROM assets WHERE id = ?1",
                params![asset_id],
                |row| row.get(0),
            )
            .ok();

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

            info!(
                "Executed investment plan: {} for asset: {}",
                plan_id, asset_id
            );
        } else {
            error!(
                "Failed to execute investment plan: {} for asset: {} - No price available",
                plan_id, asset_id
            );
        }
    }

    Ok(executed_count)
}

/**
 * @dev 计算定投计划的下一次执行时间
 */
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
        }
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
                let days_until_target = (7 + target_weekday.num_days_from_monday()
                    - current_weekday.num_days_from_monday())
                    % 7;

                // 如果今天是目标日期但已经过了执行时间，则设置为下周
                let days_to_add = if days_until_target == 0 {
                    7
                } else {
                    days_until_target
                };

                let next = now + Duration::days(days_to_add as i64);
                Ok(next.timestamp())
            } else {
                Err(AuthError::InvalidCredentials(
                    "每周定投需要指定星期几".to_string(),
                ))
            }
        }
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
                let days_until_target = (7 + target_weekday.num_days_from_monday()
                    - current_weekday.num_days_from_monday())
                    % 7;

                // 如果今天是目标日期但已经过了执行时间，则设置为两周后
                let days_to_add = if days_until_target == 0 {
                    14
                } else {
                    days_until_target + 7
                };

                let next = now + Duration::days(days_to_add as i64);
                Ok(next.timestamp())
            } else {
                Err(AuthError::InvalidCredentials(
                    "每两周定投需要指定星期几".to_string(),
                ))
            }
        }
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
                        if (target_year % 4 == 0 && target_year % 100 != 0)
                            || target_year % 400 == 0
                        {
                            29
                        } else {
                            28
                        }
                    }
                    _ => return Err(AuthError::InternalError("无效的月份".to_string())),
                };

                let actual_target_day = std::cmp::min(target_day, days_in_month);

                // 创建目标日期时间
                if let Some(target_date) = NaiveDateTime::new(
                    chrono::NaiveDate::from_ymd_opt(target_year, target_month, actual_target_day)
                        .ok_or_else(|| AuthError::InternalError("无效的日期".to_string()))?,
                    now.time(),
                )
                .and_local_timezone(Utc)
                .single()
                {
                    Ok(target_date.timestamp())
                } else {
                    Err(AuthError::InternalError("无法创建目标日期".to_string()))
                }
            } else {
                Err(AuthError::InvalidCredentials(
                    "每月定投需要指定每月几号".to_string(),
                ))
            }
        }
        _ => Err(AuthError::InvalidCredentials("无效的定投频率".to_string())),
    }
}

/**
 * @dev 获取当天需要执行的定投计划
 * @param user_id 用户ID
 * @param asset_type_id 资产类型ID，如果为0则查询所有类型
 * @return 当天需要执行的定投计划列表
 */
pub fn get_today_investment_plans(
    user_id: i64,
    asset_type_id: i64,
) -> Result<Vec<InvestmentPlan>, AuthError> {
    let conn = get_connection_from_pool()?;
    let now = Utc::now();
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().timestamp();
    let today_end = now
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .timestamp();

    // 构建查询SQL
    let mut sql = String::from(
        "SELECT p.id, p.user_id, p.asset_id, a.name, a.code, p.name, p.frequency, 
                p.day_of_week, p.day_of_month, p.amount, p.is_active, 
                p.last_executed, p.next_execution, p.created_at, p.updated_at
         FROM investment_plans p
         JOIN assets a ON p.asset_id = a.id
         WHERE p.user_id = ?1
         AND p.is_active = 1
         AND p.next_execution >= ?2
         AND p.next_execution <= ?3",
    );

    // 如果指定了资产类型且不为0，则添加资产类型过滤条件
    if asset_type_id > 0 {
        sql.push_str(" AND a.asset_type_id = ?4");
    }

    sql.push_str(" ORDER BY p.next_execution ASC");

    let mut stmt = conn.prepare(&sql)?;

    // 执行查询，根据是否有资产类型过滤条件使用不同的参数
    let plans = if asset_type_id > 0 {
        stmt.query_map(
            params![user_id, today_start, today_end, asset_type_id],
            |row| {
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
            },
        )?
    } else {
        stmt.query_map(params![user_id, today_start, today_end], |row| {
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
    };

    // 收集结果
    let plans = plans.collect::<Result<Vec<_>, _>>().map_err(|e| {
        error!("Failed to fetch today's investment plans: {}", e);
        AuthError::DatabaseError(format!("获取今日定投计划失败: {}", e))
    })?;

    // 记录日志
    info!(
        "Retrieved {} investment plans for today for user: {}",
        plans.len(),
        user_id
    );

    Ok(plans)
}
