use crate::config::Config;
use crate::database::{execute_query, get_connection_from_pool};
use crate::error::auth::AuthError;
use crate::middleware::auth::{generate_jwt_token, store_jwt_token, verify_jwt_token};
use crate::models::{PasswordResetToken, User};
use crate::services::verification::verify_code;
use crate::utils::crypto::{generate_token, hash_password, verify_password};
use chrono::{Duration, Utc};
use log::{debug, error, info};
use rusqlite::{params, Connection, Result as SqlResult};

pub fn register_user(
    username: &str,
    email: &str,
    password: &str,
    verification_code: &str,
) -> Result<User, AuthError> {
    let config = Config::get();

    // 验证密码长度
    if password.len() < config.auth.min_password_length as usize {
        return Err(AuthError::InvalidPassword(format!(
            "密码长度不能少于{}个字符",
            config.auth.min_password_length
        )));
    }
    // 验证邮箱验证码
    verify_code(email, verification_code, "register")?;

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
        "INSERT INTO users (username, email, password_hash, email_verified, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            username,
            email,
            hashed_password,
            1, // email_verified
            Utc::now().timestamp(),
            Utc::now().timestamp()
        ],
    )?;

    info!("INSERT INTO users");

    // 获取新创建的用户
    let user = conn.query_row(
        "SELECT id, username, email, email_verified, created_at, updated_at FROM users WHERE username = ?1",
        params![username],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                email_verified: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        },
    )?;

    info!("User registered: {}", username);
    Ok(user)
}

pub fn login_user(username_or_email: &str, password: &str) -> Result<(User, String), AuthError> {
    let conn = get_connection_from_pool()?;
     let config = Config::get();
    println!("login_user: {}", username_or_email);
    // 查找用户
    let result = conn.query_row(
        "SELECT id, username, email, password_hash, email_verified, created_at, updated_at FROM users 
         WHERE username = ?1 OR email = ?1",
        params![username_or_email],
        |row| {
            Ok((
                User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    email_verified: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                },
                row.get::<_, String>(3)?,
            ))
        },
    );

    match result {
        Ok((user, hash)) => {
            // 验证密码
            if verify_password(password, &hash)? {
                // 生成 JWT 令牌
                let jwt_token = generate_jwt_token(&user, &config.jwt_secret)?;

                // 存储 JWT 令牌（可选，用于撤销）
                store_jwt_token(user.id, &jwt_token)?;

                info!("User logged in: {}", user.username);

                // 返回 JWT 令牌而不是会话令牌
                Ok((user, jwt_token))
            } else {
                Err(AuthError::InvalidCredentials("密码不正确".to_string()))
            }
        }
        Err(_) => Err(AuthError::InvalidCredentials(
            "用户名或邮箱不存在".to_string(),
        )),
    }
}

pub fn logout_user(user_id: i64, token: &str) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;
    let config = Config::get();

    // 尝试验证 JWT 令牌
    match verify_jwt_token(token, &config.jwt_secret) {
        Ok(_claims) => {
            // 将 JWT 令牌添加到撤销列表
            conn.execute(
                "INSERT INTO revoked_tokens (token, revoked_at) VALUES (?1, ?2)",
                params![token, Utc::now().timestamp()],
            )?;

            // 删除 JWT 令牌记录
            conn.execute("DELETE FROM jwt_tokens WHERE token = ?1", params![token])?;
        }
        Err(_) => {
            return Err(AuthError::UserExists("退出登陆失败".to_string()));
        }
    }

    info!("User logged out: {}", user_id);
    Ok(())
}

pub fn verify_session(token: &str) -> Result<User, AuthError> {
    let conn = get_connection_from_pool()?;
    let config = Config::get();

    // 首先尝试验证 JWT 令牌
    match verify_jwt_token(token, &config.jwt_secret) {
        Ok(claims) => {
            // 检查令牌是否被撤销
            let is_revoked: bool = conn
                .query_row(
                    "SELECT 1 FROM revoked_tokens WHERE token = ?1",
                    params![token],
                    |_| Ok(true),
                )
                .unwrap_or(false);

            if is_revoked {
                return Err(AuthError::InvalidToken("令牌已被撤销".to_string()));
            }

            // 获取用户信息
            conn.query_row(
                "SELECT id, username, email, email_verified, created_at, updated_at FROM users WHERE id = ?1",
                params![claims.sub],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        email: row.get(2)?,
                        email_verified: row.get(3)?,
                        created_at: row.get(4)?,
                        updated_at: row.get(5)?,
                    })
                },
            )
            .map_err(|_| AuthError::UserNotFound("用户不存在".to_string()))
        }
        Err(_) => Err(AuthError::InvalidSession("会话不存在或已过期".to_string())),
    }
}

// 忘记密码
pub fn reset_password_with_code(
    email: &str,
    verification_code: &str,
    new_password: &str,
) -> Result<(), AuthError> {
    let config = Config::get();

    // 验证密码长度
    if new_password.len() < config.auth.min_password_length as usize {
        return Err(AuthError::InvalidPassword(format!(
            "密码长度不能少于{}个字符",
            config.auth.min_password_length
        )));
    }

    // 验证邮箱验证码
    verify_code(email, verification_code, "reset_password")?;

    let conn = get_connection_from_pool()?;

    // 检查邮箱是否存在
    let user_id: i64 = conn
        .query_row(
            "SELECT id FROM users WHERE email = ?1",
            params![email],
            |row| row.get(0),
        )
        .map_err(|_| AuthError::UserNotFound("该邮箱未注册".to_string()))?;

    // 哈希新密码
    let hashed_password = hash_password(new_password)?;

    // 更新密码
    conn.execute(
        "UPDATE users SET password_hash = ?1, updated_at = ?2 WHERE id = ?3",
        params![hashed_password, Utc::now().timestamp(), user_id],
    )?;

    // 撤销所有 JWT 令牌
    let tokens: Vec<String> = conn
        .prepare("SELECT token FROM jwt_tokens WHERE user_id = ?1")?
        .query_map(params![user_id], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;

    for token in tokens {
        conn.execute(
            "INSERT INTO revoked_tokens (token, revoked_at) VALUES (?1, ?2)",
            params![token, Utc::now().timestamp()],
        )?;

        conn.execute("DELETE FROM jwt_tokens WHERE token = ?1", params![token])?;
    }

    info!(
        "Password reset successful with verification code for user ID: {}",
        user_id
    );
    Ok(())
}

// 登陆时间重置密码
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
    let user_id: i64 = conn
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

    // 撤销所有 JWT 令牌
    let tokens: Vec<String> = conn
        .prepare("SELECT token FROM jwt_tokens WHERE user_id = ?1")?
        .query_map(params![user_id], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;

    for token in tokens {
        conn.execute(
            "INSERT INTO revoked_tokens (token, revoked_at) VALUES (?1, ?2)",
            params![token, Utc::now().timestamp()],
        )?;

        conn.execute("DELETE FROM jwt_tokens WHERE token = ?1", params![token])?;
    }

    info!("Password reset successful for user ID: {}", user_id);
    Ok(())
}
