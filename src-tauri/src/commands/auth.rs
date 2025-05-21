/**
 * 用户
 */
use crate::config::Config;
use crate::error::auth::ErrorResponse;
use crate::middleware::auth::{generate_jwt_token, revoke_jwt_token, store_jwt_token, AuthState};
use crate::models::{
    AuthResponse, ForgotPasswordRequest, LoginRequest, LogoutRequest, MessageResponse,
    RegisterRequest, ResetPasswordRequest, SendVerificationCodeRequest, SessionRequest, User,
};
use crate::services::auth::{
    login_user, logout_user, register_user, reset_password, reset_password_with_code,
    verify_session,
};
use crate::services::verification::generate_and_send_verification_code;
use log::{error, info};
use std::sync::Arc;
use tauri::{command, State};

#[tauri::command]
pub async fn auth_register_command(
    request: RegisterRequest,
) -> Result<AuthResponse, ErrorResponse> {
    //info! 宏会通过引用接收参数
    info!("Register request received for user: {}", request.username);

    match register_user(
        &request.username,
        &request.email,
        &request.password,
        &request.verification_code,
    ) {
        Ok(user) => {
            info!("User registered successfully: {}", user.username);
            Ok(AuthResponse {
                user,
                message: "注册成功".to_string(),
                token: None,
            })
        }
        Err(err) => {
            error!("Registration failed: {}", err);
            Err(err.into())
        }
    }
}

#[tauri::command]
pub async fn auth_login_command(
    request: LoginRequest,
    auth_state: State<'_, Arc<AuthState>>,
) -> Result<AuthResponse, ErrorResponse> {
    info!("Login request received for: {}", request.username_or_email);

    match login_user(&request.username_or_email, &request.password) {
        Ok((user, session_token)) => {
            info!("User logged in successfully: {}", user.username);

            // 生成 JWT 令牌
            let jwt_token = match generate_jwt_token(&user, &auth_state.jwt_secret) {
                Ok(token) => {
                    // 存储 JWT 令牌（可选，用于撤销）
                    if let Err(e) = store_jwt_token(user.id, &token) {
                        error!("Failed to store JWT token: {}", e);
                    }
                    token
                }
                Err(e) => {
                    error!("Failed to generate JWT token: {}", e);
                    session_token // 如果 JWT 生成失败，回退到会话令牌
                }
            };

            Ok(AuthResponse {
                user,
                message: "登录成功".to_string(),
                token: Some(jwt_token), // 返回 JWT 令牌
            })
        }
        Err(err) => {
            error!("Login failed: {}", err);
            Err(err.into())
        }
    }
}

#[tauri::command]
pub async fn auth_logout_command(request: LogoutRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Logout request received for user ID: {}", request.user_id);

    // 尝试撤销 JWT 令牌
    if let Err(e) = revoke_jwt_token(&request.token) {
        error!("Failed to revoke JWT token: {}", e);
    }

    // 调用原有的登出函数
    match logout_user(request.user_id, &request.token) {
        Ok(_) => {
            info!("User logged out successfully: {}", request.user_id);
            Ok(MessageResponse {
                message: "退出登录成功".to_string(),
            })
        }
        Err(err) => {
            error!("Logout failed: {}", err);
            Err(err.into())
        }
    }
}

#[command]
//// purpose:reset_password||register
pub async fn auth_send_verification_code_command(
    request: SendVerificationCodeRequest,
) -> Result<MessageResponse, ErrorResponse> {
    info!(
        "Verification code request received for email: {}, purpose: {}",
        request.email, request.purpose
    );

    match generate_and_send_verification_code(&request.email, &request.purpose) {
        Ok(_) => {
            info!("Verification code sent to: {}", request.email);
            Ok(MessageResponse {
                message: "验证码已发送到您的邮箱".to_string(),
            })
        }
        Err(err) => {
            error!("Failed to send verification code: {}", err);
            Err(err.into())
        }
    }
}

#[tauri::command]
pub async fn auth_forgot_password_command(
    request: ForgotPasswordRequest,
) -> Result<MessageResponse, ErrorResponse> {
    info!(
        "Password reset request received for email: {}",
        request.email
    );

    match reset_password_with_code(
        &request.email,
        &request.verification_code,
        &request.new_password,
    ) {
        Ok(_) => {
            info!("Password reset successful for email: {}", request.email);
            Ok(MessageResponse {
                message: "密码重置成功".to_string(),
            })
        }
        Err(err) => {
            error!("Password reset failed: {}", err);
            Err(err.into())
        }
    }
}

#[tauri::command]
pub async fn auth_reset_password_command(
    request: ResetPasswordRequest,
) -> Result<MessageResponse, ErrorResponse> {
    info!("Password reset attempt with token");

    match reset_password(&request.token, &request.new_password) {
        Ok(_) => {
            info!("Password reset successful");
            Ok(MessageResponse {
                message: "密码重置成功".to_string(),
            })
        }
        Err(err) => {
            error!("Password reset failed: {}", err);
            Err(err.into())
        }
    }
}

#[tauri::command]
pub async fn auth_verify_session_command(
    request: SessionRequest,
    auth_state: State<'_, Arc<AuthState>>,
) -> Result<User, ErrorResponse> {
    match verify_session(&request.token) {
        Ok(user) => {
            info!("Session verified for user: {}", user.username);
            Ok(user)
        }
        Err(err) => {
            error!("Session verification failed: {}", err);
            Err(err.into())
        }
    }
}
