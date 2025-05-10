use crate::config;
use chrono::Local;
use log::{Level, LevelFilter, Metadata, Record};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;

// 自定义日志记录器
struct AppLogger {
    level: LevelFilter,
    log_to_file: bool,
    file: Option<Mutex<File>>,
}

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let log_message = format!(
                "[{}] {} - {}: {}\n",
                now,
                record.level(),
                record.target(),
                record.args()
            );
            
            // 输出到控制台
            print!("{}", log_message);
            
            // 输出到文件
            if self.log_to_file {
                if let Some(file) = &self.file {
                    let mut file = file.lock().unwrap();
                    let _ = file.write_all(log_message.as_bytes());
                    let _ = file.flush();
                }
            }
        }
    }

    fn flush(&self) {
        if self.log_to_file {
            if let Some(file) = &self.file {
                let mut file = file.lock().unwrap();
                let _ = file.flush();
            }
        }
    }
}

// 初始化日志系统
pub fn init_logger() -> Result<(), String> {
    let config = config::get_config()?;
    
    if !config.logging.enable {
        return Ok(());
    }
    
    // 设置日志级别
    let level = match config.logging.level.to_lowercase().as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    
    // 如果需要输出到文件，打开或创建日志文件
    let file = if config.logging.log_to_file {
        let app_dir = tauri::api::path::app_dir(tauri::Config::default().package.name.as_str())
            .ok_or_else(|| "Failed to get app directory".to_string())?;
        
        let log_path = app_dir.join(&config.logging.file_path);
        
        // 确保日志目录存在
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create log directory: {}", e))?;
        }
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| format!("Failed to open log file: {}", e))?;
        
        Some(Mutex::new(file))
    } else {
        None
    };
    
    // 创建并设置日志记录器
    let logger = Box::new(AppLogger {
        level,
        log_to_file: config.logging.log_to_file,
        file,
    });
    
    log::set_boxed_logger(logger)
        .map(|()| log::set_max_level(level))
        .map_err(|e| format!("Failed to set logger: {}", e))?;
    
    log::info!("Logger initialized with level: {}", config.logging.level);
    
    Ok(())
}