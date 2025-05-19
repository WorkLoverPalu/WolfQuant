use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}
