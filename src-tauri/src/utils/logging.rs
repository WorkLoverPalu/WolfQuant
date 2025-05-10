use crate::config::Config;
use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config as LogConfig, Root},
    encode::pattern::PatternEncoder,
};
use std::fs;
use std::path::Path;

pub fn init_logger() -> Result<(), SetLoggerError> {
    let config = Config::get();
    
    if !config.logging.enabled {
        return Ok(());
    }
    
    let level = match config.logging.level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };
    
    let mut appenders = Vec::new();
    let mut root_builder = Root::builder();
    
    // 控制台日志
    if config.logging.console_output {
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
            .build();
        
        appenders.push(Appender::builder().build("console", Box::new(console)));
        root_builder = root_builder.appender("console");
    }
    
    // 文件日志
    if config.logging.file_output {
        let log_path = Path::new(&config.logging.log_file_path);
        
        // 确保日志目录存在
        if let Some(parent) = log_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap_or_else(|e| {
                    eprintln!("Failed to create log directory: {}", e);
                });
            }
        }
        
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
            .build(log_path)
            .unwrap_or_else(|e| {
                eprintln!("Failed to create log file appender: {}", e);
                FileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
                    .build("app.log")
                    .expect("Failed to create fallback log file")
            });
        
        appenders.push(Appender::builder().build("file", Box::new(file)));
        root_builder = root_builder.appender("file");
    }
    
    let log_config = LogConfig::builder()
        .appenders(appenders)
        .build(root_builder.build(level))
        .unwrap();
    
    log4rs::init_config(log_config)?;
    
    Ok(())
}