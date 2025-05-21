use serde::{Deserialize, Serialize};
use std::{fmt, string};

#[derive(Debug)]
pub enum AuthError {
    UserExists(String),
    UserNotFound(String),
    InvalidCredentials(String),
    InvalidPassword(String),
    InvalidToken(String),
    InvalidSession(String),
    DatabaseError(String),
    CryptoError(String),
    InternalError(String),
    TokenCreationError(),
}

// 实现 Display trait 可以像字符串一样输出
impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::UserExists(msg) => write!(f, "User exists: {}", msg),
            AuthError::UserNotFound(msg) => write!(f, "User not found: {}", msg),
            AuthError::InvalidCredentials(msg) => write!(f, "Invalid credentials: {}", msg),
            AuthError::InvalidPassword(msg) => write!(f, "Invalid password: {}", msg),
            AuthError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AuthError::InvalidSession(msg) => write!(f, "Invalid session: {}", msg),
            AuthError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AuthError::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
            AuthError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AuthError::TokenCreationError() => write!(f, "Failed to create token"),
        }
    }
}

// 表明 AuthError 是一个标准的错误类型
impl std::error::Error for AuthError {}

// 实现 From trait，允许从 rusqlite::Error 自动转换为 AuthError
// 所有数据库错误都会被包装为 AuthError::DatabaseError
impl From<rusqlite::Error> for AuthError {
    fn from(err: rusqlite::Error) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u16,
}

// 从 AuthError 转换为 ErrorResponse
impl From<AuthError> for ErrorResponse {
    fn from(err: AuthError) -> Self {
        let (message, status) = match err {
            AuthError::UserExists(msg) => (msg, 409),   //用户已存在
            AuthError::UserNotFound(msg) => (msg, 404), //用户不存在
            AuthError::InvalidCredentials(msg) => (msg, 401), //认证失败
            AuthError::InvalidPassword(msg) => (msg, 400), //无效密码
            AuthError::InvalidToken(msg) => (msg, 401),
            AuthError::InvalidSession(msg) => (msg, 401),
            AuthError::DatabaseError(msg) => (msg, 500), //服务器错误
            AuthError::CryptoError(msg) => (msg, 500),
            AuthError::InternalError(msg) => (msg, 500),
            AuthError::TokenCreationError() => ("创建令牌失败".to_string(), 500),
        };

        ErrorResponse {
            message: message,
            status,
        }
    }
}
