use crate::error::auth::AuthError;
use log::{info, warn};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

// 导入各个模块的表结构
mod asset_tables;
mod backtester_tables;
mod candles_tables;
mod importer_tables;
mod transaction_tables;
mod user_tables;
mod strategy_tables; // 新增策略表结构

/// 加载所有模块的表结构
pub fn load_all_schemas() -> Result<HashMap<String, String>, AuthError> {
    let mut schemas = HashMap::new();

    // 从文件系统加载表结构
    let file_schemas = load_table_schemas_from_files()?;
    schemas.extend(file_schemas);

    // 从代码模块加载表结构
    schemas.extend(user_tables::get_schemas());
    schemas.extend(asset_tables::get_schemas());
    schemas.extend(transaction_tables::get_schemas());
    schemas.extend(backtester_tables::get_schemas());
    schemas.extend(candles_tables::get_schemas());
    schemas.extend(importer_tables::get_schemas());
    schemas.extend(strategy_tables::get_schemas()); // 新增策略表结构

    // 可以在这里添加更多模块的表结构

    if schemas.is_empty() {
        warn!("没有找到任何表结构定义");
    }

    Ok(schemas)
}

/// 从外部文件加载表结构定义
fn load_table_schemas_from_files() -> Result<HashMap<String, String>, AuthError> {
    let mut schemas = HashMap::new();

    // 获取schema目录路径
    let config = crate::config::Config::get();
    let schema_dir = if config.database.schema_dir.is_empty() {
        Path::new("data/schemas")
    } else {
        Path::new(&config.database.schema_dir)
    };

    // 如果目录不存在，直接返回空结果
    if !schema_dir.exists() {
        return Ok(schemas);
    }

    // 读取所有.sql文件
    for entry in fs::read_dir(schema_dir)
        .map_err(|e| AuthError::DatabaseError(format!("无法读取schema目录: {}", e)))?
    {
        let entry =
            entry.map_err(|e| AuthError::DatabaseError(format!("无法读取目录项: {}", e)))?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "sql") {
            // 从文件名获取表名
            let table_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| AuthError::DatabaseError("无效的schema文件名".to_string()))?
                .to_string();

            // 读取文件内容
            let mut file = fs::File::open(&path)
                .map_err(|e| AuthError::DatabaseError(format!("无法打开schema文件: {}", e)))?;

            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| AuthError::DatabaseError(format!("无法读取schema文件: {}", e)))?;

            // 添加到schemas
            info!("从文件加载表结构: {}", table_name);
            schemas.insert(table_name, content);
        }
    }

    Ok(schemas)
}

/// 生成默认schema文件
pub fn generate_default_schemas(schema_dir: &Path) -> Result<(), AuthError> {
    // 确保目录存在
    if !schema_dir.exists() {
        fs::create_dir_all(schema_dir)
            .map_err(|e| AuthError::DatabaseError(format!("无法创建schema目录: {}", e)))?;
    }

    // 获取所有表结构
    let schemas = load_all_schemas()?;

    // 写入文件
    for (table_name, schema) in schemas {
        let file_path = schema_dir.join(format!("{}.sql", table_name));
        fs::write(&file_path, schema)
            .map_err(|e| AuthError::DatabaseError(format!("无法写入schema文件: {}", e)))?;

        info!("生成schema文件: {:?}", file_path);
    }

    Ok(())
}
