/// 资产类型结构体，表示一种资产的基本信息。
///
/// # 字段
/// - `id`: 资产类型的唯一标识符。
/// - `name`: 资产类型的名称。
/// - `description`: 资产类型的描述信息，可选字段。
/// 

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}
