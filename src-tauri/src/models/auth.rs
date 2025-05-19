use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub verification_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub user_id: i64,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendVerificationCodeRequest {
    pub email: String,
    pub purpose: String, // "register" 或 "reset_password"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
    pub verification_code: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub message: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetToken {
    pub user_id: i64,
    pub token: String,
    pub expires_at: i64,
    pub created_at: i64,
}
