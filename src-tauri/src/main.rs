#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
//兼容适配
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

mod commands;
mod config;
mod database;
mod error;
mod models;
mod services;
mod utils;

// 数据
use commands::data::{
    data_create_trade_alert,
    data_get_asset_price_history,
    data_get_portfolio_summary,
    data_get_user_trade_alerts,
    data_mark_alert_read,
    //
    data_update_asset_price,
    data_update_asset_price_batch,
};
//登陆
use crate::config::Config;
use commands::auth::{
    auth_forgot_password_command, auth_login_command, auth_logout_command, auth_register_command,
    auth_reset_password_command, auth_verify_session_command,
};
fn main() {
    // 加载配置
    if let Err(e) = Config::load() {
        eprintln!("Failed to load config: {}", e);
    }

    // 初始化日志系统
    if let Err(e) = utils::logging::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    // 初始化数据库
    if let Err(e) = database::init_database() {
        eprintln!("Failed to initialize database: {}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            //登陆
            auth_forgot_password_command,
            auth_login_command,
            auth_logout_command,
            auth_register_command,
            auth_reset_password_command,
            auth_verify_session_command,
            //其他
            data_update_asset_price,
            data_update_asset_price_batch,
            data_get_asset_price_history,
            data_create_trade_alert,
            data_mark_alert_read,
            data_get_user_trade_alerts,
            data_get_portfolio_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
