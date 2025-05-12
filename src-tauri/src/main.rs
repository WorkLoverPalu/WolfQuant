#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// 在 main.rs 中添加
#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

mod api;
mod auth;
mod config;
mod database;
mod error;
mod models;
mod utils;
// 导入api相关的crate
use crate::api::auth::{
    forgot_password_command, login_command, logout_command, register_command,
    reset_password_command, verify_session_command,
};
use crate::config::Config;
use tauri::{command, State};

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
            //注册api有效性
            register_command,
            login_command,
            logout_command,
            forgot_password_command,
            reset_password_command,
            verify_session_command
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Regular);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
