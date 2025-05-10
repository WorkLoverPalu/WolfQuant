use log::error;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    
    #[error("Password hashing error: {0}")]
    PasswordHashError(String),
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Account locked. Try again later")]
    AccountLocked,
    
    #[error("Username already taken")]
    UsernameTaken,
    
    #[error("Email already registered")]
    EmailTaken,
    
    #[error("Invalid or expired token")]
    InvalidToken,
    
    #[error("Email verification required")]
    EmailVerificationRequired,
    
    #[error("Invalid verification code")]
    InvalidVerificationCode,
    
    #[error("Password too weak: {0}")]
    WeakPassword(String),
    
    #[error("Email sending failed: {0}")]
    EmailSendingFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub success: bool,
}

impl From<AuthError> for ErrorResponse {
    fn from(error: AuthError) -> Self {
        // 记录错误
        error!("Auth error: {}", error);
        
        ErrorResponse {
            error: error.to_string(),
            success: false,
        }
    }
}

impl From<String> for ErrorResponse {
    fn from(error: String) -> Self {
        // 记录错误
        error!("Error: {}", error);
        
        ErrorResponse {
            error,
            success: false,
        }
    }
}