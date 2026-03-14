// 审计日志
// 复用检查工具的审计逻辑

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

pub struct AuditLogger {
    log_path: PathBuf,
}

impl AuditLogger {
    pub fn new(app_data_dir: &PathBuf) -> Self {
        let log_path = app_data_dir.join("audit.log");
        
        // 确保目录存在
        if let Some(parent) = log_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        
        Self { log_path }
    }
    
    pub fn log(&self, action: &str, detail: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let entry = format!("[{}] {} - {}\n", timestamp, action, detail);
        
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
        {
            file.write_all(entry.as_bytes()).ok();
        }
    }
}
