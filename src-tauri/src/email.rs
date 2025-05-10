use crate::config;
use crate::error::AuthError;
use crate::models::{EmailVerificationCode, PasswordResetToken};
use log::{error, info};

// 发送密码重置邮件
pub fn send_password_reset_email(email: &str, token: &PasswordResetToken) -> Result<(), AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    if !config.email.enable {
        info!("Email service disabled. Would send password reset email to {}", email);
        return Ok(());
    }
    
    // 在实际应用中，这里会使用SMTP客户端发送邮件
    // 这里我们只是记录日志
    info!(
        "Sending password reset email to {} with token: {}",
        email, token.token
    );
    
    // 模拟邮件发送
    if config.app.dev_mode {
        info!("DEV MODE: Password reset link: http://localhost:1420/reset-password?token={}", token.token);
    }
    
    Ok(())
}

// 发送邮箱验证码
pub fn send_verification_code(email: &str, code: &EmailVerificationCode) -> Result<(), AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    if !config.email.enable {
        info!("Email service disabled. Would send verification code to {}", email);
        return Ok(());
    }
    
    // 在实际应用中，这里会使用SMTP客户端发送邮件
    // 这里我们只是记录日志
    info!(
        "Sending verification code to {}: {}",
        email, code.code
    );
    
    // 模拟邮件发送
    if config.app.dev_mode {
        info!("DEV MODE: Verification code for {}: {}", email, code.code);
    }
    
    Ok(())
}

// 发送欢迎邮件
pub fn send_welcome_email(email: &str, username: &str) -> Result<(), AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    if !config.email.enable {
        info!("Email service disabled. Would send welcome email to {}", email);
        return Ok(());
    }
    
    // 在实际应用中，这里会使用SMTP客户端发送邮件
    // 这里我们只是记录日志
    info!(
        "Sending welcome email to {} ({})",
        email, username
    );
    
    Ok(())
}