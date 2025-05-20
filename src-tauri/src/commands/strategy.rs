use tauri::{command, State, AppHandle};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::strategy_service::StrategyService;
use crate::models::strategy::{
    Strategy, StrategyVersion, StrategyTag, StrategyRating,
    StrategyApplication, CreateStrategyRequest, UpdateStrategyRequest
};

#[derive(Debug, Deserialize)]
pub struct RateStrategyParams {
    pub strategy_id: i64,
    pub rating: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ApplyStrategyParams {
    pub strategy_id: i64,
    pub asset_id: i64,
}

#[command]
pub fn create_strategy(
    request: CreateStrategyRequest,
    app_handle: AppHandle,
) -> Result<i64, String> {
    // 获取当前用户ID
    let user_id = 1; // 从应用状态获取当前用户ID
    
    // 创建策略服务
    let strategy_service = StrategyService::new();
    
    // 创建策略
    strategy_service.create_strategy(user_id, request)
}

#[command]
pub fn update_strategy(
    request: UpdateStrategyRequest,
    app_handle: AppHandle,
) -> Result<(), String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.update_strategy(user_id, request)
}

#[command]
pub fn delete_strategy(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<(), String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.delete_strategy(user_id, strategy_id)
}

#[command]
pub fn get_strategy(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<Strategy, String> {
    let strategy_service = StrategyService::new();
    strategy_service.get_strategy(strategy_id)
}

#[command]
pub fn get_user_strategies(
    app_handle: AppHandle,
) -> Result<Vec<Strategy>, String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.get_user_strategies(user_id)
}

#[command]
pub fn get_public_strategies(
    page: usize,
    page_size: usize,
    app_handle: AppHandle,
) -> Result<Vec<Strategy>, String> {
    let strategy_service = StrategyService::new();
    strategy_service.get_public_strategies(page, page_size)
}

#[command]
pub fn get_strategy_versions(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<Vec<StrategyVersion>, String> {
    let strategy_service = StrategyService::new();
    strategy_service.get_strategy_versions(strategy_id)
}

#[command]
pub fn add_strategy_tag(
    strategy_id: i64,
    tag_name: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let strategy_service = StrategyService::new();
    strategy_service.add_strategy_tag(strategy_id, &tag_name)
}

#[command]
pub fn remove_strategy_tag(
    strategy_id: i64,
    tag_name: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let strategy_service = StrategyService::new();
    strategy_service.remove_strategy_tag(strategy_id, &tag_name)
}

#[command]
pub fn get_strategy_tags(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<Vec<StrategyTag>, String> {
    let strategy_service = StrategyService::new();
    strategy_service.get_strategy_tags(strategy_id)
}

#[command]
pub fn favorite_strategy(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<(), String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.favorite_strategy(user_id, strategy_id)
}

#[command]
pub fn unfavorite_strategy(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<(), String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.unfavorite_strategy(user_id, strategy_id)
}

#[command]
pub fn get_user_favorite_strategies(
    app_handle: AppHandle,
) -> Result<Vec<Strategy>, String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.get_user_favorite_strategies(user_id)
}

#[command]
pub fn rate_strategy(
    params: RateStrategyParams,
    app_handle: AppHandle,
) -> Result<(), String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.rate_strategy(user_id, params.strategy_id, params.rating, params.comment)
}

#[command]
pub fn get_strategy_ratings(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<Vec<StrategyRating>, String> {
    let strategy_service = StrategyService::new();
    strategy_service.get_strategy_ratings(strategy_id)
}

#[command]
pub fn get_strategy_average_rating(
    strategy_id: i64,
    app_handle: AppHandle,
) -> Result<Option<f64>, String> {
    let strategy_service = StrategyService::new();
    strategy_service.get_strategy_average_rating(strategy_id)
}

#[command]
pub fn apply_strategy_to_asset(
    params: ApplyStrategyParams,
    app_handle: AppHandle,
) -> Result<i64, String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.apply_strategy_to_asset(user_id, params.strategy_id, params.asset_id)
}

#[command]
pub fn deactivate_strategy_application(
    application_id: i64,
    app_handle: AppHandle,
) -> Result<(), String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.deactivate_strategy_application(user_id, application_id)
}

#[command]
pub fn get_asset_strategies(
    asset_id: i64,
    app_handle: AppHandle,
) -> Result<Vec<StrategyApplication>, String> {
    let user_id = 1; // 从应用状态获取当前用户ID
    let strategy_service = StrategyService::new();
    strategy_service.get_asset_strategies(user_id, asset_id)
}
