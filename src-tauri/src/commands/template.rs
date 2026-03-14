use serde::{Deserialize, Serialize};
use crate::models::format::FormatRequirement;

/// 模板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub format: FormatRequirement,
    pub is_preset: bool,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

/// 获取模板列表
#[tauri::command]
pub fn get_templates() -> Vec<Template> {
    // 预设模板
    vec![
        Template {
            id: "gov-procurement".to_string(),
            name: "政府采购通用模板".to_string(),
            description: "字体: 宋体小四 | 页边距: 2.54/3.17cm | 行距: 1.5倍".to_string(),
            format: FormatRequirement::default(),
            is_preset: true,
            created_at: "2026-01-01".to_string(),
            last_used_at: None,
        },
        Template {
            id: "enterprise-standard".to_string(),
            name: "央企招标标准模板".to_string(),
            description: "字体: 宋体四号 | 页边距: 2.5cm | 行距: 1.25倍".to_string(),
            format: FormatRequirement {
                margin_top: 2.5,
                margin_bottom: 2.5,
                margin_left: 2.5,
                margin_right: 2.5,
                body_font_size: "四号".to_string(),
                body_line_height: "1.25倍".to_string(),
                ..FormatRequirement::default()
            },
            is_preset: true,
            created_at: "2026-01-01".to_string(),
            last_used_at: None,
        },
        Template {
            id: "state-owned".to_string(),
            name: "国企投标格式模板".to_string(),
            description: "字体: 仿宋四号 | 页边距: 2.8cm | 行距: 1.5倍".to_string(),
            format: FormatRequirement {
                margin_top: 2.8,
                margin_bottom: 2.8,
                margin_left: 2.8,
                margin_right: 2.8,
                body_font: "仿宋".to_string(),
                body_font_size: "四号".to_string(),
                ..FormatRequirement::default()
            },
            is_preset: true,
            created_at: "2026-01-01".to_string(),
            last_used_at: None,
        },
    ]
}

/// 保存模板
#[tauri::command]
pub fn save_template(
    name: String,
    description: String,
    format: FormatRequirement,
) -> Result<Template, String> {
    let template = Template {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        description,
        format,
        is_preset: false,
        created_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
        last_used_at: None,
    };
    
    // TODO: 保存到数据库或文件
    
    Ok(template)
}

/// 删除模板
#[tauri::command]
pub fn delete_template(id: String) -> Result<(), String> {
    // TODO: 从数据库或文件删除
    
    let _ = id;
    Ok(())
}
