// LLM 客户端
// 支持多种 LLM 提供商的统一调用接口
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::services::logger;

/// LLM 配置
#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub model: String,
    pub base_url: Option<String>,
    // 备用模型配置
    pub enable_backup: Option<bool>,
    pub backup_provider: Option<String>,
    pub backup_api_key: Option<String>,
    pub backup_model: Option<String>,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "baidu".to_string(),
            api_key: String::new(),
            model: "ERNIE-3.5-8K".to_string(),
            base_url: None,
            enable_backup: Some(false),
            backup_provider: None,
            backup_api_key: None,
            backup_model: None,
        }
    }
}

/// 聊天请求
#[derive(Debug, Serialize)]
struct ChatRequest {
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
}

/// 消息
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

/// 聊天响应
#[derive(Debug, Deserialize)]
struct ChatResponse {
    result: Option<String>,
    choices: Option<Vec<Choice>>,
    #[serde(default)]
    error_code: i32,
    #[serde(default)]
    error_msg: String,
}

/// 选择项
#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

/// LLM 错误
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("网络错误: {0}")]
    NetworkError(String),
    
    #[error("API 错误: {0}")]
    ApiError(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
}

/// 发送聊天请求
pub async fn chat(config: &LlmConfig, prompt: String) -> Result<String, LlmError> {
    if config.api_key.is_empty() {
        return Err(LlmError::ConfigError("API Key 未配置".to_string()));
    }
    
    let client = Client::builder()
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    let start_time = std::time::Instant::now();
    
    // 记录请求
    logger::log_api_request(
        &config.provider,
        &get_endpoint(config).unwrap_or_default(),
        "Content-Type: application/json",
        &prompt,
        Some(&config.api_key),
    );
    
    let result = match config.provider.as_str() {
        "baidu" => chat_baidu(&client, config, prompt).await,
        "aliyun" => chat_aliyun(&client, config, prompt).await,
        "openai" => chat_openai(&client, config, prompt).await,
        "deepseek" => chat_deepseek(&client, config, prompt).await,
        _ => Err(LlmError::ConfigError(format!("不支持的提供商: {}", config.provider))),
    };
    
    // 记录响应
    match &result {
        Ok(response) => {
            logger::log_api_response(
                &config.provider,
                200,
                response,
                start_time.elapsed().as_millis() as u64
            );
        }
        Err(e) => {
            logger::log_api_response(
                &config.provider,
                500,
                &e.to_string(),
                start_time.elapsed().as_millis() as u64
            );
        }
    }
    
    result
}

/// 获取 API 端点
fn get_endpoint(config: &LlmConfig) -> Option<String> {
    match config.provider.as_str() {
        "baidu" => Some(format!(
            "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/{}?access_token={}",
            get_baidu_model_path(&config.model),
            config.api_key
        )),
        "aliyun" => Some("https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation".to_string()),
        "openai" => Some("https://api.openai.com/v1/chat/completions".to_string()),
        "deepseek" => Some("https://api.deepseek.com/v1/chat/completions".to_string()),
        _ => config.base_url.clone(),
    }
}

/// 百度模型路径映射
fn get_baidu_model_path(model: &str) -> &str {
    match model {
        "ERNIE-4.0-8K" => "completions_pro",
        "ERNIE-3.5-8K" => "completions",
        "ERNIE-3.5-128K" => "ernie-3.5-128k",
        _ => "completions",
    }
}

/// 百度千帆 API 调用
async fn chat_baidu(
    client: &Client,
    config: &LlmConfig,
    prompt: String,
) -> Result<String, LlmError> {
    let url = get_endpoint(config).ok_or_else(|| {
        LlmError::ConfigError("无法构建 API 端点".to_string())
    })?;
    
    let request = ChatRequest {
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: None,
    };
    
    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    let status = response.status();
    let body = response.text().await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    if !status.is_success() {
        return Err(LlmError::ApiError(format!("API 返回错误: {} - {}", status, body)));
    }
    
    let chat_response: ChatResponse = serde_json::from_str(&body)
        .map_err(|e| LlmError::ParseError(format!("解析响应失败: {} - {}", e, body)))?;
    
    if chat_response.error_code != 0 {
        return Err(LlmError::ApiError(format!(
            "API 错误: {} - {}",
            chat_response.error_code, chat_response.error_msg
        )));
    }
    
    chat_response.result.ok_or_else(|| {
        LlmError::ParseError("响应中没有结果".to_string())
    })
}

/// 阿里通义千问 API 调用
async fn chat_aliyun(
    client: &Client,
    config: &LlmConfig,
    prompt: String,
) -> Result<String, LlmError> {
    let url = "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation";
    
    #[derive(Serialize)]
    struct AliyunRequest {
        model: String,
        input: AliyunInput,
        parameters: AliyunParams,
    }
    
    #[derive(Serialize)]
    struct AliyunInput {
        messages: Vec<Message>,
    }
    
    #[derive(Serialize)]
    struct AliyunParams {
        temperature: f32,
    }
    
    let request = AliyunRequest {
        model: config.model.clone(),
        input: AliyunInput {
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        },
        parameters: AliyunParams {
            temperature: 0.7,
        },
    };
    
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    let status = response.status();
    let body = response.text().await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    if !status.is_success() {
        return Err(LlmError::ApiError(format!("API 返回错误: {} - {}", status, body)));
    }
    
    #[derive(Deserialize)]
    struct AliyunResponse {
        output: Option<AliyunOutput>,
        code: Option<String>,
        message: Option<String>,
    }
    
    #[derive(Deserialize)]
    struct AliyunOutput {
        text: Option<String>,
    }
    
    let ali_response: AliyunResponse = serde_json::from_str(&body)
        .map_err(|e| LlmError::ParseError(format!("解析响应失败: {}", e)))?;
    
    if let Some(code) = ali_response.code {
        return Err(LlmError::ApiError(format!(
            "API 错误: {} - {}",
            code,
            ali_response.message.unwrap_or_default()
        )));
    }
    
    ali_response.output
        .and_then(|o| o.text)
        .ok_or_else(|| LlmError::ParseError("响应中没有结果".to_string()))
}

/// OpenAI API 调用
async fn chat_openai(
    client: &Client,
    config: &LlmConfig,
    prompt: String,
) -> Result<String, LlmError> {
    let url = config.base_url.as_deref()
        .unwrap_or("https://api.openai.com/v1/chat/completions");
    
    let request = ChatRequest {
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
    };
    
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    let status = response.status();
    let body = response.text().await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    if !status.is_success() {
        return Err(LlmError::ApiError(format!("API 返回错误: {} - {}", status, body)));
    }
    
    let chat_response: ChatResponse = serde_json::from_str(&body)
        .map_err(|e| LlmError::ParseError(format!("解析响应失败: {}", e)))?;
    
    chat_response.choices
        .and_then(|c| c.into_iter().next())
        .map(|c| c.message.content)
        .ok_or_else(|| LlmError::ParseError("响应中没有结果".to_string()))
}

/// DeepSeek API 调用
async fn chat_deepseek(
    client: &Client,
    config: &LlmConfig,
    prompt: String,
) -> Result<String, LlmError> {
    let url = config.base_url.as_deref()
        .unwrap_or("https://api.deepseek.com/v1/chat/completions");
    
    let request = ChatRequest {
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
        temperature: Some(0.7),
        max_tokens: Some(4096),
    };
    
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    let status = response.status();
    let body = response.text().await
        .map_err(|e| LlmError::NetworkError(e.to_string()))?;
    
    if !status.is_success() {
        return Err(LlmError::ApiError(format!("API 返回错误: {} - {}", status, body)));
    }
    
    let chat_response: ChatResponse = serde_json::from_str(&body)
        .map_err(|e| LlmError::ParseError(format!("解析响应失败: {}", e)))?;
    
    chat_response.choices
        .and_then(|c| c.into_iter().next())
        .map(|c| c.message.content)
        .ok_or_else(|| LlmError::ParseError("响应中没有结果".to_string()))
}

/// 测试 API 连接
pub async fn test_connection(config: &LlmConfig) -> Result<String, LlmError> {
    let result = chat(config, "你好，请回复'连接成功'".to_string()).await?;
    Ok(result)
}

/// 带备用模型的聊天请求（主模型失败自动切换）
pub async fn chat_with_fallback(config: &LlmConfig, prompt: String) -> Result<String, LlmError> {
    // 先尝试主模型
    match chat(config, prompt.clone()).await {
        Ok(result) => {
            logger::log_info(&format!("主模型请求成功: provider={}, model={}", config.provider, config.model));
            Ok(result)
        }
        Err(e) => {
            // 检查是否启用备用模型
            if config.enable_backup.unwrap_or(false) {
                if let (Some(backup_provider), Some(backup_api_key), Some(backup_model)) = 
                    (&config.backup_provider, &config.backup_api_key, &config.backup_model) {
                    
                    logger::log_warn(&format!("主模型请求失败，切换备用模型: {} -> {}", config.provider, backup_provider));
                    
                    // 构建备用配置
                    let backup_config = LlmConfig {
                        provider: backup_provider.clone(),
                        api_key: backup_api_key.clone(),
                        model: backup_model.clone(),
                        base_url: None,
                        enable_backup: Some(false),
                        backup_provider: None,
                        backup_api_key: None,
                        backup_model: None,
                    };
                    
                    // 使用备用模型重试
                    match chat(&backup_config, prompt).await {
                        Ok(result) => {
                            logger::log_info(&format!("备用模型请求成功: provider={}, model={}", backup_provider, backup_model));
                            Ok(result)
                        }
                        Err(backup_err) => {
                            logger::log_error(&format!("备用模型也失败: {}", backup_err));
                            Err(LlmError::ApiError(format!(
                                "主模型和备用模型都失败: 主模型={}, 备用模型={}", 
                                e, backup_err
                            )))
                        }
                    }
                } else {
                    Err(LlmError::ConfigError(format!("主模型失败且备用模型配置不完整: {}", e)))
                }
            } else {
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_llm_config_default() {
        let config = LlmConfig::default();
        assert_eq!(config.provider, "baidu");
    }
    
    #[test]
    fn test_get_baidu_model_path() {
        assert_eq!(get_baidu_model_path("ERNIE-4.0-8K"), "completions_pro");
        assert_eq!(get_baidu_model_path("ERNIE-3.5-8K"), "completions");
    }
}
