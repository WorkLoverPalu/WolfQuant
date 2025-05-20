// src/commands/import.rs
use chrono::{DateTime, Utc};
use tauri::{command, State};

use crate::core::Engine;
use crate::import::{ImportTask, AvailableData};

#[command]
pub async fn start_import(
    asset_type: String,
    symbol: String,
    source: String,
    start_time: String,
    end_time: String,
    interval: String,
    engine: State<'_, Engine>,
) -> Result<ImportTask, String> {
    let start = DateTime::parse_from_rfc3339(&start_time)
        .map_err(|e| format!("Invalid start time: {}", e))?
        .with_timezone(&Utc);
    
    let end = DateTime::parse_from_rfc3339(&end_time)
        .map_err(|e| format!("Invalid end time: {}", e))?
        .with_timezone(&Utc);
    
    engine.start_import(&asset_type, &symbol, &source, start, end, &interval).await
}

#[command]
pub async fn get_import_task(
    id: String,
    engine: State<'_, Engine>,
) -> Result<Option<ImportTask>, String> {
    engine.get_import_task(&id).await
}

#[command]
pub async fn get_import_tasks(
    engine: State<'_, Engine>,
) -> Result<Vec<ImportTask>, String> {
    engine.get_import_tasks().await
}

#[command]
pub async fn get_available_data(
    engine: State<'_, Engine>,
) -> Result<Vec<AvailableData>, String> {
    engine.get_available_data().await
}