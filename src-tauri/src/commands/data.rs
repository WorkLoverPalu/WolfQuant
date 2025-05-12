use crate::error::auth::ErrorResponse;
use crate::models::{
    GetAssetPriceHistoryRequest, MarkAlertReadRequest, MessageResponse, PortfolioSummary,
    PriceHistory, TradeAlert,
};
use crate::services::data::{
    create_trade_alert, get_asset_price_history, get_portfolio_summary, get_user_trade_alerts,
    mark_alert_read, update_asset_price, update_asset_price_batch,
};
use serde_json::Value;

#[tauri::command]
pub async fn data_update_asset_price(
    asset_id: i64,
    price: f64,
    date: i64,
) -> Result<MessageResponse, ErrorResponse> {
    match update_asset_price(asset_id, price, date) {
        Ok(_) => Ok(MessageResponse {
            message: format!("Price for {} updated successfully", asset_id),
        }),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn data_update_asset_price_batch(
    prices: Vec<(i64, f64, i64)>,
) -> Result<MessageResponse, ErrorResponse> {
    match update_asset_price_batch(&prices) {
        Ok(_) => Ok(MessageResponse {
            message: "Batch price update completed successfully".to_string(),
        }),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn data_get_asset_price_history(
    request: GetAssetPriceHistoryRequest,
) -> Result<Vec<PriceHistory>, ErrorResponse> {
    match get_asset_price_history(request.asset_id, request.start_date, request.end_date) {
        Ok(history) => Ok(history),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn data_create_trade_alert(
    user_id: &str,
    asset_id: i64,
    strategy_id: Option<i64>,
    alert_type: &str,
    message: &str,
) -> Result<TradeAlert, ErrorResponse> {
    match create_trade_alert(user_id, asset_id, strategy_id, alert_type, message) {
        Ok(alert) => Ok(alert),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn data_mark_alert_read(
    request: MarkAlertReadRequest,
) -> Result<MessageResponse, ErrorResponse> {
    match mark_alert_read(request.id, &request.user_id) {
        Ok(_) => Ok(MessageResponse {
            message: "Alert marked as read".to_string(),
        }),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn data_get_user_trade_alerts(
    user_id: &str,
    is_read: Option<bool>,
    limit: Option<i64>,
) -> Result<Vec<TradeAlert>, ErrorResponse> {
    match get_user_trade_alerts(user_id, is_read, limit) {
        Ok(alerts) => Ok(alerts),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn data_get_portfolio_summary(user_id: &str) -> Result<PortfolioSummary, ErrorResponse> {
    match get_portfolio_summary(user_id) {
        Ok(summary) => Ok(summary),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}
