use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub user_id: i64,
    pub name: String,
    pub asset_type_id: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteGroupRequest {
    pub id: i64,
    pub user_id: i64,
}
