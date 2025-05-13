use crate::config::Config;
use crate::database::get_db_connection;
use crate::error::AuthError;
use crate::models::EmailVerificationCode;
use crate::utils::crypto::generate_verification_code;
use crate::utils::email::send_verification_code_email;
use chrono::{Duration, Utc};
use log::{error, info};
use rusqlite::params;

// 生成并发送验证码
pub fn generate_and_send_verification_code(email: &str, purpose: &str) -> Result<(), AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    let config = Config::get();
    // 验证用途
    if purpose != "register" && purpose != "reset_password" {
        return Err(AuthError::InvalidCredentials("无效的验证码用途".to_string()));
    }
    
    // 生成6位数字验证码
    let code = generate_verification_code();
    
    // 设置过期时间（10分钟）
    let expires_at = (Utc::now() + Duration::minutes(config.auth.emial_code_valid_duration)).timestamp();
    
    // 检查是否已存在该邮箱和用途的验证码
    let code_exists: bool = conn.query_row(
        "SELECT 1 FROM email_verification_codes WHERE email = ?1 AND purpose = ?2",
        params![email, purpose],
        |_| Ok(true),
    ).unwrap_or(false);
    
    if code_exists {
        // 更新现有验证码
        conn.execute(
            "UPDATE email_verification_codes 
             SET code = ?1, expires_at = ?2, created_at = ?3 
             WHERE email = ?4 AND purpose = ?5",
            params![code, expires_at, now, email, purpose],
        )?;
    } else {
        // 创建新验证码
        conn.execute(
            "INSERT INTO email_verification_codes (email, code, purpose, expires_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![email, code, purpose, expires_at, now],
        )?;
    }
    
    // 发送验证码邮件
    send_verification_code_email(email, &code, purpose)?;
    
    info!("Verification code sent to: {} for purpose: {}", email, purpose);
    Ok(())
}

// 验证验证码
pub fn verify_code(email: &str, code: &str, purpose: &str) -> Result<bool, AuthError> {
    let conn = get_db_connection()?;
    let now = Utc::now().timestamp();
    
    // 查找验证码
    let result = conn.query_row(
        "SELECT code, expires_at FROM email_verification_codes 
         WHERE email = ?1 AND purpose = ?2",
        params![email, purpose],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
            ))
        },
    );
    
    match result {
        Ok((stored_code, expires_at)) => {
            // 检查是否过期
            if expires_at < now {
                return Err(AuthError::InvalidToken("验证码已过期".to_string()));
            }
            
            // 检查验证码是否匹配
            if stored_code != code {
                return Err(AuthError::InvalidCredentials("验证码不正确".to_string()));
            }
            
            // 验证成功后删除验证码
            conn.execute(
                "DELETE FROM email_verification_codes WHERE email = ?1 AND purpose = ?2",
                params![email, purpose],
            )?;
            
            info!("Verification code verified for: {} purpose: {}", email, purpose);
            Ok(true)
        },
        Err(_) => Err(AuthError::InvalidToken("验证码不存在".to_string())),
    }
}