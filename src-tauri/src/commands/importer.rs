use tauri::{command, State, AppHandle};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::services::importer_service::ImporterService;
use crate::models::import_task::ImportTask;
use crate::models::dataset::DatasetInfo;
use crate::event::EventBus;

#[derive(Debug, Deserialize)]
pub struct StartImportParams {
    pub asset_type: String,
    pub symbol: String,
    pub source: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub interval: String,
}

#[command]
pub fn start_import(
    params: StartImportParams,
    app_handle: AppHandle,
) -> Result<ImportTask, String> {
    // 获取事件总线
    let event_bus = app_handle.state::<Arc<EventBus>>();
    
    // 创建导入服务
    let importer_service = ImporterService::new(event_bus.inner().clone());
    
    // 启动导入任务
    importer_service.start_import(
        params.asset_type,
        params.symbol,
        params.source,
        params.start_time,
        params.end_time,
        params.interval,
    )
}

#[command]
pub fn get_import_task(
    task_id: String,
    app_handle: AppHandle,
) -> Result<Option<ImportTask>, String> {
    let event_bus = app_handle.state::<Arc<EventBus>>();
    let importer_service = ImporterService::new(event_bus.inner().clone());
    importer_service.get_task(&task_id)
}

#[command]
pub fn get_import_tasks(
    app_handle: AppHandle,
) -> Result<Vec<ImportTask>, String> {
    let event_bus = app_handle.state::<Arc<EventBus>>();
    let importer_service = ImporterService::new(event_bus.inner().clone());
    importer_service.get_tasks()
}

#[command]
pub fn get_available_datasets(
    app_handle: AppHandle,
) -> Result<Vec<DatasetInfo>, String> {
    let event_bus = app_handle.state::<Arc<EventBus>>();
    let importer_service = ImporterService::new(event_bus.inner().clone());
    importer_service.get_available_datasets()
}
