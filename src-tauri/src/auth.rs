use crate::config;
use crate::database::DB;
use crate::email;
use crate::error::AuthError;
use crate::models::{
    AuthResponse, EmailVerificationCode, MessageResponse, PasswordResetToken, Session, User,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use log::{error, info, warn};
use rusqlite::{params, Result as SqliteResult};

// 注册新用户
pub fn register_user(username: &str, email: &str, password: &str) -> Result<User, AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    // 验证密码强度
    validate_password(password)?;
    
    // 检查用户名是否已存在
    if user_exists_by_username(username)? {
        return Err(AuthError::UsernameTaken);
    }

    // 检查邮箱是否已注册
    if user_exists_by_email(email)? {
        return Err(AuthError::EmailTaken);
    }

    // 哈希密码
    let password_hash = hash_password(password)?;

    // 创建新用户
    let user = User::new(username.to_string(), email.to_string(), password_hash);

    // 保存用户到数据库
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    conn.execute(
        "INSERT INTO users (id, username, email, password_hash, email_verified, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.email_verified,
            user.created_at.to_rfc3339(),
            user.updated_at.to_rfc3339()
        ],
    )
    .map_err(AuthError::DatabaseError)?;

    // 如果启用了邮箱验证，发送验证码
    if config.auth.enable_email_verification {
        let verification_code = create_email_verification_code(&user.id, email)?;
        email::send_verification_code(email, &verification_code)?;
    } else {
        // 如果没有启用邮箱验证，直接标记为已验证
        let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
        conn.execute(
            "UPDATE users SET email_verified = 1 WHERE id = ?1",
            params![user.id],
        )
        .map_err(AuthError::DatabaseError)?;
        
        // 发送欢迎邮件
        let _ = email::send_welcome_email(email, username);
    }

    info!("User registered successfully: {}", username);
    Ok(user)
}

// 用户登录
pub fn login_user(username_or_email: &str, password: &str, ip_address: Option<String>, user_agent: Option<String>) -> Result<(User, String), AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    // 查找用户（通过用户名或邮箱）
    let mut stmt = conn
        .prepare("SELECT id, username, email, password_hash, email_verified, created_at, updated_at, last_login, login_attempts, locked_until FROM users WHERE username = ?1 OR email = ?1")
        .map_err(AuthError::DatabaseError)?;
    
    let user_result = stmt.query_row(params![username_or_email], |row| {
        let locked_until: Option<String> = row.get(9)?;
        let locked_until = locked_until.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|_| rusqlite::Error::InvalidColumnType(9, "DateTime".to_string()))
                .map(|dt| dt.with_timezone(&Utc))
        }).transpose()?;
        
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            email_verified: row.get(4)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(5, "DateTime".to_string()))?
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(6, "DateTime".to_string()))?
                .with_timezone(&Utc),
            last_login: row.get::<_, Option<String>>(7)?.map(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(7, "DateTime".to_string()))
                    .map(|dt| dt.with_timezone(&Utc))
            }).transpose()?,
            login_attempts: row.get(8)?,
            locked_until,
        })
    });
    
    match user_result {
        Ok(mut user) => {
            // 检查账户是否被锁定
            if user.is_locked() {
                return Err(AuthError::AccountLocked);
            }
            
            // 验证密码
            if verify_password(password, &user.password_hash)? {
                // 重置登录尝试次数
                conn.execute(
                    "UPDATE users SET login_attempts = 0, locked_until = NULL, last_login = ?1, updated_at = ?1 WHERE id = ?2",
                    params![
                        Utc::now().to_rfc3339(),
                        user.id
                    ],
                )
                .map_err(AuthError::DatabaseError)?;
                
                // 如果需要邮箱验证但邮箱未验证
                if config.auth.enable_email_verification && !user.email_verified {
                    return Err(AuthError::EmailVerificationRequired);
                }
                
                // 创建会话
                let session = create_session(&user.id, ip_address, user_agent)?;
                
                // 更新用户对象
                user.login_attempts = 0;
                user.locked_until = None;
                user.last_login = Some(Utc::now());
                
                info!("User logged in successfully: {}", user.username);
                Ok((user, session.token))
            } else {
                // 增加登录尝试次数
                let new_attempts = user.login_attempts + 1;
                
                // 如果超过最大尝试次数，锁定账户
                let locked_until = if new_attempts >= config.auth.max_login_attempts {
                    let locked_until = Utc::now() + chrono::Duration::minutes(config.auth.login_lockout_minutes as i64);
                    Some(locked_until.to_rfc3339())
                } else {
                    None
                };
                
                conn.execute(
                    "UPDATE users SET login_attempts = ?1, locked_until = ?2, updated_at = ?3 WHERE id = ?4",
                    params![
                        new_attempts,
                        locked_until,
                        Utc::now().to_rfc3339(),
                        user.id
                    ],
                )
                .map_err(AuthError::DatabaseError)?;
                
                if new_attempts >= config.auth.max_login_attempts {
                    warn!("Account locked due to too many failed login attempts: {}", user.username);
                    Err(AuthError::AccountLocked)
                } else {
                    warn!("Failed login attempt for user: {}", user.username);
                    Err(AuthError::InvalidCredentials)
                }
            }
        }
        Err(_) => {
            warn!("Login attempt for non-existent user: {}", username_or_email);
            Err(AuthError::InvalidCredentials)
        }
    }
}

// 创建密码重置令牌
pub fn create_password_reset_token(email: &str) -> Result<PasswordResetToken, AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    // 查找用户
    let user = get_user_by_email(email)?;
    
    // 创建新令牌
    let token = PasswordResetToken::new(
        user.id.clone(),
        config.auth.password_reset_token_expiry_hours
    );
    
    // 保存令牌到数据库
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    // 删除该用户之前的所有令牌
    conn.execute(
        "DELETE FROM password_reset_tokens WHERE user_id = ?1",
        params![user.id],
    )
    .map_err(AuthError::DatabaseError)?;
    
    // 插入新令牌
    conn.execute(
        "INSERT INTO password_reset_tokens (token, user_id, expires_at) VALUES (?1, ?2, ?3)",
        params![
            token.token,
            token.user_id,
            token.expires_at.to_rfc3339()
        ],
    )
    .map_err(AuthError::DatabaseError)?;
    
    // 发送密码重置邮件
    email::send_password_reset_email(&user.email, &token)?;
    
    info!("Password reset token created for user: {}", user.username);
    Ok(token)
}

// 重置密码
pub fn reset_password(token: &str, new_password: &str) -> Result<(), AuthError> {
    // 验证密码强度
    validate_password(new_password)?;
    
    // 验证令牌
    let reset_token = get_reset_token(token)?;
    
    // 检查令牌是否过期
    if reset_token.is_expired() {
        return Err(AuthError::InvalidToken);
    }
    
    // 哈希新密码
    let password_hash = hash_password(new_password)?;
    
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    // 更新用户密码
    conn.execute(
        "UPDATE users SET password_hash = ?1, updated_at = ?2, login_attempts = 0, locked_until = NULL WHERE id = ?3",
        params![
            password_hash,
            Utc::now().to_rfc3339(),
            reset_token.user_id
        ],
    )
    .map_err(AuthError::DatabaseError)?;
    
    // 删除使用过的令牌
    conn.execute(
        "DELETE FROM password_reset_tokens WHERE token = ?1",
        params![token],
    )
    .map_err(AuthError::DatabaseError)?;
    
    info!("Password reset successfully for user ID: {}", reset_token.user_id);
    Ok(())
}

// 创建邮箱验证码
pub fn create_email_verification_code(user_id: &str, email: &str) -> Result<EmailVerificationCode, AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    // 创建新验证码
    let verification_code = EmailVerificationCode::new(
        user_id.to_string(),
        email.to_string(),
        config.auth.verification_code_expiry_minutes
    );
    
    // 保存验证码到数据库
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    // 删除该用户之前的所有验证码
    conn.execute(
        "DELETE FROM email_verification_codes WHERE user_id = ?1",
        params![user_id],
    )
    .map_err(AuthError::DatabaseError)?;
    
    // 插入新验证码
    conn.execute(
        "INSERT INTO email_verification_codes (code, user_id, email, expires_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            verification_code.code,
            verification_code.user_id,
            verification_code.email,
            verification_code.expires_at.to_rfc3339()
        ],
    )
    .map_err(AuthError::DatabaseError)?;
    
    info!("Email verification code created for user ID: {}", user_id);
    Ok(verification_code)
}

// 验证邮箱
pub fn verify_email(code: &str, email: &str) -> Result<(), AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    // 查找验证码
    let mut stmt = conn
        .prepare("SELECT code, user_id, email, expires_at FROM email_verification_codes WHERE code = ?1 AND email = ?2")
        .map_err(AuthError::DatabaseError)?;
    
    let verification_code_result = stmt.query_row(params![code, email], |row| {
        Ok(EmailVerificationCode {
            code: row.get(0)?,
            user_id: row.get(1)?,
            email: row.get(2)?,
            expires_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(3, "DateTime".to_string()))?
                .with_timezone(&Utc),
        })
    });
    
    match verification_code_result {
        Ok(verification_code) => {
            // 检查验证码是否过期
            if verification_code.is_expired() {
                return Err(AuthError::InvalidVerificationCode);
            }
            
            // 更新用户邮箱验证状态
            conn.execute(
                "UPDATE users SET email_verified = 1, updated_at = ?1 WHERE id = ?2",
                params![
                    Utc::now().to_rfc3339(),
                    verification_code.user_id
                ],
            )
            .map_err(AuthError::DatabaseError)?;
            
            // 删除使用过的验证码
            conn.execute(
                "DELETE FROM email_verification_codes WHERE code = ?1",
                params![code],
            )
            .map_err(AuthError::DatabaseError)?;
            
            // 获取用户信息
            let user = get_user_by_id(&verification_code.user_id)?;
            
            // 发送欢迎邮件
            let _ = email::send_welcome_email(&user.email, &user.username);
            
            info!("Email verified successfully for user: {}", user.username);
            Ok(())
        }
        Err(_) => {
            warn!("Invalid verification code attempt: {}", code);
            Err(AuthError::InvalidVerificationCode)
        }
    }
}

// 创建会话
pub fn create_session(user_id: &str, ip_address: Option<String>, user_agent: Option<String>) -> Result<Session, AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    // 创建新会话
    let session = Session::new(
        user_id.to_string(),
        config.auth.jwt_expiry_hours,
        ip_address,
        user_agent
    );
    
    // 保存会话到数据库
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    conn.execute(
        "INSERT INTO sessions (token, user_id, created_at, expires_at, ip_address, user_agent) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            session.token,
            session.user_id,
            session.created_at.to_rfc3339(),
            session.expires_at.to_rfc3339(),
            session.ip_address,
            session.user_agent
        ],
    )
    .map_err(AuthError::DatabaseError)?;
    
    info!("Session created for user ID: {}", user_id);
    Ok(session)
}

// 验证会话
pub fn validate_session(token: &str) -> Result<User, AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    // 查找会话
    let mut stmt = conn
        .prepare("SELECT token, user_id, created_at, expires_at, ip_address, user_agent FROM sessions WHERE token = ?1")
        .map_err(AuthError::DatabaseError)?;
    
    let session_result = stmt.query_row(params![token], |row| {
        Ok(Session {
            token: row.get(0)?,
            user_id: row.get(1)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(2, "DateTime".to_string()))?
                .with_timezone(&Utc),
            expires_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(3, "DateTime".to_string()))?
                .with_timezone(&Utc),
            ip_address: row.get(4)?,
            user_agent: row.get(5)?,
        })
    });
    
    match session_result {
        Ok(session) => {
            // 检查会话是否过期
            if session.is_expired() {
                // 删除过期会话
                conn.execute(
                    "DELETE FROM sessions WHERE token = ?1",
                    params![token],
                )
                .map_err(AuthError::DatabaseError)?;
                
                return Err(AuthError::InvalidToken);
            }
            
            // 获取用户信息
            get_user_by_id(&session.user_id)
        }
        Err(_) => Err(AuthError::InvalidToken),
    }
}

// 注销会话
pub fn logout_session(token: &str) -> Result<(), AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    
    // 删除会话
    conn.execute(
        "DELETE FROM sessions WHERE token = ?1",
        params![token],
    )
    .map_err(AuthError::DatabaseError)?;
    
    info!("Session logged out: {}", token);
    Ok(())
}

// 辅助函数

// 哈希密码
fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AuthError::PasswordHashError(e.to_string()))
}

// 验证密码
fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AuthError::PasswordHashError(e.to_string()))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// 验证密码强度
fn validate_password(password: &str) -> Result<(), AuthError> {
    let config = config::get_config().map_err(|e| AuthError::ConfigError(e))?;
    
    // 检查密码长度
    if password.len() < config.auth.min_password_length {
        return Err(AuthError::WeakPassword(format!(
            "Password must be at least {} characters long",
            config.auth.min_password_length
        )));
    }
    
    // 检查是否需要特殊字符
    if config.auth.require_special_chars && !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(AuthError::WeakPassword(
            "Password must contain at least one special character".to_string()
        ));
    }
    
    // 检查是否需要数字
    if config.auth.require_numbers && !password.chars().any(|c| c.is_numeric()) {
        return Err(AuthError::WeakPassword(
            "Password must contain at least one number".to_string()
        ));
    }
    
    // 检查是否需要大写字母
    if config.auth.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
        return Err(AuthError::WeakPassword(
            "Password must contain at least one uppercase letter".to_string()
        ));
    }
    
    Ok(())
}

// 检查用户名是否存在
fn user_exists_by_username(username: &str) -> Result<bool, AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    let mut stmt = conn
        .prepare("SELECT 1 FROM users WHERE username = ?1 LIMIT 1")
        .map_err(AuthError::DatabaseError)?;
    
    let exists = stmt
        .exists(params![username])
        .map_err(AuthError::DatabaseError)?;
    
    Ok(exists)
}

// 检查邮箱是否存在
fn user_exists_by_email(email: &str) -> Result<bool, AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    let mut stmt = conn
        .prepare("SELECT 1 FROM users WHERE email = ?1 LIMIT 1")
        .map_err(AuthError::DatabaseError)?;
    
    let exists = stmt
        .exists(params![email])
        .map_err(AuthError::DatabaseError)?;
    
    Ok(exists)
}

// 通过ID获取用户
fn get_user_by_id(id: &str) -> Result<User, AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    let mut stmt = conn
        .prepare("SELECT id, username, email, password_hash, email_verified, created_at, updated_at, last_login, login_attempts, locked_until FROM users WHERE id = ?1")
        .map_err(AuthError::DatabaseError)?;
    
    let user_result = stmt.query_row(params![id], |row| {
        let locked_until: Option<String> = row.get(9)?;
        let locked_until = locked_until.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|_| rusqlite::Error::InvalidColumnType(9, "DateTime".to_string()))
                .map(|dt| dt.with_timezone(&Utc))
        }).transpose()?;
        
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            email_verified: row.get(4)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(5, "DateTime".to_string()))?
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(6, "DateTime".to_string()))?
                .with_timezone(&Utc),
            last_login: row.get::<_, Option<String>>(7)?.map(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(7, "DateTime".to_string()))
                    .map(|dt| dt.with_timezone(&Utc))
            }).transpose()?,
            login_attempts: row.get(8)?,
            locked_until,
        })
    });
    
    match user_result {
        Ok(user) => Ok(user),
        Err(_) => Err(AuthError::UserNotFound),
    }
}

// 通过邮箱获取用户
fn get_user_by_email(email: &str) -> Result<User, AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    let mut stmt = conn
        .prepare("SELECT id, username, email, password_hash, email_verified, created_at, updated_at, last_login, login_attempts, locked_until FROM users WHERE email = ?1")
        .map_err(AuthError::DatabaseError)?;
    
    let user_result = stmt.query_row(params![email], |row| {
        let locked_until: Option<String> = row.get(9)?;
        let locked_until = locked_until.map(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map_err(|_| rusqlite::Error::InvalidColumnType(9, "DateTime".to_string()))
                .map(|dt| dt.with_timezone(&Utc))
        }).transpose()?;
        
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            email_verified: row.get(4)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(5, "DateTime".to_string()))?
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(6, "DateTime".to_string()))?
                .with_timezone(&Utc),
            last_login: row.get::<_, Option<String>>(7)?.map(|s| {
                chrono::DateTime::parse_from_rfc3339(&s)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(7, "DateTime".to_string()))
                    .map(|dt| dt.with_timezone(&Utc))
            }).transpose()?,
            login_attempts: row.get(8)?,
            locked_until,
        })
    });
    
    match user_result {
        Ok(user) => Ok(user),
        Err(_) => Err(AuthError::UserNotFound),
    }
}

// 获取密码重置令牌
fn get_reset_token(token: &str) -> Result<PasswordResetToken, AuthError> {
    let conn = DB.lock().map_err(|e| AuthError::InternalError(e.to_string()))?;
    let mut stmt = conn
        .prepare("SELECT token, user_id, expires_at FROM password_reset_tokens WHERE token = ?1")
        .map_err(AuthError::DatabaseError)?;
    
    let token_result = stmt.query_row(params![token], |row| {
        Ok(PasswordResetToken {
            token: row.get(0)?,
            user_id: row.get(1)?,
            expires_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                .map_err(|_| rusqlite::Error::InvalidColumnType(2, "DateTime".to_string()))?
                .with_timezone(&Utc),
        })
    });
    
    match token_result {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthError::InvalidToken),
    }
}