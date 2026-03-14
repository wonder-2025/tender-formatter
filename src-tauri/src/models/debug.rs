// Debug 配置模型
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use serde::{Deserialize, Serialize};

/// Debug 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// 是否启用 Debug 模式
    pub enabled: bool,
    
    /// 日志级别: info, debug, trace
    pub log_level: String,
    
    /// 记录内容项
    pub log_items: Vec<String>,
    
    /// 日志文件路径
    pub log_file_path: String,
}

impl Default for DebugConfig {
    fn default() -> Self {
        // 获取日志目录
        let log_dir = get_log_directory();
        let log_path = format!("{}/app.log", log_dir);
        
        Self {
            enabled: false,
            log_level: "info".to_string(),
            log_items: vec![
                "apiRequest".to_string(),
                "apiResponse".to_string(),
                "fileOperation".to_string(),
            ],
            log_file_path: log_path,
        }
    }
}

impl DebugConfig {
    /// 检查是否记录指定项
    pub fn should_log(&self, item: &str) -> bool {
        self.enabled && self.log_items.contains(&item.to_string())
    }
    
    /// 获取日志级别
    pub fn get_log_level(&self) -> tracing::Level {
        match self.log_level.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            _ => tracing::Level::INFO,
        }
    }
}

/// 获取日志目录
pub fn get_log_directory() -> String {
    // 尝试获取用户数据目录
    if let Some(data_dir) = dirs::data_local_dir() {
        let log_dir = data_dir.join("tender-formatter").join("logs");
        
        // 确保目录存在
        if let Err(e) = std::fs::create_dir_all(&log_dir) {
            eprintln!("无法创建日志目录: {}", e);
        }
        
        return log_dir.to_string_lossy().to_string();
    }
    
    // 回退到当前目录
    "./logs".to_string()
}

/// 获取日志文件路径
pub fn get_log_file_path() -> String {
    let log_dir = get_log_directory();
    let date = chrono::Local::now().format("%Y-%m-%d");
    format!("{}/app-{}.log", log_dir, date)
}

/// 敏感信息脱敏显示
pub fn mask_sensitive(text: &str, keep_prefix: usize, keep_suffix: usize) -> String {
    if text.len() <= keep_prefix + keep_suffix {
        return "*".repeat(text.len());
    }
    
    let prefix = &text[..keep_prefix];
    let suffix = &text[text.len() - keep_suffix..];
    let masked_len = text.len() - keep_prefix - keep_suffix;
    
    format!("{}{}{}", prefix, "*".repeat(masked_len), suffix)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mask_sensitive() {
        assert_eq!(mask_sensitive("1234567890abcdef", 4, 4), "1234********cdef");
        assert_eq!(mask_sensitive("short", 2, 2), "sh*rt");
        assert_eq!(mask_sensitive("ab", 1, 1), "**");
    }
    
    #[test]
    fn test_should_log() {
        let config = DebugConfig {
            enabled: true,
            log_items: vec!["apiRequest".to_string()],
            ..Default::default()
        };
        
        assert!(config.should_log("apiRequest"));
        assert!(!config.should_log("desensitize"));
    }
}
