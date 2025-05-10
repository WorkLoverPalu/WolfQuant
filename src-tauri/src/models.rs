use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub login_attempts: u32,
    #[serde(skip_serializing)]
    pub locked_until: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            email_verified: false,
            created_at: now,
            updated_at: now,
            last_login: None,
            login_attempts: 0,
            locked_until: None,
        }
    }
    
    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            return locked_until > Utc::now();
        }
        false
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetToken {
    pub token: String,
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
}

impl PasswordResetToken {
    pub fn new(user_id: String, expiry_hours: u64) -> Self {
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::hours(expiry_hours as i64);
        
        Self {
            token,
            user_id,
            expires_at,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailVerificationCode {
    pub code: String,
    pub user_id: String,
    pub email: String,
    pub expires_at: DateTime<Utc>,
}

impl EmailVerificationCode {
    pub fn new(user_id: String, email: String, expiry_minutes: u64) -> Self {
        // 生成6位数字验证码
        let code = format!("{:06}", rand::random::<u32>() % 1000000);
        let expires_at = Utc::now() + Duration::minutes(expiry_minutes as i64);
        
        Self {
            code,
            user_id,
            email,
            expires_at,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl Session {
    pub fn new(user_id: String, expiry_hours: u64, ip_address: Option<String>, user_agent: Option<String>) -> Self {
        let now = Utc::now();
        let token = Uuid::new_v4().to_string();
        let expires_at = now + Duration::hours(expiry_hours as i64);
        
        Self {
            token,
            user_id,
            created_at: now,
            expires_at,
            ip_address,
            user_agent,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

// 请求和响应结构
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub code: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: User,
}