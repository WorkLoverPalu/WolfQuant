use crate::config::Config;
use crate::error::auth::AuthError;
use log::error;

// 发送密码重置邮件
pub fn send_password_reset_email(email: &str, token: &str) -> Result<(), AuthError> {
    let config = Config::get();

    // 如果不是开发模式且未启用邮箱验证，则跳过发送
    if !config.dev_mode && !config.auth.enable_email_verification {
        return Ok(());
    }

    // 在开发模式下，只打印邮件内容
    if config.dev_mode {
        println!("发送密码重置邮件到 {}", email);
        println!("重置令牌: {}", token);
        println!(
            "重置链接: http://localhost:1420/reset-password?token={}",
            token
        );
        return Ok(());
    }

    // 实际应用中，这里应该使用邮件发送库
    // 例如 lettre 或其他 SMTP 客户端
    // 以下是一个示例实现，实际应用需要替换

    /*
    use lettre::{Message, SmtpTransport, Transport};
    use lettre::transport::smtp::authentication::Credentials;

    let email_body = format!(
        "您好，\n\n您收到此邮件是因为您请求重置密码。\n\n请点击以下链接重置密码：\n\nhttp://your-app-url/reset-password?token={}\n\n如果您没有请求重置密码，请忽略此邮件。\n\n谢谢！",
        token
    );

    let email = Message::builder()
        .from("noreply@your-app.com".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("密码重置请求")
        .body(email_body)
        .unwrap();

    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    let mailer = SmtpTransport::relay("smtp.your-provider.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to send email: {}", e);
            Err(AuthError::InternalError(format!("发送邮件失败: {}", e)))
        }
    }
    */

    // 临时实现，实际应用需要替换
    Ok(())
}

// 发送验证码邮件
pub fn send_verification_code_email(
    email: &str,
    code: &str,
    purpose: &str,
) -> Result<(), AuthError> {
    let config = Config::get();

    let purpose_text = match purpose {
        "register" => "注册账号",
        "reset_password" => "重置密码",
        _ => "验证邮箱",
    };

    // 在开发模式下，只打印邮件内容
    if config.dev_mode {
        println!("发送验证码邮件到 {}", email);
        println!("验证码: {}", code);
        println!("用途: {}", purpose_text);
        return Ok(());
    }

    // 如果未启用邮箱验证，则跳过发送
    if !config.auth.enable_email_verification {
        return Ok(());
    }

    // 实际应用中，这里应该使用邮件发送库
    // 例如 lettre 或其他 SMTP 客户端
    // 以下是一个示例实现，实际应用需要替换

    /*
    use lettre::{Message, SmtpTransport, Transport};
    use lettre::transport::smtp::authentication::Credentials;

    let email_body = format!(
        "您好，\n\n您的验证码是: {}\n\n此验证码用于{}，有效期为10分钟。\n\n如果您没有请求此验证码，请忽略此邮件。\n\n谢谢！",
        code,
        purpose_text
    );

    let email = Message::builder()
        .from("noreply@your-app.com".parse().unwrap())
        .to(email.parse().unwrap())
        .subject(format!("WolfQuant - {}验证码", purpose_text))
        .body(email_body)
        .unwrap();

    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    let mailer = SmtpTransport::relay("smtp.your-provider.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to send email: {}", e);
            Err(AuthError::InternalError(format!("发送邮件失败: {}", e)))
        }
    }
    */

    // 临时实现，实际应用需要替换
    Ok(())
}
