use serde::{Deserialize, Serialize};
use std::fmt;

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
}

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
        }
    }
}

impl std::error::Error for AuthError {}

impl From<rusqlite::Error> for AuthError {
    fn from(err: rusqlite::Error) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}

impl From<AuthError> for ErrorResponse {
    fn from(err: AuthError) -> Self {
        let (message, status) = match err {
            AuthError::UserExists(msg) => (msg, 409),
            AuthError::UserNotFound(msg) => (msg, 404),
            AuthError::InvalidCredentials(msg) => (msg, 401),
            AuthError::InvalidPassword(msg) => (msg, 400),
            AuthError::InvalidToken(msg) => (msg, 401),
            AuthError::InvalidSession(msg) => (msg, 401),
            AuthError::DatabaseError(msg) => (msg, 500),
            AuthError::CryptoError(msg) => (msg, 500),
            AuthError::InternalError(msg) => (msg, 500),
        };
        
        ErrorResponse {
            error: message,
            status,
        }
    }
}