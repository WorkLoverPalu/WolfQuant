use serde::{Deserialize, Serialize};

/// 用户组结构体，表示一个用户组的详细信息。
///
/// 字段说明：
/// - `id`: 用户组唯一标识
/// - `user_id`: 所属用户的唯一标识
/// - `name`: 用户组名称
/// - `asset_type_id`: 资产类型唯一标识
/// - `asset_type_name`: 资产类型名称
/// - `description`: 用户组描述（可选）
/// - `created_at`: 创建时间（时间戳）
/// - `updated_at`: 更新时间（时间戳）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserGroup {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub asset_type_id: i64,
    pub asset_type_name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 创建用户组的请求结构体。
///
/// 字段说明：
/// - `user_id`: 所属用户的唯一标识
/// - `name`: 用户组名称
/// - `asset_type_id`: 资产类型唯一标识
/// - `description`: 用户组描述（可选）
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub user_id: i64,
    pub name: String,
    pub asset_type_id: i64,
    pub description: Option<String>,
}
/// 更新用户组的请求结构体。
///
/// 字段说明：
/// - `id`: 用户组唯一标识
/// - `user_id`: 所属用户的唯一标识
/// - `name`: 用户组名称
/// - `description`: 用户组描述（可选）
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
}
/// 删除用户组的请求结构体。
///
/// 字段说明：
/// - `id`: 用户组唯一标识
/// - `user_id`: 所属用户的唯一标识
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteGroupRequest {
    pub id: i64,
    pub user_id: i64,
}
