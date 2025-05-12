use crate::error::ErrorResponse;
use crate::models::{
    PriceHistory, TradeAlert, PortfolioSummary, GetAssetPriceHistoryRequest,
    MarkAlertReadRequest, MessageResponse,
};
use crate::services::data::{
    update_asset_price, update_asset_price_batch, get_asset_price_history,
    create_trade_alert, mark_alert_read, get_user_trade_alerts, get_portfolio_summary,
};
use serde_json::Value;

#[tauri::command]
pub async fn cmd_update_asset_price(
    symbol: String,
    price: f64,
) -> Result<MessageResponse, ErrorResponse> {
    match update_asset_price(&symbol, price).await {
        Ok(_) => Ok(MessageResponse {
            message: format!("Price for {} updated successfully", symbol),
        }),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn cmd_update_asset_price_batch(
    prices: Vec<(String, f64)>,
) -> Result<MessageResponse, ErrorResponse> {
    match update_asset_price_batch(prices).await {
        Ok(_) => Ok(MessageResponse {
            message: "Batch price update completed successfully".to_string(),
        }),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn cmd_get_asset_price_history(
    request: GetAssetPriceHistoryRequest,
) -> Result<Vec<PriceHistory>, ErrorResponse> {
    match get_asset_price_history(&request.symbol, &request.timeframe, request.limit).await {
        Ok(history) => Ok(history),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn cmd_create_trade_alert(
    symbol: String,
    alert_type: String,
    price_target: f64,
    message: String,
    user_id: i32,
) -> Result<TradeAlert, ErrorResponse> {
    match create_trade_alert(&symbol, &alert_type, price_target, &message, user_id).await {
        Ok(alert) => Ok(alert),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn cmd_mark_alert_read(
    request: MarkAlertReadRequest,
) -> Result<MessageResponse, ErrorResponse> {
    match mark_alert_read(request.alert_id).await {
        Ok(_) => Ok(MessageResponse {
            message: "Alert marked as read".to_string(),
        }),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn cmd_get_user_trade_alerts(
    user_id: i32,
    include_read: bool,
) -> Result<Vec<TradeAlert>, ErrorResponse> {
    match get_user_trade_alerts(user_id, include_read).await {
        Ok(alerts) => Ok(alerts),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[tauri::command]
pub async fn cmd_get_portfolio_summary(
    user_id: i32,
) -> Result<PortfolioSummary, ErrorResponse> {
    match get_portfolio_summary(user_id).await {
        Ok(summary) => Ok(summary),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}