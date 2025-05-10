#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]
  
  mod api;
  mod auth;
  mod config;
  mod database;
  mod error;
  mod models;
  mod utils;
  
  use crate::api::auth::{
      forgot_password_command, login_command, logout_command, 
      register_command, reset_password_command, verify_session_command
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
              register_command,
              login_command,
              logout_command,
              forgot_password_command,
              reset_password_command,
              verify_session_command
          ])
          .run(tauri::generate_context!())
          .expect("Error while running tauri application");
  }