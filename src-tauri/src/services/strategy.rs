/**
 * 策略
 */
use crate::database::get_connection_from_pool;
use crate::error::auth::AuthError;
use crate::models::{
    BacktestResult, InvestmentStrategy, PerformancePoint, StrategyApplication, Transaction,
};
use chrono::Utc;
use log::{error, info};
use rusqlite::params;
use serde_json::{self, Value};

pub fn create_investment_strategy(
    user_id: &str,
    name: &str,
    description: Option<&str>,
    strategy_type: &str,
    parameters: &str,
) -> Result<InvestmentStrategy, AuthError> {
    let conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 验证策略类型
    match strategy_type {
        "MACD" | "RSI" | "MOVING_AVERAGE" | "CUSTOM" => {}
        _ => return Err(AuthError::InvalidCredentials("无效的策略类型".to_string())),
    }

    // 验证参数是否为有效的JSON
    if let Err(e) = serde_json::from_str::<Value>(parameters) {
        return Err(AuthError::InvalidCredentials(format!(
            "无效的策略参数: {}",
            e
        )));
    }

    // 检查是否已存在同名策略
    let strategy_exists: bool = conn
        .query_row(
            "SELECT 1 FROM investment_strategies WHERE user_id = ?1 AND name = ?2",
            params![user_id, name],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if strategy_exists {
        return Err(AuthError::InvalidCredentials("已存在同名策略".to_string()));
    }

    // 创建策略
    conn.execute(
        "INSERT INTO investment_strategies (user_id, name, description, strategy_type, parameters, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            user_id,
            name,
            description,
            strategy_type,
            parameters,
            now,
            now
        ],
    )?;

    let strategy_id = conn.last_insert_rowid();

    let strategy = InvestmentStrategy {
        id: strategy_id,
        user_id: user_id.to_string(),
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        strategy_type: strategy_type.to_string(),
        parameters: parameters.to_string(),
        created_at: now,
        updated_at: now,
    };

    info!(
        "Investment strategy created: {} for user: {}",
        name, user_id
    );
    Ok(strategy)
}

pub fn update_investment_strategy(
    id: i64,
    user_id: &str,
    name: &str,
    description: Option<&str>,
    parameters: &str,
) -> Result<InvestmentStrategy, AuthError> {
    let conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 检查策略是否存在且属于该用户
    let (strategy_type, created_at): (String, i64) = conn.query_row(
        "SELECT strategy_type, created_at FROM investment_strategies WHERE id = ?1 AND user_id = ?2",
        params![id, user_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|_| AuthError::InvalidCredentials("策略不存在或无权限".to_string()))?;

    // 验证参数是否为有效的JSON
    if let Err(e) = serde_json::from_str::<Value>(parameters) {
        return Err(AuthError::InvalidCredentials(format!(
            "无效的策略参数: {}",
            e
        )));
    }

    // 检查是否已存在同名策略
    let same_name_exists: bool = conn
        .query_row(
            "SELECT 1 FROM investment_strategies 
         WHERE user_id = ?1 AND name = ?2 AND id != ?3",
            params![user_id, name, id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if same_name_exists {
        return Err(AuthError::InvalidCredentials("已存在同名策略".to_string()));
    }

    // 更新策略
    conn.execute(
        "UPDATE investment_strategies 
         SET name = ?1, description = ?2, parameters = ?3, updated_at = ?4
         WHERE id = ?5",
        params![name, description, parameters, now, id],
    )?;

    let strategy = InvestmentStrategy {
        id,
        user_id: user_id.to_string(),
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        strategy_type,
        parameters: parameters.to_string(),
        created_at,
        updated_at: now,
    };

    info!(
        "Investment strategy updated: {} for user: {}",
        name, user_id
    );
    Ok(strategy)
}

pub fn delete_investment_strategy(id: i64, user_id: &str) -> Result<(), AuthError> {
    let mut conn = get_connection_from_pool()?;

    // 检查策略是否存在且属于该用户
    let strategy_exists: bool = conn
        .query_row(
            "SELECT 1 FROM investment_strategies WHERE id = ?1 AND user_id = ?2",
            params![id, user_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !strategy_exists {
        return Err(AuthError::InvalidCredentials(
            "策略不存在或无权限".to_string(),
        ));
    }

    // 开始事务
    let tx = conn.transaction()?;

    // 删除策略应用
    tx.execute(
        "DELETE FROM strategy_applications WHERE strategy_id = ?1",
        params![id],
    )?;

    // 删除相关的交易提醒
    tx.execute(
        "DELETE FROM trade_alerts WHERE strategy_id = ?1",
        params![id],
    )?;

    // 删除策略
    tx.execute(
        "DELETE FROM investment_strategies WHERE id = ?1",
        params![id],
    )?;

    // 提交事务
    tx.commit()?;

    info!("Investment strategy deleted: {} for user: {}", id, user_id);
    Ok(())
}

pub fn get_user_investment_strategies(user_id: &str) -> Result<Vec<InvestmentStrategy>, AuthError> {
    let conn = get_connection_from_pool()?;

    let mut stmt = conn.prepare(
        "SELECT id, user_id, name, description, strategy_type, parameters, created_at, updated_at
         FROM investment_strategies
         WHERE user_id = ?1
         ORDER BY name",
    )?;

    let strategies = stmt
        .query_map(params![user_id], |row| {
            Ok(InvestmentStrategy {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                strategy_type: row.get(4)?,
                parameters: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            error!("Failed to fetch user investment strategies: {}", e);
            AuthError::DatabaseError(format!("获取用户投资策略失败: {}", e))
        })?;

    Ok(strategies)
}

pub fn apply_strategy(
    user_id: &str,
    strategy_id: i64,
    asset_id: i64,
) -> Result<StrategyApplication, AuthError> {
    let conn = get_connection_from_pool()?;
    let now = Utc::now().timestamp();

    // 检查策略是否存在且属于该用户
    let strategy_exists: bool = conn
        .query_row(
            "SELECT 1 FROM investment_strategies WHERE id = ?1 AND user_id = ?2",
            params![strategy_id, user_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !strategy_exists {
        return Err(AuthError::InvalidCredentials(
            "策略不存在或无权限".to_string(),
        ));
    }

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

    // 检查是否已应用该策略
    let application_exists: bool = conn
        .query_row(
            "SELECT 1 FROM strategy_applications 
         WHERE strategy_id = ?1 AND asset_id = ?2",
            params![strategy_id, asset_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if application_exists {
        return Err(AuthError::InvalidCredentials("已应用该策略".to_string()));
    }

    // 应用策略
    conn.execute(
        "INSERT INTO strategy_applications (user_id, strategy_id, asset_id, is_active, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            user_id,
            strategy_id,
            asset_id,
            true,
            now,
            now
        ],
    )?;

    let application_id = conn.last_insert_rowid();

    // 获取策略和资产信息
    let strategy_name: String = conn.query_row(
        "SELECT name FROM investment_strategies WHERE id = ?1",
        params![strategy_id],
        |row| row.get(0),
    )?;

    let (asset_name, asset_code): (String, String) = conn.query_row(
        "SELECT name, code FROM assets WHERE id = ?1",
        params![asset_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    let application = StrategyApplication {
        id: application_id,
        user_id: user_id.to_string(),
        strategy_id,
        strategy_name: strategy_name.clone(),
        asset_id,
        asset_name: asset_name.clone(),
        asset_code,
        is_active: true,
        created_at: now,
        updated_at: now,
    };

    info!(
        "Strategy applied: {} to asset: {} for user: {}",
        strategy_name, asset_name, user_id
    );
    Ok(application)
}

pub fn remove_strategy_application(id: i64, user_id: &str) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;

    // 检查应用是否存在且属于该用户
    let application_exists: bool = conn
        .query_row(
            "SELECT 1 FROM strategy_applications WHERE id = ?1 AND user_id = ?2",
            params![id, user_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !application_exists {
        return Err(AuthError::InvalidCredentials(
            "策略应用不存在或无权限".to_string(),
        ));
    }

    // 删除应用
    conn.execute(
        "DELETE FROM strategy_applications WHERE id = ?1",
        params![id],
    )?;

    info!("Strategy application removed: {} for user: {}", id, user_id);
    Ok(())
}

pub fn get_user_strategy_applications(
    user_id: &str,
    asset_id: Option<i64>,
) -> Result<Vec<StrategyApplication>, AuthError> {
    let conn = get_connection_from_pool()?;

    let mut query = if let Some(a_id) = asset_id {
        conn.prepare(
            "SELECT sa.id, sa.user_id, sa.strategy_id, s.name, sa.asset_id, a.name, a.code, 
                    sa.is_active, sa.created_at, sa.updated_at
             FROM strategy_applications sa
             JOIN investment_strategies s ON sa.strategy_id = s.id
             JOIN assets a ON sa.asset_id = a.id
             WHERE sa.user_id = ?1 AND sa.asset_id = ?2
             ORDER BY s.name",
        )?
    } else {
        conn.prepare(
            "SELECT sa.id, sa.user_id, sa.strategy_id, s.name, sa.asset_id, a.name, a.code, 
                    sa.is_active, sa.created_at, sa.updated_at
             FROM strategy_applications sa
             JOIN investment_strategies s ON sa.strategy_id = s.id
             JOIN assets a ON sa.asset_id = a.id
             WHERE sa.user_id = ?1
             ORDER BY s.name",
        )?
    };

    let applications = if let Some(a_id) = asset_id {
        query
            .query_map(params![user_id, a_id], |row| {
                Ok(StrategyApplication {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    strategy_id: row.get(2)?,
                    strategy_name: row.get(3)?,
                    asset_id: row.get(4)?,
                    asset_name: row.get(5)?,
                    asset_code: row.get(6)?,
                    is_active: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user strategy applications: {}", e);
                AuthError::DatabaseError(format!("获取用户策略应用失败: {}", e))
            })?
    } else {
        query
            .query_map(params![user_id], |row| {
                Ok(StrategyApplication {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    strategy_id: row.get(2)?,
                    strategy_name: row.get(3)?,
                    asset_id: row.get(4)?,
                    asset_name: row.get(5)?,
                    asset_code: row.get(6)?,
                    is_active: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Failed to fetch user strategy applications: {}", e);
                AuthError::DatabaseError(format!("获取用户策略应用失败: {}", e))
            })?
    };

    Ok(applications)
}

pub fn backtest_strategy(
    user_id: &str,
    strategy_id: i64,
    asset_id: i64,
    start_date: i64,
    end_date: i64,
) -> Result<BacktestResult, AuthError> {
    let conn = get_connection_from_pool()?;

    // 检查策略是否存在且属于该用户
    let (strategy_type, parameters): (String, String) = conn.query_row(
        "SELECT strategy_type, parameters FROM investment_strategies WHERE id = ?1 AND user_id = ?2",
        params![strategy_id, user_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|_| AuthError::InvalidCredentials("策略不存在或无权限".to_string()))?;

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

    // 获取历史价格数据
    let mut stmt = conn.prepare(
        "SELECT date, close_price
         FROM price_history
         WHERE asset_id = ?1 AND date >= ?2 AND date <= ?3
         ORDER BY date",
    )?;

    let price_data = stmt
        .query_map(params![asset_id, start_date, end_date], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, f64>(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            error!("Failed to fetch price history: {}", e);
            AuthError::DatabaseError(format!("获取历史价格数据失败: {}", e))
        })?;

    if price_data.is_empty() {
        return Err(AuthError::InvalidCredentials(
            "所选时间范围内没有历史价格数据".to_string(),
        ));
    }

    // 根据策略类型执行回测
    let backtest_result = match strategy_type.as_str() {
        "MACD" => backtest_macd_strategy(asset_id, &price_data, &parameters)?,
        "RSI" => backtest_rsi_strategy(asset_id, &price_data, &parameters)?,
        "MOVING_AVERAGE" => backtest_ma_strategy(asset_id, &price_data, &parameters)?,
        "CUSTOM" => backtest_custom_strategy(asset_id, &price_data, &parameters)?,
        _ => {
            return Err(AuthError::InvalidCredentials(
                "不支持的策略类型".to_string(),
            ))
        }
    };

    info!(
        "Strategy backtest completed for strategy: {} on asset: {}",
        strategy_id, asset_id
    );
    Ok(backtest_result)
}

fn backtest_macd_strategy(
    asset_id: i64,
    price_data: &[(i64, f64)],
    parameters: &str,
) -> Result<BacktestResult, AuthError> {
    // 解析参数
    let params: Value = serde_json::from_str(parameters)
        .map_err(|e| AuthError::InvalidCredentials(format!("无效的策略参数: {}", e)))?;

    let fast_period = params["fast_period"].as_u64().unwrap_or(12) as usize;
    let slow_period = params["slow_period"].as_u64().unwrap_or(26) as usize;
    let signal_period = params["signal_period"].as_u64().unwrap_or(9) as usize;
    let initial_investment = params["initial_investment"].as_f64().unwrap_or(10000.0);

    if price_data.len() < slow_period + signal_period {
        return Err(AuthError::InvalidCredentials(
            "历史数据不足，无法执行MACD策略回测".to_string(),
        ));
    }

    // 提取价格序列
    let prices: Vec<f64> = price_data.iter().map(|(_, price)| *price).collect();
    let dates: Vec<i64> = price_data.iter().map(|(date, _)| *date).collect();

    // 计算MACD
    let (macd_line, signal_line, _) =
        calculate_macd(&prices, fast_period, slow_period, signal_period);

    // 模拟交易
    let mut cash = initial_investment;
    let mut shares = 0.0;
    let mut transactions = Vec::new();
    let mut performance_data = Vec::new();

    // 记录初始状态
    performance_data.push(PerformancePoint {
        date: dates[0],
        value: cash,
        benchmark_value: Some(initial_investment),
    });

    for i in (slow_period + signal_period)..prices.len() {
        let current_price = prices[i];
        let current_date = dates[i];
        let macd_value = macd_line[i - 1];
        let signal_value = signal_line[i - 1];
        let prev_macd_value = macd_line[i - 2];
        let prev_signal_value = signal_line[i - 2];

        // MACD穿越信号线向上（买入信号）
        if prev_macd_value < prev_signal_value && macd_value > signal_value && cash > 0.0 {
            let amount = cash / current_price;
            shares += amount;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "BUY".to_string(),
                amount,
                price: current_price,
                total_cost: cash,
                transaction_date: current_date,
                notes: Some("MACD策略回测买入".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            cash = 0.0;
        }
        // MACD穿越信号线向下（卖出信号）
        else if prev_macd_value > prev_signal_value && macd_value < signal_value && shares > 0.0 {
            let value = shares * current_price;
            cash += value;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "SELL".to_string(),
                amount: shares,
                price: current_price,
                total_cost: value,
                transaction_date: current_date,
                notes: Some("MACD策略回测卖出".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            shares = 0.0;
        }

        // 记录每日表现
        let portfolio_value = cash + shares * current_price;
        let benchmark_value =
            initial_investment * (current_price / prices[slow_period + signal_period]);

        performance_data.push(PerformancePoint {
            date: current_date,
            value: portfolio_value,
            benchmark_value: Some(benchmark_value),
        });
    }

    // 计算最终结果
    let final_value = cash + shares * prices.last().unwrap();
    let total_return = (final_value - initial_investment) / initial_investment * 100.0;

    // 计算年化收益率
    let days = (dates.last().unwrap() - dates[slow_period + signal_period]) / 86400; // 秒转天
    let years = days as f64 / 365.0;
    let annualized_return = (final_value / initial_investment).powf(1.0 / years) - 1.0;

    // 计算最大回撤
    let mut max_drawdown = 0.0;
    let mut peak = performance_data[0].value;

    for point in &performance_data {
        if point.value > peak {
            peak = point.value;
        } else {
            let drawdown = (peak - point.value) / peak;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }
    }

    Ok(BacktestResult {
        initial_investment,
        final_value,
        total_return,
        annualized_return: annualized_return * 100.0,
        max_drawdown: max_drawdown * 100.0,
        transactions,
        performance_data,
    })
}

fn backtest_rsi_strategy(
    asset_id: i64,
    price_data: &[(i64, f64)],
    parameters: &str,
) -> Result<BacktestResult, AuthError> {
    // 解析参数
    let params: Value = serde_json::from_str(parameters)
        .map_err(|e| AuthError::InvalidCredentials(format!("无效的策略参数: {}", e)))?;

    let period = params["period"].as_u64().unwrap_or(14) as usize;
    let oversold = params["oversold"].as_f64().unwrap_or(30.0);
    let overbought = params["overbought"].as_f64().unwrap_or(70.0);
    let initial_investment = params["initial_investment"].as_f64().unwrap_or(10000.0);

    if price_data.len() < period + 1 {
        return Err(AuthError::InvalidCredentials(
            "历史数据不足，无法执行RSI策略回测".to_string(),
        ));
    }

    // 提取价格序列
    let prices: Vec<f64> = price_data.iter().map(|(_, price)| *price).collect();
    let dates: Vec<i64> = price_data.iter().map(|(date, _)| *date).collect();

    // 计算RSI
    let rsi = calculate_rsi(&prices, period);

    // 模拟交易
    let mut cash = initial_investment;
    let mut shares = 0.0;
    let mut transactions = Vec::new();
    let mut performance_data = Vec::new();

    // 记录初始状态
    performance_data.push(PerformancePoint {
        date: dates[0],
        value: cash,
        benchmark_value: Some(initial_investment),
    });

    for i in (period + 1)..prices.len() {
        let current_price = prices[i];
        let current_date = dates[i];
        let current_rsi = rsi[i - 1];
        let prev_rsi = rsi[i - 2];

        // RSI从超卖区域回升（买入信号）
        if prev_rsi < oversold && current_rsi > oversold && cash > 0.0 {
            let amount = cash / current_price;
            shares += amount;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "BUY".to_string(),
                amount,
                price: current_price,
                total_cost: cash,
                transaction_date: current_date,
                notes: Some("RSI策略回测买入".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            cash = 0.0;
        }
        // RSI从超买区域回落（卖出信号）
        else if prev_rsi > overbought && current_rsi < overbought && shares > 0.0 {
            let value = shares * current_price;
            cash += value;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "SELL".to_string(),
                amount: shares,
                price: current_price,
                total_cost: value,
                transaction_date: current_date,
                notes: Some("RSI策略回测卖出".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            shares = 0.0;
        }

        // 记录每日表现
        let portfolio_value = cash + shares * current_price;
        let benchmark_value = initial_investment * (current_price / prices[period + 1]);

        performance_data.push(PerformancePoint {
            date: current_date,
            value: portfolio_value,
            benchmark_value: Some(benchmark_value),
        });
    }

    // 计算最终结果
    let final_value = cash + shares * prices.last().unwrap();
    let total_return = (final_value - initial_investment) / initial_investment * 100.0;

    // 计算年化收益率
    let days = (dates.last().unwrap() - dates[period + 1]) / 86400; // 秒转天
    let years = days as f64 / 365.0;
    let annualized_return = (final_value / initial_investment).powf(1.0 / years) - 1.0;

    // 计算最大回撤
    let mut max_drawdown = 0.0;
    let mut peak = performance_data[0].value;

    for point in &performance_data {
        if point.value > peak {
            peak = point.value;
        } else {
            let drawdown = (peak - point.value) / peak;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }
    }

    Ok(BacktestResult {
        initial_investment,
        final_value,
        total_return,
        annualized_return: annualized_return * 100.0,
        max_drawdown: max_drawdown * 100.0,
        transactions,
        performance_data,
    })
}

fn backtest_ma_strategy(
    asset_id: i64,
    price_data: &[(i64, f64)],
    parameters: &str,
) -> Result<BacktestResult, AuthError> {
    // 解析参数
    let params: Value = serde_json::from_str(parameters)
        .map_err(|e| AuthError::InvalidCredentials(format!("无效的策略参数: {}", e)))?;

    let short_period = params["short_period"].as_u64().unwrap_or(5) as usize;
    let long_period = params["long_period"].as_u64().unwrap_or(20) as usize;
    let initial_investment = params["initial_investment"].as_f64().unwrap_or(10000.0);

    if price_data.len() < long_period {
        return Err(AuthError::InvalidCredentials(
            "历史数据不足，无法执行均线策略回测".to_string(),
        ));
    }

    // 提取价格序列
    let prices: Vec<f64> = price_data.iter().map(|(_, price)| *price).collect();
    let dates: Vec<i64> = price_data.iter().map(|(date, _)| *date).collect();

    // 计算移动平均线
    let short_ma = calculate_ma(&prices, short_period);
    let long_ma = calculate_ma(&prices, long_period);

    // 模拟交易
    let mut cash = initial_investment;
    let mut shares = 0.0;
    let mut transactions = Vec::new();
    let mut performance_data = Vec::new();

    // 记录初始状态
    performance_data.push(PerformancePoint {
        date: dates[0],
        value: cash,
        benchmark_value: Some(initial_investment),
    });

    for i in long_period..prices.len() {
        let current_price = prices[i];
        let current_date = dates[i];
        let short_ma_value = short_ma[i - 1];
        let long_ma_value = long_ma[i - 1];
        let prev_short_ma_value = short_ma[i - 2];
        let prev_long_ma_value = long_ma[i - 2];

        // 短期均线上穿长期均线（金叉，买入信号）
        if prev_short_ma_value < prev_long_ma_value && short_ma_value > long_ma_value && cash > 0.0
        {
            let amount = cash / current_price;
            shares += amount;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "BUY".to_string(),
                amount,
                price: current_price,
                total_cost: cash,
                transaction_date: current_date,
                notes: Some("均线策略回测买入".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            cash = 0.0;
        }
        // 短期均线下穿长期均线（死叉，卖出信号）
        else if prev_short_ma_value > prev_long_ma_value
            && short_ma_value < long_ma_value
            && shares > 0.0
        {
            let value = shares * current_price;
            cash += value;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "SELL".to_string(),
                amount: shares,
                price: current_price,
                total_cost: value,
                transaction_date: current_date,
                notes: Some("均线策略回测卖出".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            shares = 0.0;
        }

        // 记录每日表现
        let portfolio_value = cash + shares * current_price;
        let benchmark_value = initial_investment * (current_price / prices[long_period]);

        performance_data.push(PerformancePoint {
            date: current_date,
            value: portfolio_value,
            benchmark_value: Some(benchmark_value),
        });
    }

    // 计算最终结果
    let final_value = cash + shares * prices.last().unwrap();
    let total_return = (final_value - initial_investment) / initial_investment * 100.0;

    // 计算年化收益率
    let days = (dates.last().unwrap() - dates[long_period]) / 86400; // 秒转天
    let years = days as f64 / 365.0;
    let annualized_return = (final_value / initial_investment).powf(1.0 / years) - 1.0;

    // 计算最大回撤
    let mut max_drawdown = 0.0;
    let mut peak = performance_data[0].value;

    for point in &performance_data {
        if point.value > peak {
            peak = point.value;
        } else {
            let drawdown = (peak - point.value) / peak;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }
    }

    Ok(BacktestResult {
        initial_investment,
        final_value,
        total_return,
        annualized_return: annualized_return * 100.0,
        max_drawdown: max_drawdown * 100.0,
        transactions,
        performance_data,
    })
}

fn backtest_custom_strategy(
    asset_id: i64,
    price_data: &[(i64, f64)],
    parameters: &str,
) -> Result<BacktestResult, AuthError> {
    // 自定义策略的回测实现
    // 这里只是一个简单的示例，实际应用中可以根据需求扩展

    // 解析参数
    let params: Value = serde_json::from_str(parameters)
        .map_err(|e| AuthError::InvalidCredentials(format!("无效的策略参数: {}", e)))?;

    let initial_investment = params["initial_investment"].as_f64().unwrap_or(10000.0);
    let buy_threshold = params["buy_threshold"].as_f64().unwrap_or(-0.05); // 价格下跌5%买入
    let sell_threshold = params["sell_threshold"].as_f64().unwrap_or(0.08); // 价格上涨8%卖出

    if price_data.len() < 2 {
        return Err(AuthError::InvalidCredentials(
            "历史数据不足，无法执行自定义策略回测".to_string(),
        ));
    }

    // 提取价格序列
    let prices: Vec<f64> = price_data.iter().map(|(_, price)| *price).collect();
    let dates: Vec<i64> = price_data.iter().map(|(date, _)| *date).collect();

    // 模拟交易
    let mut cash = initial_investment;
    let mut shares = 0.0;
    let mut transactions = Vec::new();
    let mut performance_data = Vec::new();
    let mut last_buy_price = 0.0;
    let mut last_sell_price = 0.0;

    // 记录初始状态
    performance_data.push(PerformancePoint {
        date: dates[0],
        value: cash,
        benchmark_value: Some(initial_investment),
    });

    for i in 1..prices.len() {
        let current_price = prices[i];
        let current_date = dates[i];
        let prev_price = prices[i - 1];
        let price_change = (current_price - prev_price) / prev_price;

        // 价格下跌超过阈值（买入信号）
        if price_change <= buy_threshold && cash > 0.0 {
            let amount = cash / current_price;
            shares += amount;
            last_buy_price = current_price;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "BUY".to_string(),
                amount,
                price: current_price,
                total_cost: cash,
                transaction_date: current_date,
                notes: Some("自定义策略回测买入".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            cash = 0.0;
        }
        // 价格上涨超过阈值（卖出信号）
        else if last_buy_price > 0.0
            && (current_price - last_buy_price) / last_buy_price >= sell_threshold
            && shares > 0.0
        {
            let value = shares * current_price;
            cash += value;
            last_sell_price = current_price;

            let transaction = Transaction {
                id: 0, // 模拟ID
                user_id: "backtest".to_string(),
                asset_id,
                asset_name: "Backtest Asset".to_string(),
                asset_code: "BACKTEST".to_string(),
                transaction_type: "SELL".to_string(),
                amount: shares,
                price: current_price,
                total_cost: value,
                transaction_date: current_date,
                notes: Some("自定义策略回测卖出".to_string()),
                created_at: 0,
            };

            transactions.push(transaction);
            shares = 0.0;
        }

        // 记录每日表现
        let portfolio_value = cash + shares * current_price;
        let benchmark_value = initial_investment * (current_price / prices[0]);

        performance_data.push(PerformancePoint {
            date: current_date,
            value: portfolio_value,
            benchmark_value: Some(benchmark_value),
        });
    }

    // 计算最终结果
    let final_value = cash + shares * prices.last().unwrap();
    let total_return = (final_value - initial_investment) / initial_investment * 100.0;

    // 计算年化收益率
    let days = (dates.last().unwrap() - dates[0]) / 86400; // 秒转天
    let years = days as f64 / 365.0;
    let annualized_return = (final_value / initial_investment).powf(1.0 / years) - 1.0;

    // 计算最大回撤
    let mut max_drawdown = 0.0;
    let mut peak = performance_data[0].value;

    for point in &performance_data {
        if point.value > peak {
            peak = point.value;
        } else {
            let drawdown = (peak - point.value) / peak;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }
    }

    Ok(BacktestResult {
        initial_investment,
        final_value,
        total_return,
        annualized_return: annualized_return * 100.0,
        max_drawdown: max_drawdown * 100.0,
        transactions,
        performance_data,
    })
}

// 计算MACD指标
fn calculate_macd(
    prices: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // 计算快速EMA
    let fast_ema = calculate_ema(prices, fast_period);

    // 计算慢速EMA
    let slow_ema = calculate_ema(prices, slow_period);

    // 计算MACD线
    let mut macd_line = Vec::with_capacity(prices.len());
    for i in 0..prices.len() {
        if i < slow_period - 1 {
            macd_line.push(0.0);
        } else {
            macd_line.push(fast_ema[i] - slow_ema[i]);
        }
    }

    // 计算信号线（MACD的EMA）
    let signal_line = calculate_ema(&macd_line, signal_period);

    // 计算柱状图
    let mut histogram = Vec::with_capacity(prices.len());
    for i in 0..prices.len() {
        if i < slow_period + signal_period - 2 {
            histogram.push(0.0);
        } else {
            histogram.push(macd_line[i] - signal_line[i]);
        }
    }

    (macd_line, signal_line, histogram)
}

// 计算RSI指标
fn calculate_rsi(prices: &[f64], period: usize) -> Vec<f64> {
    if prices.len() <= period {
        return vec![50.0; prices.len()];
    }

    let mut rsi = Vec::with_capacity(prices.len());
    let mut gains = Vec::with_capacity(prices.len());
    let mut losses = Vec::with_capacity(prices.len());

    // 计算价格变化
    for i in 1..prices.len() {
        let change = prices[i] - prices[i - 1];
        gains.push(if change > 0.0 { change } else { 0.0 });
        losses.push(if change < 0.0 { -change } else { 0.0 });
    }

    // 前period个RSI值设为50（中性）
    for _ in 0..period {
        rsi.push(50.0);
    }

    // 计算第一个RSI值
    let avg_gain = gains[0..period].iter().sum::<f64>() / period as f64;
    let avg_loss = losses[0..period].iter().sum::<f64>() / period as f64;

    if avg_loss == 0.0 {
        rsi.push(100.0);
    } else {
        let rs = avg_gain / avg_loss;
        rsi.push(100.0 - (100.0 / (1.0 + rs)));
    }

    // 计算剩余的RSI值
    let mut avg_gain = avg_gain;
    let mut avg_loss = avg_loss;

    for i in period..gains.len() {
        avg_gain = (avg_gain * (period - 1) as f64 + gains[i]) / period as f64;
        avg_loss = (avg_loss * (period - 1) as f64 + losses[i]) / period as f64;

        if avg_loss == 0.0 {
            rsi.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            rsi.push(100.0 - (100.0 / (1.0 + rs)));
        }
    }

    rsi
}

// 计算移动平均线
fn calculate_ma(prices: &[f64], period: usize) -> Vec<f64> {
    let mut ma = Vec::with_capacity(prices.len());

    // 前period-1个值设为0
    for _ in 0..period - 1 {
        ma.push(0.0);
    }

    // 计算移动平均
    for i in period - 1..prices.len() {
        let sum: f64 = prices[i - (period - 1)..=i].iter().sum();
        ma.push(sum / period as f64);
    }

    ma
}

// 计算指数移动平均线
fn calculate_ema(prices: &[f64], period: usize) -> Vec<f64> {
    let mut ema = Vec::with_capacity(prices.len());
    let multiplier = 2.0 / (period as f64 + 1.0);

    // 第一个EMA值使用SMA
    let mut sum = 0.0;
    for i in 0..period {
        sum += prices[i];
    }
    let first_ema = sum / period as f64;

    // 前period-1个值设为0
    for _ in 0..period - 1 {
        ema.push(0.0);
    }

    ema.push(first_ema);

    // 计算剩余的EMA值
    for i in period..prices.len() {
        let new_ema = (prices[i] - ema[i - 1]) * multiplier + ema[i - 1];
        ema.push(new_ema);
    }

    ema
}
