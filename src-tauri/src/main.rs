// 标书格式一键优化工具
// 主入口文件
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)
//
// Prevents additional console window on Windows in release
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod services;
mod models;
mod security;

use tauri::Manager;
use std::sync::Arc;
use parking_lot::Mutex;

/// 应用状态
pub struct AppState {
    pub config: Arc<Mutex<AppConfig>>,
    pub debug_config: Arc<Mutex<models::debug::DebugConfig>>,
}

/// 应用配置
#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub api_provider: String,
    pub api_key: String,
    pub api_model: String,
    pub output_mode: String,
    pub auto_backup: bool,
    // 备用模型配置
    pub enable_backup: bool,
    pub backup_provider: String,
    pub backup_api_key: String,
    pub backup_model: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 初始化应用状态
            let state = AppState {
                config: Arc::new(Mutex::new(AppConfig::default())),
                debug_config: Arc::new(Mutex::new(models::debug::DebugConfig::default())),
            };
            
            app.manage(state);
            
            println!("标书格式优化工具 v1.0.0 启动成功");
            println!("设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 格式提取
            commands::format::extract_format_from_tender,
            commands::format::analyze_document_format,
            commands::format::compare_format_diff,
            commands::format::apply_format,
            
            // 模板管理
            commands::template::get_templates,
            commands::template::save_template,
            commands::template::delete_template,
            
            // 文档操作
            commands::document::open_document,
            commands::document::save_document,
            commands::document::backup_document,
            
            // 配置
            commands::config::load_config,
            commands::config::save_config,
            commands::config::test_api_connection,
            
            // Debug 调试
            commands::debug::get_debug_config,
            commands::debug::save_debug_config,
            commands::debug::load_debug_config,
            commands::debug::read_log_file,
            commands::debug::clear_log_file,
            commands::debug::get_log_path,
            commands::debug::open_log_directory,
            commands::debug::get_log_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
