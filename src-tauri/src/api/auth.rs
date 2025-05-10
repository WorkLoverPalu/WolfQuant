use crate::auth::{create_password_reset_token, login_user, logout_user, register_user, reset_password, verify_session};
use crate::config::Config;
use crate::error::ErrorResponse;
use crate::models::{
    AuthResponse, ForgotPasswordRequest, LoginRequest, LogoutRequest,
    MessageResponse, RegisterRequest, ResetPasswordRequest, SessionRequest, User
};
use tauri::command;
use log::{info, error};

#[command]
pub async fn register_command(request: RegisterRequest) -> Result<AuthResponse, ErrorResponse> {
    info!("Register request received for user: {}", request.username);
    
    match register_user(&request.username, &request.email, &request.password) {
        Ok(user) => {
            info!("User registered successfully: {}", user.username);
            Ok(AuthResponse {
                user,
                message: "注册成功".to_string(),
                token: None,
            })
        },
        Err(err) => {
            error!("Registration failed: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn login_command(request: LoginRequest) -> Result<AuthResponse, ErrorResponse> {
    info!("Login request received for: {}", request.username_or_email);
    
    match login_user(&request.username_or_email, &request.password) {
        Ok((user, token)) => {
            info!("User logged in successfully: {}", user.username);
            Ok(AuthResponse {
                user,
                message: "登录成功".to_string(),
                token: Some(token),
            })
        },
        Err(err) => {
            error!("Login failed: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn logout_command(request: LogoutRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Logout request received for user ID: {}", request.user_id);
    
    match logout_user(&request.user_id, &request.token) {
        Ok(_) => {
            info!("User logged out successfully: {}", request.user_id);
            Ok(MessageResponse {
                message: "退出登录成功".to_string(),
            })
        },
        Err(err) => {
            error!("Logout failed: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn forgot_password_command(request: ForgotPasswordRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Password reset request received for email: {}", request.email);
    
    let config = Config::get();
    
    match create_password_reset_token(&request.email) {
        Ok(token) => {
            info!("Password reset token created for email: {}", request.email);
            
            // 如果启用了邮箱验证，则发送邮件
            if config.auth.enable_email_verification {
                if let Err(e) = crate::utils::email::send_password_reset_email(&request.email, &token.token) {
                    error!("Failed to send password reset email: {}", e);
                    return Err(ErrorResponse {
                        error: "发送重置邮件失败，请稍后再试".to_string(),
                        status: 500,
                    });
                }
                
                Ok(MessageResponse {
                    message: "密码重置链接已发送到您的邮箱".to_string(),
                })
            } else {
                // 开发模式下直接返回令牌
                Ok(MessageResponse {
                    message: format!("密码重置令牌: {}", token.token),
                })
            }
        },
        Err(err) => {
            error!("Failed to create password reset token: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn reset_password_command(request: ResetPasswordRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Password reset attempt with token");
    
    match reset_password(&request.token, &request.new_password) {
        Ok(_) => {
            info!("Password reset successful");
            Ok(MessageResponse {
                message: "密码重置成功".to_string(),
            })
        },
        Err(err) => {
            error!("Password reset failed: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn verify_session_command(request: SessionRequest) -> Result<User, ErrorResponse> {
    match verify_session(&request.token) {
        Ok(user) => {
            info!("Session verified for user: {}", user.username);
            Ok(user)
        },
        Err(err) => {
            error!("Session verification failed: {}", err);
            Err(err.into())
        },
    }
}