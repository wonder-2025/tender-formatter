// 格式相关命令
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;
use crate::models::format::FormatRequirement;
use crate::services::format_extractor::{self, DocumentFormatInfo};
use crate::services::docx_editor::{self, DocumentInfo};
use crate::services::llm_client::LlmConfig;
use crate::services::logger;

/// 从招标文件提取格式要求
#[tauri::command]
pub async fn extract_format_from_tender(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<FormatRequirement, String> {
    // 记录文件操作
    logger::log_file_operation("提取格式", &file_path, true, None);
    
    // 获取 API 配置（在单独的 block 中释放 MutexGuard）
    let llm_config = {
        let config = state.config.lock();
        LlmConfig {
            provider: config.api_provider.clone(),
            api_key: config.api_key.clone(),
            model: config.api_model.clone(),
            base_url: get_base_url(&config.api_provider),
            enable_backup: Some(config.enable_backup),
            backup_provider: Some(config.backup_provider.clone()).filter(|s| !s.is_empty()),
            backup_api_key: Some(config.backup_api_key.clone()).filter(|s| !s.is_empty()),
            backup_model: Some(config.backup_model.clone()).filter(|s| !s.is_empty()),
        }
    };
    
    // 记录 API 请求开始
    let start_time = std::time::Instant::now();
    
    // 调用格式提取服务
    let result = format_extractor::extract_format(&file_path, &llm_config).await;
    
    match result {
        Ok(format) => {
            // 记录 API 响应
            logger::log_api_response(
                &llm_config.provider,
                200,
                "格式提取成功",
                start_time.elapsed().as_millis() as u64
            );
            
            Ok(format)
        }
        Err(e) => {
            // 记录错误
            logger::log_file_operation("提取格式", &file_path, false, Some(&e.to_string()));
            Err(e.to_string())
        }
    }
}

/// 分析文档当前格式
#[tauri::command]
pub async fn analyze_document_format(
    file_path: String,
) -> Result<DocumentFormat, String> {
    // 记录文件操作
    logger::log_file_operation("分析格式", &file_path, true, None);
    
    let result = format_extractor::analyze_document_format(&file_path);
    
    match result {
        Ok(info) => {
            Ok(DocumentFormat {
                paper_size: info.paper_size,
                margin_top: info.margin_top,
                margin_bottom: info.margin_bottom,
                margin_left: info.margin_left,
                margin_right: info.margin_right,
                body_font: info.body_font,
                body_font_size: info.body_font_size,
                line_height: info.line_height,
                orientation: info.orientation,
            })
        }
        Err(e) => {
            logger::log_file_operation("分析格式", &file_path, false, Some(&e.to_string()));
            Err(e.to_string())
        }
    }
}

/// 比较格式差异
#[tauri::command]
pub fn compare_format_diff(
    current: DocumentFormat,
    target: FormatRequirement,
) -> Vec<FormatDiff> {
    let mut diffs = Vec::new();
    
    // 比较纸张大小
    if current.paper_size != target.paper_size {
        diffs.push(FormatDiff {
            category: "页面设置".to_string(),
            name: "纸张大小".to_string(),
            current: current.paper_size.clone(),
            target: target.paper_size.clone(),
        });
    }
    
    // 比较页边距
    if (current.margin_top - target.margin_top).abs() > 0.01 {
        diffs.push(FormatDiff {
            category: "页面设置".to_string(),
            name: "页边距-上".to_string(),
            current: format!("{:.2}cm", current.margin_top),
            target: format!("{:.2}cm", target.margin_top),
        });
    }
    
    if (current.margin_bottom - target.margin_bottom).abs() > 0.01 {
        diffs.push(FormatDiff {
            category: "页面设置".to_string(),
            name: "页边距-下".to_string(),
            current: format!("{:.2}cm", current.margin_bottom),
            target: format!("{:.2}cm", target.margin_bottom),
        });
    }
    
    if (current.margin_left - target.margin_left).abs() > 0.01 {
        diffs.push(FormatDiff {
            category: "页面设置".to_string(),
            name: "页边距-左".to_string(),
            current: format!("{:.2}cm", current.margin_left),
            target: format!("{:.2}cm", target.margin_left),
        });
    }
    
    if (current.margin_right - target.margin_right).abs() > 0.01 {
        diffs.push(FormatDiff {
            category: "页面设置".to_string(),
            name: "页边距-右".to_string(),
            current: format!("{:.2}cm", current.margin_right),
            target: format!("{:.2}cm", target.margin_right),
        });
    }
    
    // 比较字体
    if current.body_font != target.body_font {
        diffs.push(FormatDiff {
            category: "字体格式".to_string(),
            name: "正文字体".to_string(),
            current: current.body_font.clone(),
            target: target.body_font.clone(),
        });
    }
    
    if current.body_font_size != target.body_font_size {
        diffs.push(FormatDiff {
            category: "字体格式".to_string(),
            name: "正文字号".to_string(),
            current: current.body_font_size.clone(),
            target: target.body_font_size.clone(),
        });
    }
    
    if current.line_height != target.body_line_height {
        diffs.push(FormatDiff {
            category: "字体格式".to_string(),
            name: "行间距".to_string(),
            current: current.line_height.clone(),
            target: target.body_line_height.clone(),
        });
    }
    
    // 记录格式差异
    for diff in &diffs {
        logger::log_format_change(&diff.name, &diff.current, &diff.target);
    }
    
    diffs
}

/// 应用格式到文档
#[tauri::command]
pub async fn apply_format(
    file_path: String,
    format: FormatRequirement,
    output_mode: String,
    backup: bool,
) -> Result<String, String> {
    // 记录开始
    logger::log_file_operation("应用格式", &file_path, true, None);
    
    // 备份原文档（如果需要）
    if backup {
        match docx_editor::backup_document(&file_path) {
            Ok(backup_path) => {
                logger::log_file_operation("备份文档", &backup_path, true, None);
            }
            Err(e) => {
                logger::log_file_operation("备份文档", &file_path, false, Some(&e.to_string()));
            }
        }
    }
    
    // 确定输出路径
    let output_path = match output_mode.as_str() {
        "modify" => file_path.clone(),
        "new" => {
            let path = std::path::Path::new(&file_path);
            let dir = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
            let name = path.file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("document");
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("docx");
            format!("{}/{}_优化.{}", dir, name, ext)
        }
        "copy" => {
            let path = std::path::Path::new(&file_path);
            let dir = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
            let name = path.file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("document");
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("docx");
            format!("{}/{}_副本.{}", dir, name, ext)
        }
        _ => file_path.clone(),
    };
    
    // 应用格式
    let result = docx_editor::apply_format(&file_path, &output_path, &format);
    
    match result {
        Ok(_) => {
            logger::log_file_operation("应用格式", &output_path, true, None);
            Ok(output_path)
        }
        Err(e) => {
            logger::log_file_operation("应用格式", &file_path, false, Some(&e.to_string()));
            Err(e.to_string())
        }
    }
}

/// 文档格式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFormat {
    pub paper_size: String,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub margin_right: f64,
    pub body_font: String,
    pub body_font_size: String,
    pub line_height: String,
    pub orientation: String,
}

/// 格式差异
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatDiff {
    pub category: String,
    pub name: String,
    pub current: String,
    pub target: String,
}

/// 获取 API Base URL
fn get_base_url(provider: &str) -> Option<String> {
    match provider {
        "baidu" => Some("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat".to_string()),
        "aliyun" => Some("https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation".to_string()),
        "openai" => Some("https://api.openai.com/v1".to_string()),
        "deepseek" => Some("https://api.deepseek.com/v1".to_string()),
        _ => None,
    }
}
