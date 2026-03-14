// 日志记录器服务
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use crate::models::debug::{DebugConfig, get_log_file_path, mask_sensitive};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use chrono::Local;
use parking_lot::RwLock;

/// 全局日志配置
static LOGGER: once_cell::sync::Lazy<Arc<RwLock<Logger>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(Logger::new())));

/// 日志记录器
pub struct Logger {
    config: DebugConfig,
    initialized: bool,
}

impl Logger {
    fn new() -> Self {
        Self {
            config: DebugConfig::default(),
            initialized: false,
        }
    }
    
    /// 初始化日志
    pub fn init(&mut self, config: &DebugConfig) -> Result<(), String> {
        self.config = config.clone();
        
        if !config.enabled {
            self.initialized = false;
            return Ok(());
        }
        
        // 确保日志目录存在
        let log_path = &config.log_file_path;
        if let Some(parent) = Path::new(log_path).parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("创建日志目录失败: {}", e));
            }
        }
        
        // 清理旧日志
        if let Err(e) = cleanup_old_logs() {
            eprintln!("清理旧日志失败: {}", e);
        }
        
        self.initialized = true;
        Ok(())
    }
    
    /// 写入日志
    fn write_log(&self, category: &str, content: &str) {
        if !self.initialized || !self.config.enabled {
            return;
        }
        
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level = self.config.log_level.to_uppercase();
        
        let log_entry = format!(
            "[{}] [{}] [{}] {}\n",
            timestamp, level, category, content
        );
        
        let log_path = get_log_file_path();
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
        {
            let _ = file.write_all(log_entry.as_bytes());
        }
    }
    
    /// 检查是否应该记录
    fn should_log(&self, item: &str) -> bool {
        self.config.should_log(item)
    }
}

/// 初始化日志
pub fn init_logger(config: &DebugConfig) -> Result<(), String> {
    let mut logger = LOGGER.write();
    logger.init(config)
}

/// 获取当前配置
pub fn get_current_config() -> DebugConfig {
    let logger = LOGGER.read();
    logger.config.clone()
}

/// 记录脱敏过程
pub fn log_desensitize(original: &str, desensitized: &str, rules: &[String]) {
    let logger = LOGGER.read();
    if !logger.should_log("desensitize") {
        return;
    }
    
    let content = format!(
        r#"
  Original: {}
  Desensitized: {}
  Applied rules: {:?}"#,
        original,
        desensitized,
        rules
    );
    
    logger.write_log("DESENSITIZE", &content);
}

/// 记录 API 请求
pub fn log_api_request(
    provider: &str,
    endpoint: &str,
    headers: &str,
    body: &str,
    api_key: Option<&str>,
) {
    let logger = LOGGER.read();
    if !logger.should_log("apiRequest") {
        return;
    }
    
    // 脱敏 API Key
    let masked_key = api_key.map(|k| mask_sensitive(k, 4, 4));
    let auth_line = masked_key.map(|k| format!("\n  Authorization: Bearer {}...", k)).unwrap_or_default();
    
    let content = format!(
        r#"
  Provider: {}
  Endpoint: {}
  Headers: {}{}
  Body: {}"#,
        provider,
        endpoint,
        headers,
        auth_line,
        body
    );
    
    logger.write_log("API_REQUEST", &content);
}

/// 记录 API 响应
pub fn log_api_response(provider: &str, status: u16, body: &str, duration_ms: u64) {
    let logger = LOGGER.read();
    if !logger.should_log("apiResponse") {
        return;
    }
    
    let content = format!(
        r#"
  Provider: {}
  Status: {}
  Duration: {}ms
  Body: {}"#,
        provider,
        status,
        duration_ms,
        body
    );
    
    logger.write_log("API_RESPONSE", &content);
}

/// 记录文件操作
pub fn log_file_operation(operation: &str, path: &str, success: bool, error: Option<&str>) {
    let logger = LOGGER.read();
    if !logger.should_log("fileOperation") {
        return;
    }
    
    let status = if success { "成功" } else { "失败" };
    let error_line = error.map(|e| format!("\n  Error: {}", e)).unwrap_or_default();
    
    let content = format!(
        r#"
  Operation: {}
  Path: {}
  Status: {}{}"#,
        operation,
        path,
        status,
        error_line
    );
    
    logger.write_log("FILE_OPERATION", &content);
}

/// 记录格式修改
pub fn log_format_change(property: &str, old_value: &str, new_value: &str) {
    let logger = LOGGER.read();
    if !logger.should_log("formatChange") {
        return;
    }
    
    let content = format!(
        r#"
  Property: {}
  Old value: {}
  New value: {}"#,
        property,
        old_value,
        new_value
    );
    
    logger.write_log("FORMAT_CHANGE", &content);
}

/// 记录一般调试信息
pub fn log_debug(category: &str, message: &str) {
    let logger = LOGGER.read();
    if !logger.config.enabled {
        return;
    }
    
    logger.write_log(category, message);
}

/// 读取日志文件
pub fn read_log_file(lines: Option<usize>) -> Result<String, String> {
    let log_path = get_log_file_path();
    
    if !Path::new(&log_path).exists() {
        return Ok("日志文件不存在".to_string());
    }
    
    let content = fs::read_to_string(&log_path)
        .map_err(|e| format!("读取日志失败: {}", e))?;
    
    if let Some(n) = lines {
        let all_lines: Vec<&str> = content.lines().collect();
        let start = all_lines.len().saturating_sub(n);
        Ok(all_lines[start..].join("\n"))
    } else {
        Ok(content)
    }
}

/// 清空日志文件
pub fn clear_log_file() -> Result<(), String> {
    let log_path = get_log_file_path();
    
    if Path::new(&log_path).exists() {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&log_path)
            .map_err(|e| format!("清空日志失败: {}", e))?;
    }
    
    Ok(())
}

/// 获取日志文件路径
pub fn get_log_path() -> String {
    get_log_file_path()
}

/// 清理旧日志（保留最近7天）
fn cleanup_old_logs() -> Result<(), String> {
    let log_dir = crate::models::debug::get_log_directory();
    let log_path = Path::new(&log_dir);
    
    if !log_path.exists() {
        return Ok(());
    }
    
    let mut log_files: Vec<(String, i64)> = Vec::new();
    
    // 收集所有日志文件
    for entry in fs::read_dir(log_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if let Some(ext) = path.extension() {
            if ext == "log" {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let modified_time = modified
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(0);
                        
                        log_files.push((path.to_string_lossy().to_string(), modified_time));
                    }
                }
            }
        }
    }
    
    // 按修改时间排序
    log_files.sort_by(|a, b| b.1.cmp(&a.1));
    
    // 删除超过7天的日志
    let seven_days_ago = chrono::Utc::now().timestamp() - 7 * 24 * 60 * 60;
    
    for (path, modified) in log_files.iter().skip(7) {
        if *modified < seven_days_ago {
            let _ = fs::remove_file(path);
        }
    }
    
    Ok(())
}

/// 记录应用启动
pub fn log_app_start(version: &str) {
    let logger = LOGGER.read();
    if !logger.config.enabled {
        return;
    }
    
    let content = format!(
        r#"
  Version: {}
  Platform: {}
  Time: {}"#,
        version,
        std::env::consts::OS,
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    
    logger.write_log("APP_START", &content);
}

/// 记录应用关闭
pub fn log_app_stop() {
    let logger = LOGGER.read();
    if !logger.config.enabled {
        return;
    }
    
    let content = format!(
        "应用关闭 - Time: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    
    logger.write_log("APP_STOP", &content);
}
