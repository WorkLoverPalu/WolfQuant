use crate::config::Config;
use crate::database::{execute_query, get_connection_from_pool,get_connection_from_pool};
use crate::error::auth::AuthError;
use crate::models::{PasswordResetToken, User};
use crate::utils::crypto::{generate_token, hash_password, verify_password};
use chrono::{Duration, Utc};
use log::{debug, error, info};
use rusqlite::{params, Connection, Result as SqlResult};

pub fn register_user(username: &str, email: &str, password: &str) -> Result<User, AuthError> {
    let config = Config::get();

    // 验证密码长度
    if password.len() < config.auth.min_password_length as usize {
        return Err(AuthError::InvalidPassword(format!(
            "密码长度不能少于{}个字符",
            config.auth.min_password_length
        )));
    }
    //从连接池获取数据库实例
    let conn = get_connection_from_pool()?;

    // 检查用户名是否已存在
    let username_exists: bool = conn
        .query_row(
            "SELECT 1 FROM users WHERE username = ?1 LIMIT 1",
            params![username],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if username_exists {
        return Err(AuthError::UserExists("用户名已被使用".to_string()));
    }

    // 检查邮箱是否已存在
    let email_exists: bool = conn
        .query_row(
            "SELECT 1 FROM users WHERE email = ?1 LIMIT 1",
            params![email],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if email_exists {
        return Err(AuthError::UserExists("邮箱已被注册".to_string()));
    }

    // 哈希密码
    let hashed_password = hash_password(password)?;

    // 创建用户
    conn.execute(
        "INSERT INTO users (username, email, password_hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            username,
            email,
            hashed_password,
            Utc::now().timestamp(),
            Utc::now().timestamp()
        ],
    )?;

    // 获取新创建的用户
    let user = conn.query_row(
        "SELECT id, username, email, created_at, updated_at FROM users WHERE username = ?1",
        params![username],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    )?;

    info!("User registered: {}", username);
    Ok(user)
}

pub fn login_user(username_or_email: &str, password: &str) -> Result<(User, String), AuthError> {
    let conn = get_connection_from_pool()?;
    println!("login_user: {}",username_or_email);
    // 查找用户
    let result = conn.query_row(
        "SELECT id, username, email, password_hash, created_at, updated_at FROM users 
         WHERE username = ?1 OR email = ?1",
        params![username_or_email],
        |row| {
            Ok((
                User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                },
                row.get::<_, String>(3)?,
            ))
        },
    );

    match result {
        Ok((user, hash)) => {
            // 验证密码
            if verify_password(password, &hash)? {
                // 生成会话令牌
                let token = generate_token();
                let config = Config::get();
                let expiry = Utc::now() + Duration::hours(config.auth.token_expiry_hours as i64);

                // 存储会话
                conn.execute(
                    "INSERT INTO sessions (user_id, token, expires_at, created_at) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        user.id,
                        &token,
                        expiry.timestamp(),
                        Utc::now().timestamp()
                    ],
                )?;

                info!("User logged in: {}", user.username);
                Ok((user, token))
            } else {
                Err(AuthError::InvalidCredentials("密码不正确".to_string()))
            }
        }
        Err(_) => Err(AuthError::InvalidCredentials(
            "用户名或邮箱不存在".to_string(),
        )),
    }
}

pub fn logout_user(user_id: &str, token: &str) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;

    // 删除会话
    let rows_affected = conn.execute(
        "DELETE FROM sessions WHERE user_id = ?1 AND token = ?2",
        params![user_id, token],
    )?;

    if rows_affected == 0 {
        return Err(AuthError::InvalidSession("会话不存在或已过期".to_string()));
    }

    info!("User logged out: {}", user_id);
    Ok(())
}

pub fn verify_session(token: &str) -> Result<User, AuthError> {
    let conn = get_connection_from_pool()?;

    // 查找会话
    let result = conn.query_row(
        "SELECT u.id, u.username, u.email, u.created_at, u.updated_at 
         FROM sessions s
         JOIN users u ON s.user_id = u.id
         WHERE s.token = ?1 AND s.expires_at > ?2",
        params![token, Utc::now().timestamp()],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    );

    match result {
        Ok(user) => {
            debug!("Session verified for user: {}", user.username);
            Ok(user)
        }
        Err(_) => Err(AuthError::InvalidSession("会话不存在或已过期".to_string())),
    }
}

pub fn create_password_reset_token(email: &str) -> Result<PasswordResetToken, AuthError> {
    let conn = get_connection_from_pool()?;

    // 检查邮箱是否存在
    let user_id: String = conn
        .query_row(
            "SELECT id FROM users WHERE email = ?1",
            params![email],
            |row| row.get(0),
        )
        .map_err(|_| AuthError::UserNotFound("该邮箱未注册".to_string()))?;

    // 生成令牌
    let token = generate_token();
    let config = Config::get();
    let expiry =
        Utc::now() + Duration::minutes(config.auth.password_reset_token_expiry_minutes as i64);

    // 删除该用户之前的重置令牌
    conn.execute(
        "DELETE FROM password_reset_tokens WHERE user_id = ?1",
        params![user_id],
    )?;

    // 存储新令牌
    conn.execute(
        "INSERT INTO password_reset_tokens (user_id, token, expires_at, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            user_id,
            &token,
            expiry.timestamp(),
            Utc::now().timestamp()
        ],
    )?;

    info!("Password reset token created for user ID: {}", user_id);
    Ok(PasswordResetToken {
        user_id,
        token,
        expires_at: expiry.timestamp(),
        created_at: Utc::now().timestamp(),
    })
}

pub fn reset_password(token: &str, new_password: &str) -> Result<(), AuthError> {
    let config = Config::get();

    // 验证密码长度
    if new_password.len() < config.auth.min_password_length as usize {
        return Err(AuthError::InvalidPassword(format!(
            "密码长度不能少于{}个字符",
            config.auth.min_password_length
        )));
    }

    let conn = get_connection_from_pool()?;

    // 查找令牌
    let user_id: String = conn
        .query_row(
            "SELECT user_id FROM password_reset_tokens WHERE token = ?1 AND expires_at > ?2",
            params![token, Utc::now().timestamp()],
            |row| row.get(0),
        )
        .map_err(|_| AuthError::InvalidToken("重置令牌无效或已过期".to_string()))?;

    // 哈希新密码
    let hashed_password = hash_password(new_password)?;

    // 更新密码
    conn.execute(
        "UPDATE users SET password_hash = ?1, updated_at = ?2 WHERE id = ?3",
        params![hashed_password, Utc::now().timestamp(), user_id],
    )?;

    // 删除令牌
    conn.execute(
        "DELETE FROM password_reset_tokens WHERE token = ?1",
        params![token],
    )?;

    // 删除所有会话，强制用户重新登录
    conn.execute("DELETE FROM sessions WHERE user_id = ?1", params![user_id])?;

    info!("Password reset successful for user ID: {}", user_id);
    Ok(())
}
