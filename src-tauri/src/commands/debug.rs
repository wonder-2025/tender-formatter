// Debug 调试命令
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use crate::models::debug::DebugConfig;
use crate::services::logger;
use tauri::State;
use crate::AppState;

/// 获取 Debug 配置
#[tauri::command]
pub fn get_debug_config(state: State<'_, AppState>) -> DebugConfig {
    let config = state.debug_config.lock();
    config.clone()
}

/// 保存 Debug 配置
#[tauri::command]
pub fn save_debug_config(
    config: DebugConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 更新内存中的配置
    {
        let mut debug_config = state.debug_config.lock();
        *debug_config = config.clone();
    }
    
    // 初始化日志系统
    logger::init_logger(&config)?;
    
    // 保存到文件
    save_debug_config_to_file(&config)?;
    
    Ok(())
}

/// 加载 Debug 配置
#[tauri::command]
pub fn load_debug_config(state: State<'_, AppState>) -> DebugConfig {
    let config = load_debug_config_from_file();
    
    // 更新内存配置
    {
        let mut debug_config = state.debug_config.lock();
        *debug_config = config.clone();
    }
    
    // 初始化日志
    let _ = logger::init_logger(&config);
    
    config
}

/// 获取日志内容
#[tauri::command]
pub fn read_log_file(lines: Option<usize>) -> Result<String, String> {
    logger::read_log_file(lines)
}

/// 清空日志
#[tauri::command]
pub fn clear_log_file() -> Result<(), String> {
    logger::clear_log_file()
}

/// 获取日志文件路径
#[tauri::command]
pub fn get_log_path() -> String {
    logger::get_log_path()
}

/// 打开日志文件所在目录
#[tauri::command]
pub fn open_log_directory() -> Result<String, String> {
    let log_dir = crate::models::debug::get_log_directory();
    
    // 尝试用系统默认程序打开目录
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| format!("无法打开目录: {}", e))?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| format!("无法打开目录: {}", e))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&log_dir)
            .spawn()
            .map_err(|e| format!("无法打开目录: {}", e))?;
    }
    
    Ok(log_dir)
}

/// 获取日志统计信息
#[tauri::command]
pub fn get_log_stats() -> LogStats {
    let log_dir = crate::models::debug::get_log_directory();
    let log_path = std::path::Path::new(&log_dir);
    
    let mut total_size: u64 = 0;
    let mut file_count: u32 = 0;
    let mut earliest_date: Option<String> = None;
    let mut latest_date: Option<String> = None;
    
    if log_path.exists() {
        if let Ok(entries) = std::fs::read_dir(log_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "log").unwrap_or(false) {
                    if let Ok(metadata) = entry.metadata() {
                        total_size += metadata.len();
                        file_count += 1;
                    }
                    
                    // 从文件名提取日期
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        // 文件名格式: app-YYYY-MM-DD.log
                        if name.starts_with("app-") && name.ends_with(".log") {
                            let date_part = &name[4..name.len() - 4];
                            if latest_date.is_none() {
                                latest_date = Some(date_part.to_string());
                            }
                            earliest_date = Some(date_part.to_string());
                        }
                    }
                }
            }
        }
    }
    
    LogStats {
        total_size,
        file_count,
        earliest_date,
        latest_date,
        log_directory: log_dir,
    }
}

/// 日志统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogStats {
    /// 总大小（字节）
    pub total_size: u64,
    
    /// 文件数量
    pub file_count: u32,
    
    /// 最早日期
    pub earliest_date: Option<String>,
    
    /// 最新日期
    pub latest_date: Option<String>,
    
    /// 日志目录
    pub log_directory: String,
}

// 配置文件操作

/// 保存 Debug 配置到文件
fn save_debug_config_to_file(config: &DebugConfig) -> Result<(), String> {
    let config_dir = crate::models::debug::get_log_directory()
        .replace("logs", "config");
    
    // 确保目录存在
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;
    
    let config_path = format!("{}/debug.json", config_dir);
    
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    
    std::fs::write(&config_path, content)
        .map_err(|e| format!("写入配置失败: {}", e))?;
    
    Ok(())
}

/// 从文件加载 Debug 配置
fn load_debug_config_from_file() -> DebugConfig {
    let config_dir = crate::models::debug::get_log_directory()
        .replace("logs", "config");
    let config_path = format!("{}/debug.json", config_dir);
    
    if !std::path::Path::new(&config_path).exists() {
        return DebugConfig::default();
    }
    
    match std::fs::read_to_string(&config_path) {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_default()
        }
        Err(_) => DebugConfig::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_debug_config() {
        let config = DebugConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.log_level, "info");
    }
}
