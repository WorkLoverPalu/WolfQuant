use crate::config::Config;
use crate::database::get_connection_from_pool;
use crate::error::auth::AuthError;
use crate::models::User;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error, info};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tauri::ipc::{InvokeBody,Invoke};
use tauri::{command, AppHandle, Manager, State};

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
#[derive(Deserialize)]
struct TokenPayload {
    token: String,
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
    .map_err(|_| AuthError::TokenCreationError("无效的令牌".to_string()))
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

// 从请求上下文中获取当前用户
pub fn get_current_user(token: &str, auth_state: &AuthState) -> Result<AuthUser, AuthError> {
    // 首先尝试验证 JWT 令牌
    match verify_jwt_token(token, &auth_state.jwt_secret) {
        Ok(claims) => {
            // 检查令牌是否被撤销
            if is_jwt_token_revoked(token)? {
                return Err(AuthError::InvalidToken("令牌已被撤销".to_string()));
            }

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
        Err(_) => {
            // JWT 验证失败，尝试使用会话令牌
            let conn = get_connection_from_pool()?;
            let user = conn
                .query_row(
                    "SELECT u.id, u.username, u.email 
                 FROM sessions s
                 JOIN users u ON s.user_id = u.id
                 WHERE s.token = ?1 AND s.expires_at > ?2",
                    params![token, Utc::now().timestamp()],
                    |row| {
                        Ok(AuthUser {
                            user_id: row.get(0)?,
                            username: row.get(1)?,
                            email: row.get(2)?,
                        })
                    },
                )
                .map_err(|_| AuthError::InvalidSession("会话不存在或已过期".to_string()))?;

            Ok(user)
        }
    }
}

// 检查用户是否具有特定角色
pub fn check_user_role(user_id: i64, role: &str) -> Result<bool, AuthError> {
    let conn = get_connection_from_pool()?;

    // 检查用户角色
    let has_role: bool = conn
        .query_row(
            "SELECT 1 FROM user_roles ur 
             JOIN roles r ON ur.role_id = r.id 
             WHERE ur.user_id = ?1 AND r.name = ?2",
            params![user_id, role],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if has_role {
        return Ok(true);
    }

    // 检查是否是管理员
    let is_admin: bool = conn
        .query_row(
            "SELECT 1 FROM user_roles ur 
             JOIN roles r ON ur.role_id = r.id 
             WHERE ur.user_id = ?1 AND r.name = 'admin'",
            params![user_id],
            |_| Ok(true),
        )
        .unwrap_or(false);

    Ok(is_admin)
}

// JWT 中间件
pub struct JwtMiddleware;

impl JwtMiddleware {
    // 创建中间件
    pub fn new() -> Self {
        Self {}
    }

    // 中间件处理函数
    pub fn middleware<F>(self, app: &AppHandle, invoke: Invoke, next: F)
    where
        F: FnOnce(Invoke) + Send + 'static,
    {
        let cmd = invoke.message.command();

        // 不需要认证的命令列表
        let public_commands = vec![
            "auth_login_command",
            "auth_register_command",
            "auth_send_verification_code_command",
            "auth_forgot_password_command",
            "auth_reset_password_command",
            "auth_verify_session_command", // 这个命令自己会验证令牌
        ];

        // 如果是公开命令，直接放行
        if public_commands.iter().any(|&c| c == cmd) {
            next(invoke);
            return;
        }

        // 获取 auth_state
        let auth_state = app.state::<Arc<AuthState>>();

        // 获取令牌
        let payload = invoke.message.payload();
        let token = match payload {
            InvokeBody::Json(value) => {
                if let Some(token) = value.get("token").and_then(|t| t.as_str()) {
                    token.to_string()
                } else {
                    // 没有提供令牌，拒绝请求
                    invoke.resolver.reject("未提供认证令牌");
                    return;
                }
            }
            _ => {
                // 没有提供 payload 或格式不正确，拒绝请求
                invoke.resolver.reject("未提供认证令牌");
                return;
            }
        };

        // 验证令牌
        match get_current_user(&token, &auth_state) {
            Ok(_) => {
                // 认证成功，继续处理请求
                next(invoke);
            }
            Err(err) => {
                // 认证失败，拒绝请求
                invoke.resolver.reject(&format!("{}", err));
            }
        }
    }
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

    conn.execute(
        "INSERT INTO revoked_tokens (token, revoked_at) VALUES (?1, ?2)",
        params![token, Utc::now().timestamp()],
    )?;

    conn.execute("DELETE FROM jwt_tokens WHERE token = ?1", params![token])?;

    Ok(())
}
