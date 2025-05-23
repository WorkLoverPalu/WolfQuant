use crate::error::auth::AuthError;
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

// 导入各个模块的表结构
mod asset_tables;
mod backtester_tables;
mod candles_tables;
mod importer_tables;
mod strategy_tables;
mod transaction_tables;
mod user_tables;

/// 表结构定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub name: String,
    pub sql: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub checksum: String,
}

impl TableSchema {
    /// 创建新的表结构定义
    pub fn new(name: &str, sql: &str, version: &str) -> Self {
        let now = Utc::now();
        let checksum = Self::calculate_checksum(sql);
        
        Self {
            name: name.to_string(),
            sql: sql.to_string(),
            version: version.to_string(),
            description: None,
            dependencies: Vec::new(),
            created_at: now,
            updated_at: now,
            checksum,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// 设置依赖
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    /// 计算SQL的校验和
    fn calculate_checksum(sql: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        sql.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// 验证SQL语法
    pub fn validate_sql(&self) -> Result<(), AuthError> {
        // 基本的SQL语法检查
        let sql_lower = self.sql.to_lowercase();
        
        if sql_lower.trim().is_empty() {
            return Err(AuthError::DatabaseError("SQL不能为空".to_string()));
        }

        // 检查是否包含危险操作
        let dangerous_keywords = ["drop database", "truncate", "delete from", "update"];
        for keyword in dangerous_keywords {
            if sql_lower.contains(keyword) {
                warn!("表结构 {} 包含潜在危险操作: {}", self.name, keyword);
            }
        }

        // 检查是否是CREATE TABLE语句
        if !sql_lower.trim_start().starts_with("create table") {
            return Err(AuthError::DatabaseError(format!(
                "表结构 {} 必须以 CREATE TABLE 开始", self.name
            )));
        }

        Ok(())
    }
}

/// 模式加载器配置
#[derive(Debug, Clone)]
pub struct SchemaLoaderConfig {
    pub schema_dirs: Vec<PathBuf>,
    pub supported_extensions: Vec<String>,
    pub enable_caching: bool,
    pub cache_ttl_seconds: u64,
    pub validate_schemas: bool,
    pub parallel_loading: bool,
}

impl Default for SchemaLoaderConfig {
    fn default() -> Self {
        Self {
            schema_dirs: vec![PathBuf::from("data/schemas")],
            supported_extensions: vec!["sql".to_string(), "ddl".to_string()],
            enable_caching: true,
            cache_ttl_seconds: 3600, // 1小时
            validate_schemas: true,
            parallel_loading: false,
        }
    }
}

/// 缓存条目
#[derive(Debug, Clone)]
struct CacheEntry {
    schemas: HashMap<String, TableSchema>,
    loaded_at: SystemTime,
    file_checksums: HashMap<PathBuf, String>,
}

impl CacheEntry {
    fn is_expired(&self, ttl_seconds: u64) -> bool {
        self.loaded_at
            .elapsed()
            .map(|duration| duration.as_secs() > ttl_seconds)
            .unwrap_or(true)
    }
}

/// 模式加载器
pub struct SchemaLoader {
    config: SchemaLoaderConfig,
    cache: Arc<RwLock<Option<CacheEntry>>>,
    module_loaders: HashMap<String, Box<dyn Fn() -> HashMap<String, TableSchema> + Send + Sync>>,
}

impl SchemaLoader {
    /// 创建新的模式加载器
    pub fn new(config: SchemaLoaderConfig) -> Self {
        let mut loader = Self {
            config,
            cache: Arc::new(RwLock::new(None)),
            module_loaders: HashMap::new(),
        };

        // 注册内置模块加载器
        loader.register_module_loaders();
        loader
    }

    /// 注册模块加载器
    fn register_module_loaders(&mut self) {
        self.module_loaders.insert(
            "users".to_string(),
            Box::new(|| {
                user_tables::get_schemas()
                    .into_iter()
                    .map(|(k, v)| {
                        let name = k.clone();
                        (name, TableSchema::new(&name, &v, "1.0.0"))
                    })
                    .collect()
            }),
        );

        self.module_loaders.insert(
            "assets".to_string(),
            Box::new(|| {
                asset_tables::get_schemas()
                    .into_iter()
                    .map(|(k, v)| (k, TableSchema::new(&k, &v, "1.0.0")))
                    .collect()
            }),
        );

        self.module_loaders.insert(
            "transactions".to_string(),
            Box::new(|| {
                transaction_tables::get_schemas()
                    .into_iter()
                    .map(|(k, v)| (k, TableSchema::new(&k, &v, "1.0.0")))
                    .collect()
            }),
        );

        self.module_loaders.insert(
            "backtester".to_string(),
            Box::new(|| {
                backtester_tables::get_schemas()
                    .into_iter()
                    .map(|(k, v)| (k, TableSchema::new(&k, &v, "1.0.0")))
                    .collect()
            }),
        );

        self.module_loaders.insert(
            "candles".to_string(),
            Box::new(|| {
                candles_tables::get_schemas()
                    .into_iter()
                    .map(|(k, v)| (k, TableSchema::new(&k, &v, "1.0.0")))
                    .collect()
            }),
        );

        self.module_loaders.insert(
            "importer".to_string(),
            Box::new(|| {
                importer_tables::get_schemas()
                    .into_iter()
                    .map(|(k, v)| (k, TableSchema::new(&k, &v, "1.0.0")))
                    .collect()
            }),
        );

        self.module_loaders.insert(
            "strategies".to_string(),
            Box::new(|| {
                strategy_tables::get_schemas()
                    .into_iter()
                    .map(|(k, v)| (k, TableSchema::new(&k, &v, "1.0.0")))
                    .collect()
            }),
        );
    }

    /// 加载所有模式
    pub fn load_all_schemas(&self) -> Result<HashMap<String, TableSchema>, AuthError> {
        // 检查缓存
        if self.config.enable_caching {
            if let Some(cached) = self.get_cached_schemas()? {
                debug!("使用缓存的模式定义");
                return Ok(cached);
            }
        }

        info!("开始加载所有表结构定义");
        let start_time = std::time::Instant::now();

        let mut all_schemas = HashMap::new();

        // 从文件加载
        let file_schemas = self.load_schemas_from_files()?;
        let file_count = file_schemas.len();
        all_schemas.extend(file_schemas);

        // 从模块加载
        let module_schemas = self.load_schemas_from_modules()?;
        let module_count = module_schemas.len();
        all_schemas.extend(module_schemas);

        // 验证模式
        if self.config.validate_schemas {
            self.validate_all_schemas(&all_schemas)?;
        }

        // 解析依赖关系
        let ordered_schemas = self.resolve_dependencies(&all_schemas)?;

        // 更新缓存
        if self.config.enable_caching {
            self.update_cache(ordered_schemas.clone())?;
        }

        let elapsed = start_time.elapsed();
        info!(
            "加载完成: {} 个表结构 ({} 个来自文件, {} 个来自模块), 耗时: {:?}",
            ordered_schemas.len(),
            file_count,
            module_count,
            elapsed
        );

        if ordered_schemas.is_empty() {
            warn!("没有找到任何表结构定义");
        }

        Ok(ordered_schemas)
    }

    /// 从缓存获取模式
    fn get_cached_schemas(&self) -> Result<Option<HashMap<String, TableSchema>>, AuthError> {
        let cache = self.cache.read().map_err(|e| {
            AuthError::DatabaseError(format!("读取缓存锁失败: {}", e))
        })?;

        if let Some(ref entry) = *cache {
            if !entry.is_expired(self.config.cache_ttl_seconds) {
                // 检查文件是否有变化
                if self.check_file_changes(entry)? {
                    return Ok(Some(entry.schemas.clone()));
                }
            }
        }

        Ok(None)
    }

    /// 检查文件变化
    fn check_file_changes(&self, cache_entry: &CacheEntry) -> Result<bool, AuthError> {
        for schema_dir in &self.config.schema_dirs {
            if !schema_dir.exists() {
                continue;
            }

            for entry in fs::read_dir(schema_dir).map_err(|e| {
                AuthError::DatabaseError(format!("无法读取schema目录: {}", e))
            })? {
                let entry = entry.map_err(|e| {
                    AuthError::DatabaseError(format!("无法读取目录项: {}", e))
                })?;
                let path = entry.path();

                if self.is_supported_file(&path) {
                    let current_checksum = self.calculate_file_checksum(&path)?;
                    if let Some(cached_checksum) = cache_entry.file_checksums.get(&path) {
                        if cached_checksum != &current_checksum {
                            debug!("文件已变化: {:?}", path);
                            return Ok(false);
                        }
                    } else {
                        debug!("发现新文件: {:?}", path);
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }

    /// 计算文件校验和
    fn calculate_file_checksum(&self, path: &Path) -> Result<String, AuthError> {
        let content = fs::read_to_string(path).map_err(|e| {
            AuthError::DatabaseError(format!("无法读取文件 {:?}: {}", path, e))
        })?;

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// 更新缓存
    fn update_cache(&self, schemas: HashMap<String, TableSchema>) -> Result<(), AuthError> {
        let mut file_checksums = HashMap::new();

        // 计算所有文件的校验和
        for schema_dir in &self.config.schema_dirs {
            if !schema_dir.exists() {
                continue;
            }

            for entry in fs::read_dir(schema_dir).map_err(|e| {
                AuthError::DatabaseError(format!("无法读取schema目录: {}", e))
            })? {
                let entry = entry.map_err(|e| {
                    AuthError::DatabaseError(format!("无法读取目录项: {}", e))
                })?;
                let path = entry.path();

                if self.is_supported_file(&path) {
                    let checksum = self.calculate_file_checksum(&path)?;
                    file_checksums.insert(path, checksum);
                }
            }
        }

        let cache_entry = CacheEntry {
            schemas,
            loaded_at: SystemTime::now(),
            file_checksums,
        };

        let mut cache = self.cache.write().map_err(|e| {
            AuthError::DatabaseError(format!("写入缓存锁失败: {}", e))
        })?;
        *cache = Some(cache_entry);

        debug!("缓存已更新");
        Ok(())
    }

    /// 从文件加载模式
    fn load_schemas_from_files(&self) -> Result<HashMap<String, TableSchema>, AuthError> {
        let mut schemas = HashMap::new();

        for schema_dir in &self.config.schema_dirs {
            if !schema_dir.exists() {
                debug!("Schema目录不存在: {:?}", schema_dir);
                continue;
            }

            debug!("从目录加载模式: {:?}", schema_dir);
            let dir_schemas = self.load_schemas_from_directory(schema_dir)?;
            schemas.extend(dir_schemas);
        }

        Ok(schemas)
    }

    /// 从目录加载模式
    fn load_schemas_from_directory(&self, dir: &Path) -> Result<HashMap<String, TableSchema>, AuthError> {
        let mut schemas = HashMap::new();

        for entry in fs::read_dir(dir).map_err(|e| {
            AuthError::DatabaseError(format!("无法读取schema目录 {:?}: {}", dir, e))
        })? {
            let entry = entry.map_err(|e| {
                AuthError::DatabaseError(format!("无法读取目录项: {}", e))
            })?;
            let path = entry.path();

            if self.is_supported_file(&path) {
                match self.load_schema_from_file(&path) {
                    Ok(schema) => {
                        debug!("从文件加载表结构: {} <- {:?}", schema.name, path);
                        schemas.insert(schema.name.clone(), schema);
                    }
                    Err(e) => {
                        error!("加载文件 {:?} 失败: {}", path, e);
                        if self.config.validate_schemas {
                            return Err(e);
                        }
                    }
                }
            }
        }

        Ok(schemas)
    }

    /// 检查是否为支持的文件
    fn is_supported_file(&self, path: &Path) -> bool {
        path.is_file() && 
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.config.supported_extensions.contains(&ext.to_lowercase()))
            .unwrap_or(false)
    }

    /// 从文件加载单个模式
    fn load_schema_from_file(&self, path: &Path) -> Result<TableSchema, AuthError> {
        let table_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| AuthError::DatabaseError("无效的schema文件名".to_string()))?
            .to_string();

        let content = fs::read_to_string(path).map_err(|e| {
            AuthError::DatabaseError(format!("无法读取schema文件 {:?}: {}", path, e))
        })?;

        // 尝试解析为JSON格式的扩展模式定义
        if let Ok(extended_schema) = serde_json::from_str::<TableSchema>(&content) {
            return Ok(extended_schema);
        }

        // 否则作为纯SQL处理
        let mut schema = TableSchema::new(&table_name, &content, "1.0.0");
        
        // 从注释中提取元数据
        self.extract_metadata_from_sql(&mut schema)?;

        Ok(schema)
    }

    /// 从SQL注释中提取元数据
    fn extract_metadata_from_sql(&self, schema: &mut TableSchema) -> Result<(), AuthError> {
        let lines: Vec<&str> = schema.sql.lines().collect();
        
        for line in lines {
            let line = line.trim();
            if line.starts_with("-- @description:") {
                schema.description = Some(line[16..].trim().to_string());
            } else if line.starts_with("-- @version:") {
                schema.version = line[12..].trim().to_string();
            } else if line.starts_with("-- @depends:") {
                let deps: Vec<String> = line[12..]
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                schema.dependencies = deps;
            }
        }

        Ok(())
    }

    /// 从模块加载模式
    fn load_schemas_from_modules(&self) -> Result<HashMap<String, TableSchema>, AuthError> {
        let mut all_schemas = HashMap::new();

        for (module_name, loader) in &self.module_loaders {
            debug!("从模块加载表结构: {}", module_name);
            
            let module_schemas = loader();
            let count = module_schemas.len();
            
            all_schemas.extend(module_schemas);
            
            if count > 0 {
                debug!("模块 {} 提供了 {} 个表结构", module_name, count);
            }
        }

        Ok(all_schemas)
    }

    /// 验证所有模式
    fn validate_all_schemas(&self, schemas: &HashMap<String, TableSchema>) -> Result<(), AuthError> {
        debug!("开始验证表结构");
        
        let mut errors = Vec::new();

        for (name, schema) in schemas {
            if let Err(e) = schema.validate_sql() {
                errors.push(format!("表 {}: {}", name, e));
            }
        }

        if !errors.is_empty() {
            return Err(AuthError::DatabaseError(format!(
                "模式验证失败:\n{}",
                errors.join("\n")
            )));
        }

        debug!("所有表结构验证通过");
        Ok(())
    }

    /// 解析依赖关系并排序
    fn resolve_dependencies(&self, schemas: &HashMap<String, TableSchema>) -> Result<HashMap<String, TableSchema>, AuthError> {
        let mut ordered = HashMap::new();
        let mut visited = HashSet::new();
        let mut visiting = HashSet::new();

        // 拓扑排序
        for schema_name in schemas.keys() {
            self.visit_schema(schema_name, schemas, &mut visited, &mut visiting, &mut ordered)?;
        }

        Ok(ordered)
    }

    /// 访问模式（用于拓扑排序）
    fn visit_schema(
        &self,
        name: &str,
        schemas: &HashMap<String, TableSchema>,
        visited: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
        ordered: &mut HashMap<String, TableSchema>,
    ) -> Result<(), AuthError> {
        if visited.contains(name) {
            return Ok(());
        }

        if visiting.contains(name) {
            return Err(AuthError::DatabaseError(format!(
                "检测到循环依赖: {}", name
            )));
        }

        if let Some(schema) = schemas.get(name) {
            visiting.insert(name.to_string());

            // 先处理依赖
            for dep in &schema.dependencies {
                self.visit_schema(dep, schemas, visited, visiting, ordered)?;
            }

            visiting.remove(name);
            visited.insert(name.to_string());
            ordered.insert(name.to_string(), schema.clone());
        }

        Ok(())
    }

    /// 清除缓存
    pub fn clear_cache(&self) -> Result<(), AuthError> {
        let mut cache = self.cache.write().map_err(|e| {
            AuthError::DatabaseError(format!("写入缓存锁失败: {}", e))
        })?;
        *cache = None;
        info!("缓存已清除");
        Ok(())
    }

    /// 获取模式统计信息
    pub fn get_schema_stats(&self) -> Result<SchemaStats, AuthError> {
        let schemas = self.load_all_schemas()?;
        
        let mut stats = SchemaStats {
            total_schemas: schemas.len(),
            file_schemas: 0,
            module_schemas: 0,
            schemas_with_dependencies: 0,
            total_dependencies: 0,
            schema_versions: HashMap::new(),
        };

        for schema in schemas.values() {
            if !schema.dependencies.is_empty() {
                stats.schemas_with_dependencies += 1;
                stats.total_dependencies += schema.dependencies.len();
            }

            *stats.schema_versions.entry(schema.version.clone()).or_insert(0) += 1;
        }

        Ok(stats)
    }
}

/// 模式统计信息
#[derive(Debug, Serialize)]
pub struct SchemaStats {
    pub total_schemas: usize,
    pub file_schemas: usize,
    pub module_schemas: usize,
    pub schemas_with_dependencies: usize,
    pub total_dependencies: usize,
    pub schema_versions: HashMap<String, usize>,
}

/// 生成默认schema文件
pub fn generate_default_schemas(schema_dir: &Path) -> Result<(), AuthError> {
    info!("生成默认schema文件到: {:?}", schema_dir);

    // 确保目录存在
    if !schema_dir.exists() {
        fs::create_dir_all(schema_dir).map_err(|e| {
            AuthError::DatabaseError(format!("无法创建schema目录: {}", e))
        })?;
    }

    let config = SchemaLoaderConfig::default();
    let loader = SchemaLoader::new(config);
    let schemas = loader.load_all_schemas()?;

    let mut generated_count = 0;

    // 写入文件
    for (table_name, schema) in schemas {
        let file_path = schema_dir.join(format!("{}.json", table_name));
        
        let json_content = serde_json::to_string_pretty(&schema).map_err(|e| {
            AuthError::DatabaseError(format!("序列化schema失败: {}", e))
        })?;

        fs::write(&file_path, json_content).map_err(|e| {
            AuthError::DatabaseError(format!("无法写入schema文件: {}", e))
        })?;

        debug!("生成schema文件: {:?}", file_path);
        generated_count += 1;
    }

    info!("成功生成 {} 个schema文件", generated_count);
    Ok(())
}

/// 便利函数：加载所有模式
pub fn load_all_schemas() -> Result<HashMap<String, TableSchema>, AuthError> {
    let config = SchemaLoaderConfig::default();
    let loader = SchemaLoader::new(config);
    loader.load_all_schemas()
}

/// 便利函数：获取模式统计
pub fn get_schema_statistics() -> Result<SchemaStats, AuthError> {
    let config = SchemaLoaderConfig::default();
    let loader = SchemaLoader::new(config);
    loader.get_schema_stats()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_schema_creation() {
        let schema = TableSchema::new("test_table", "CREATE TABLE test (id INTEGER)", "1.0.0");
        assert_eq!(schema.name, "test_table");
        assert_eq!(schema.version, "1.0.0");
        assert!(!schema.checksum.is_empty());
    }

    #[test]
    fn test_schema_validation() {
        let valid_schema = TableSchema::new(
            "test_table",
            "CREATE TABLE test (id INTEGER PRIMARY KEY)",
            "1.0.0"
        );
        assert!(valid_schema.validate_sql().is_ok());

        let invalid_schema = TableSchema::new(
            "test_table",
            "SELECT * FROM test",
            "1.0.0"
        );
        assert!(invalid_schema.validate_sql().is_err());
    }

    #[test]
    fn test_schema_loader() {
        let config = SchemaLoaderConfig::default();
        let loader = SchemaLoader::new(config);
        
        // 这个测试依赖于模块加载器的存在
        let result = loader.load_schemas_from_modules();
        assert!(result.is_ok());
    }

    #[test]
    fn test_dependency_resolution() {
        let config = SchemaLoaderConfig::default();
        let loader = SchemaLoader::new(config);
        
        let mut schemas = HashMap::new();
        
        let schema_a = TableSchema::new("table_a", "CREATE TABLE table_a (id INTEGER)", "1.0.0");
        let schema_b = TableSchema::new("table_b", "CREATE TABLE table_b (id INTEGER)", "1.0.0")
            .with_dependencies(vec!["table_a".to_string()]);
        
        schemas.insert("table_a".to_string(), schema_a);
        schemas.insert("table_b".to_string(), schema_b);
        
        let result = loader.resolve_dependencies(&schemas);
        assert!(result.is_ok());
    }

    #[test]
    fn test_circular_dependency_detection() {
        let config = SchemaLoaderConfig::default();
        let loader = SchemaLoader::new(config);
        
        let mut schemas = HashMap::new();
        
        let schema_a = TableSchema::new("table_a", "CREATE TABLE table_a (id INTEGER)", "1.0.0")
            .with_dependencies(vec!["table_b".to_string()]);
        let schema_b = TableSchema::new("table_b", "CREATE TABLE table_b (id INTEGER)", "1.0.0")
            .with_dependencies(vec!["table_a".to_string()]);
        
        schemas.insert("table_a".to_string(), schema_a);
        schemas.insert("table_b".to_string(), schema_b);
        
        let result = loader.resolve_dependencies(&schemas);
        assert!(result.is_err());
    }
}