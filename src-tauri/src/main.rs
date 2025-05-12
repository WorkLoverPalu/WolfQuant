#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
//兼容适配
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

mod config;
mod commands;
mod database;
mod error;
mod models;
mod services;
mod utils;

// 数据
use commands::data::{
    cmd_create_trade_alert,
    cmd_get_asset_price_history,
    cmd_get_portfolio_summary,
    cmd_get_user_trade_alerts,
    cmd_mark_alert_read,
    //
    cmd_update_asset_price,
    cmd_update_asset_price_batch,
};
//登陆
use commands::auth::{
    forgot_password_command,
    login_command,
    logout_command,
    register_command,
    reset_password_command,
    verify_session_command,
};
use crate::config::config::Config;
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
            forgot_password_command,
            login_command,
            logout_command,
            register_command,
            reset_password_command,
            verify_session_command,
            //其他
            cmd_update_asset_price,
            cmd_update_asset_price_batch,
            cmd_get_asset_price_history,
            cmd_create_trade_alert,
            cmd_mark_alert_read,
            cmd_get_user_trade_alerts,
            cmd_get_portfolio_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
