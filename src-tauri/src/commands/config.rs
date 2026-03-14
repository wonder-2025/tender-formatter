use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

/// 配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_provider: String,
    pub api_key: String,
    pub api_model: String,
    pub output_mode: String,
    pub auto_backup: bool,
    pub desensitize: bool,
    pub audit_log: bool,
    pub theme: String,
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_provider: "baidu".to_string(),
            api_key: String::new(),
            api_model: "ERNIE-3.5-8K".to_string(),
            output_mode: "new".to_string(),
            auto_backup: true,
            desensitize: true,
            audit_log: true,
            theme: "light".to_string(),
            language: "zh-CN".to_string(),
        }
    }
}

/// 加载配置
#[tauri::command]
pub fn load_config(state: State<'_, AppState>) -> Config {
    let config = state.config.lock();
    Config {
        api_provider: config.api_provider.clone(),
        api_key: config.api_key.clone(),
        api_model: config.api_model.clone(),
        output_mode: config.output_mode.clone(),
        auto_backup: config.auto_backup,
        desensitize: true,
        audit_log: true,
        theme: "light".to_string(),
        language: "zh-CN".to_string(),
    }
}

/// 保存配置
#[tauri::command]
pub fn save_config(
    config: Config,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut app_config = state.config.lock();
    app_config.api_provider = config.api_provider;
    app_config.api_key = config.api_key;
    app_config.api_model = config.api_model;
    app_config.output_mode = config.output_mode;
    app_config.auto_backup = config.auto_backup;
    
    // TODO: 保存到文件
    
    Ok(())
}

/// 测试 API 连接
#[tauri::command]
pub async fn test_api_connection(
    provider: String,
    api_key: String,
    model: String,
) -> Result<String, String> {
    // TODO: 实现实际的 API 测试
    
    let _ = (provider, api_key, model);
    
    // 模拟测试
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    Ok("连接成功".to_string())
}
