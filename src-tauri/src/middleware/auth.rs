use crate::config::Config;
use crate::database::get_connection_from_pool;
use crate::error::auth::AuthError;
use crate::models::User;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error, info};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{command, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,         // user_id
    pub username: String, // 用户名
    pub email: String,    // 邮箱
    pub exp: usize,       // 过期时间
    pub iat: usize,       // 签发时间
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub username: String,
    pub email: String,
}

pub struct AuthState {
    pub jwt_secret: String,
}

impl AuthState {
    pub fn new(secret: &str) -> Self {
        Self {
            jwt_secret: secret.to_string(),
        }
    }
}

// 从用户模型生成 JWT token
pub fn generate_jwt_token(user: &User, secret: &str) -> Result<String, AuthError> {
    let config = Config::get();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(config.auth.token_expiry_hours as i64))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreationError())
}

// 验证 JWT token
pub fn verify_jwt_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AuthError::InvalidToken("令牌已过期".to_string())
        }
        _ => AuthError::InvalidToken("无效的令牌".to_string()),
    })
}

// 从请求上下文中获取当前用户
#[command]
pub fn get_current_user(
    token: Option<String>,
    auth_state: State<Arc<AuthState>>,
) -> Result<AuthUser, AuthError> {
    match token {
        Some(token) => {
            // 首先尝试验证 JWT 令牌
            match verify_jwt_token(&token, &auth_state.jwt_secret) {
                Ok(claims) => {
                    // JWT 验证成功，从数据库获取最新的用户信息
                    let conn = get_connection_from_pool()?;
                    let user = conn
                        .query_row(
                            "SELECT id, username, email FROM users WHERE id = ?1",
                            params![claims.sub],
                            |row| {
                                Ok(AuthUser {
                                    user_id: row.get(0)?,
                                    username: row.get(1)?,
                                    email: row.get(2)?,
                                })
                            },
                        )
                        .map_err(|_| AuthError::UserNotFound("用户不存在".to_string()))?;

                    Ok(user)
                }
                Err(_) => Err(AuthError::InvalidSession("会话不存在或已过期".to_string())),
            }
        }
        None => Err(AuthError::InvalidToken("未提供认证令牌".to_string())),
    }
}

// 中间件：验证用户是否已认证
pub fn require_auth<F, R>(
    token: Option<String>,
    auth_state: State<Arc<AuthState>>,
    callback: F,
) -> Result<R, AuthError>
where
    F: FnOnce(AuthUser) -> Result<R, AuthError>,
{
    let user = get_current_user(token, auth_state)?;
    callback(user)
}

// 中间件：验证用户是否具有特定角色
pub fn require_role<F, R>(
    token: Option<String>,
    role: &str,
    auth_state: State<Arc<AuthState>>,
    callback: F,
) -> Result<R, AuthError>
where
    F: FnOnce(AuthUser) -> Result<R, AuthError>,
{
    let user = get_current_user(token.clone(), auth_state.clone())?;

    // 检查用户角色
    let conn = get_connection_from_pool()?;
    let has_role: bool = conn
        .query_row(
            "SELECT 1 FROM user_roles ur 
             JOIN roles r ON ur.role_id = r.id 
             WHERE ur.user_id = ?1 AND r.name = ?2",
            params![user.user_id, role],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !has_role {
        // 检查是否是管理员
        let is_admin: bool = conn
            .query_row(
                "SELECT 1 FROM user_roles ur 
                 JOIN roles r ON ur.role_id = r.id 
                 WHERE ur.user_id = ?1 AND r.name = 'admin'",
                params![user.user_id],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !is_admin {
            return Err(AuthError::InvalidCredentials(
                "用户没有所需的权限".to_string(),
            ));
        }
    }

    callback(user)
}

// 中间件：验证用户是否是管理员
pub fn require_admin<F, R>(
    token: Option<String>,
    auth_state: State<Arc<AuthState>>,
    callback: F,
) -> Result<R, AuthError>
where
    F: FnOnce(AuthUser) -> Result<R, AuthError>,
{
    require_role(token, "admin", auth_state, callback)
}

// 将 JWT 令牌存储到数据库中
pub fn store_jwt_token(user_id: i64, jwt_token: &str) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;
    let config = Config::get();
    let expiry = Utc::now() + Duration::hours(config.auth.token_expiry_hours as i64);

    conn.execute(
        "INSERT INTO jwt_tokens (user_id, token, expires_at, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            user_id,
            jwt_token,
            expiry.timestamp(),
            Utc::now().timestamp()
        ],
    )?;

    Ok(())
}

// 撤销 JWT 令牌
pub fn revoke_jwt_token(token: &str) -> Result<(), AuthError> {
    let conn = get_connection_from_pool()?;

    conn.execute("DELETE FROM jwt_tokens WHERE token = ?1", params![token])?;

    Ok(())
}

// 检查 JWT 令牌是否被撤销
pub fn is_jwt_token_revoked(token: &str) -> Result<bool, AuthError> {
    let conn = get_connection_from_pool()?;

    let is_revoked: bool = conn
        .query_row(
            "SELECT 1 FROM revoked_tokens WHERE token = ?1",
            params![token],
            |_| Ok(true),
        )
        .unwrap_or(false);

    Ok(is_revoked)
}
