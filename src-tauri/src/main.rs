#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod auth;
mod config;
mod database;
mod email;
mod error;
mod logger;
mod models;

use crate::auth::{
    create_password_reset_token, login_user, logout_session, register_user, reset_password,
    validate_session, verify_email,
};
use crate::error::{AuthError, ErrorResponse};
use crate::models::{
    AuthResponse, ForgotPasswordRequest, LoginRequest, MessageResponse, RegisterRequest,
    ResetPasswordRequest, UserResponse, VerifyEmailRequest,
};
use log::{error, info};
use tauri::{command, State, Window};

// 初始化应用程序
fn main() {
    // 初始化配置
    if let Err(e) = config::load_config() {
        eprintln!("Failed to load configuration: {}", e);
        return;
    }
    
    // 初始化日志
    if let Err(e) = logger::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        return;
    }
    
    // 初始化数据库
    if let Err(e) = database::init_database() {
        error!("Failed to initialize database: {}", e);
        return;
    }
    
    info!("Starting WolfQuant application");
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            logout,
            forgot_password,
            reset_password_command,
            verify_email_command,
            get_current_user
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}

// 注册命令
#[command]
async fn register(request: RegisterRequest) -> Result<MessageResponse, ErrorResponse> {
    match register_user(&request.username, &request.email, &request.password) {
        Ok(user) => {
            let config = config::get_config().map_err(|e| e.to_string())?;
            
            let message = if config.auth.enable_email_verification {
                format!("Registration successful. Please check your email to verify your account.")
            } else {
                format!("Registration successful. You can now log in.")
            };
            
            Ok(MessageResponse {
                message,
                success: true,
            })
        }
        Err(err) => Err(err.into()),
    }
}

// 登录命令
#[command]
async fn login(window: Window, request: LoginRequest) -> Result<AuthResponse, ErrorResponse> {
    // 获取IP地址和用户代理（在实际应用中，你可能需要从前端传递这些信息）
    let ip_address = Some("127.0.0.1".to_string());
    let user_agent = window.user_agent().ok();
    
    match login_user(&request.username, &request.password, ip_address, user_agent) {
        Ok((user, token)) => Ok(AuthResponse {
            user,
            token,
            message: "Login successful".to_string(),
        }),
        Err(err) => Err(err.into()),
    }
}

// 注销命令
#[command]
async fn logout(token: String) -> Result<MessageResponse, ErrorResponse> {
    match logout_session(&token) {
        Ok(_) => Ok(MessageResponse {
            message: "Logout successful".to_string(),
            success: true,
        }),
        Err(err) => Err(err.into()),
    }
}

// 忘记密码命令
#[command]
async fn forgot_password(request: ForgotPasswordRequest) -> Result<MessageResponse, ErrorResponse> {
    match create_password_reset_token(&request.email) {
        Ok(_) => {
            let config = config::get_config().map_err(|e| e.to_string())?;
            
            // 在开发模式下，返回更详细的信息
            let message = if config.app.dev_mode {
                "Password reset link has been sent to your email. Check the application logs for the token.".to_string()
            } else {
                "Password reset link has been sent to your email.".to_string()
            };
            
            Ok(MessageResponse {
                message,
                success: true,
            })
        }
        Err(err) => Err(err.into()),
    }
}

// 重置密码命令
#[command]
async fn reset_password_command(request: ResetPasswordRequest) -> Result<MessageResponse, ErrorResponse> {
    match reset_password(&request.token, &request.new_password) {
        Ok(_) => Ok(MessageResponse {
            message: "Password reset successful".to_string(),
            success: true,
        }),
        Err(err) => Err(err.into()),
    }
}

// 验证邮箱命令
#[command]
async fn verify_email_command(request: VerifyEmailRequest) -> Result<MessageResponse, ErrorResponse> {
    match verify_email(&request.code, &request.email) {
        Ok(_) => Ok(MessageResponse {
            message: "Email verification successful".to_string(),
            success: true,
        }),
        Err(err) => Err(err.into()),
    }
}

// 获取当前用户命令
#[command]
async fn get_current_user(token: String) -> Result<UserResponse, ErrorResponse> {
    match validate_session(&token) {
        Ok(user) => Ok(UserResponse { user }),
        Err(err) => Err(err.into()),
    }
}