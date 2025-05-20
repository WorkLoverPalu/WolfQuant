/// 用户结构体，表示系统中的用户信息。
///
/// 字段说明：
/// - `id`: 用户唯一标识符。
/// - `username`: 用户名。
/// - `email`: 用户邮箱地址。
/// - `email_verified`: 邮箱是否已验证。
/// - `created_at`: 用户创建时间（时间戳，单位为秒）。
/// - `updated_at`: 用户信息最后更新时间（时间戳，单位为秒）。
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
