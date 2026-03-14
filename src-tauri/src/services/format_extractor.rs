// 格式提取器
// 从招标文件提取格式要求
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use crate::models::format::FormatRequirement;
use crate::security::desensitize::{desensitize, get_default_rules};
use crate::services::llm_client::{chat_with_fallback, LlmConfig};
use std::path::Path;
use std::fs;
use zip::ZipArchive;
use std::io::Cursor;

/// 从招标文件提取格式要求
pub async fn extract_format(
    file_path: &str,
    config: &LlmConfig,
) -> Result<FormatRequirement, FormatExtractError> {
    // 1. 读取文件内容
    let content = read_file_content(file_path)?;
    
    // 2. 脱敏处理
    let rules = get_default_rules();
    let desensitized_content = desensitize(&content, &rules);
    
    // 3. 构建 LLM 提示词
    let prompt = build_extraction_prompt(&desensitized_content);
    
    // 4. 调用 LLM API
    let response = chat_with_fallback(config, prompt)
        .await
        .map_err(|e| FormatExtractError::LlmError(e.to_string()))?;
    
    // 5. 解析返回结果
    parse_format_response(&response)
}

/// 读取文件内容
fn read_file_content(file_path: &str) -> Result<String, FormatExtractError> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(FormatExtractError::FileNotFound(file_path.to_string()));
    }
    
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match extension.as_str() {
        "docx" => read_docx_content(file_path),
        "doc" => Err(FormatExtractError::UnsupportedFormat("旧版 .doc 格式暂不支持，请转换为 .docx".to_string())),
        "pdf" => read_pdf_content(file_path),
        "txt" => read_text_content(file_path),
        _ => Err(FormatExtractError::UnsupportedFormat(format!("不支持的文件格式: {}", extension))),
    }
}

/// 读取 DOCX 文件内容
fn read_docx_content(file_path: &str) -> Result<String, FormatExtractError> {
    let file = fs::File::open(file_path)
        .map_err(|e| FormatExtractError::ReadError(format!("无法打开文件: {}", e)))?;
    
    let reader = std::io::BufReader::new(file);
    let mut archive = ZipArchive::new(reader)
        .map_err(|e| FormatExtractError::ReadError(format!("无效的 DOCX 文件: {}", e)))?;
    
    // 读取 document.xml
    let document_xml = archive.by_name("word/document.xml")
        .map_err(|e| FormatExtractError::ReadError(format!("无法读取文档内容: {}", e)))?;
    
    let mut content = String::new();
    let mut reader = std::io::BufReader::new(document_xml);
    std::io::Read::read_to_string(&mut reader, &mut content)
        .map_err(|e| FormatExtractError::ReadError(format!("读取内容失败: {}", e)))?;
    
    // 解析 XML 提取纯文本
    extract_text_from_xml(&content)
}

/// 从 XML 中提取纯文本
fn extract_text_from_xml(xml: &str) -> Result<String, FormatExtractError> {
    use quick_xml::Reader;
    use quick_xml::events::Event;
    
    let mut reader = Reader::from_str(xml);
    let mut text = String::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                if let Ok(t) = e.unescape() {
                    text.push_str(&t);
                }
            }
            Ok(Event::End(_)) => {
                text.push(' ');
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    
    // 清理多余空格
    let cleaned: String = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    
    Ok(cleaned)
}

/// 读取 PDF 文件内容
fn read_pdf_content(file_path: &str) -> Result<String, FormatExtractError> {
    let content = fs::read(file_path)
        .map_err(|e| FormatExtractError::ReadError(format!("无法读取 PDF 文件: {}", e)))?;
    
    let pdf = lopdf::Document::load_mem(&content)
        .map_err(|e| FormatExtractError::ReadError(format!("无效的 PDF 文件: {}", e)))?;
    
    let mut text = String::new();
    
    // 遍历所有页面
    for (_page_num, page) in pdf.get_pages() {
        if let Ok(page_content) = pdf.get_page_content(page) {
            // 简单提取文本（实际生产中需要更复杂的解析）
            if let Ok(page_text) = extract_text_from_pdf_page(&page_content) {
                text.push_str(&page_text);
                text.push('\n');
            }
        }
    }
    
    if text.is_empty() {
        // 如果无法提取文本，返回提示信息
        return Ok("[PDF 内容] 包含格式要求，建议转换为 Word 格式以获得更准确的提取结果".to_string());
    }
    
    Ok(text)
}

/// 从 PDF 页面提取文本
fn extract_text_from_pdf_page(content: &[u8]) -> Result<String, FormatExtractError> {
    // 简化的 PDF 文本提取
    // 实际生产中应使用更完善的 PDF 解析库
    let mut text = String::new();
    let mut in_text = false;
    
    for byte in content {
        match *byte {
            b'B' if !in_text => in_text = true,
            b'E' if in_text => in_text = false,
            b' ' | b'\n' | b'\r' => text.push(' '),
            _ if in_text && *byte >= 32 && *byte < 127 => {
                text.push(*byte as char);
            }
            _ if in_text && *byte >= 0xE0 => {
                // 尝试处理 UTF-8 中文字符
                text.push(*byte as char);
            }
            _ => {}
        }
    }
    
    Ok(text)
}

/// 读取纯文本文件
fn read_text_content(file_path: &str) -> Result<String, FormatExtractError> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| FormatExtractError::ReadError(format!("无法读取文件: {}", e)))?;
    Ok(content)
}

/// 构建格式提取提示词
fn build_extraction_prompt(content: &str) -> String {
    // 限制内容长度，避免超出 token 限制
    let max_chars = 8000;
    let truncated = if content.len() > max_chars {
        &content[..max_chars]
    } else {
        content
    };
    
    format!(
        r#"你是一个专业的标书格式分析助手。请从以下招标文件内容中提取格式要求。

请仔细分析文档中关于投标文件格式的具体要求，包括：
1. 页面设置：纸张大小、页边距（上下左右）、装订线、页面方向
2. 字体格式：正文字体、字号、行间距
3. 标题格式：各级标题（一级到四级）的字体、字号、段前段后间距
4. 页眉页脚：页眉内容、字体、页码位置和格式
5. 表格格式：表格字体、字号、边框样式

请以 JSON 格式返回结果，格式如下：
{{
  "paper_size": "A4",
  "margin_top": 2.54,
  "margin_bottom": 2.54,
  "margin_left": 3.17,
  "margin_right": 3.17,
  "gutter": 0.0,
  "gutter_position": "left",
  "orientation": "portrait",
  "body_font": "宋体",
  "body_font_size": "小四",
  "body_line_height": "1.5倍",
  "body_paragraph_spacing": {{ "before": 0.0, "after": 0.0 }},
  "heading1": {{ "font": "黑体", "font_size": "三号", "spacing": {{ "before": 0.5, "after": 0.5 }} }},
  "heading2": {{ "font": "黑体", "font_size": "四号", "spacing": {{ "before": 0.5, "after": 0.5 }} }},
  "heading3": {{ "font": "黑体", "font_size": "小四", "spacing": {{ "before": 0.0, "after": 0.0 }} }},
  "heading4": {{ "font": "黑体", "font_size": "小四", "spacing": {{ "before": 0.0, "after": 0.0 }} }},
  "header": {{ "content": "项目名称", "font": "宋体", "font_size": "小五" }},
  "footer": {{ "position": "居中", "format": "第X页 共X页" }},
  "table_font": "宋体",
  "table_font_size": "小四",
  "table_border": "单线",
  "table_align": "居中"
}}

注意事项：
- 页边距单位为厘米
- 如果文档中没有明确说明某项格式要求，请填写常见默认值
- 字号使用中文名称（如：小四、四号、三号等）
- 只返回 JSON，不要添加其他说明文字

招标文件内容：
{}"#,
        truncated
    )
}

/// 解析 LLM 返回的格式响应
fn parse_format_response(response: &str) -> Result<FormatRequirement, FormatExtractError> {
    // 尝试从响应中提取 JSON
    let json_str = extract_json(response)?;
    
    // 解析 JSON
    let format: FormatRequirement = serde_json::from_str(&json_str)
        .map_err(|e| FormatExtractError::ParseError(format!("解析 JSON 失败: {}", e)))?;
    
    Ok(format)
}

/// 从响应中提取 JSON
fn extract_json(response: &str) -> Result<String, FormatExtractError> {
    // 尝试找到 JSON 块
    let trimmed = response.trim();
    
    // 如果响应被 ```json 包裹
    if let Some(start) = trimmed.find("```json") {
        let rest = &trimmed[start + 7..];
        if let Some(end) = rest.find("```") {
            return Ok(rest[..end].trim().to_string());
        }
    }
    
    // 如果响应被 ``` 包裹
    if let Some(start) = trimmed.find("```") {
        let rest = &trimmed[start + 3..];
        if let Some(end) = rest.find("```") {
            return Ok(rest[..end].trim().to_string());
        }
    }
    
    // 尝试直接找到 JSON 对象
    if let Some(start) = trimmed.find('{') {
        let mut depth = 0;
        let mut end = start;
        for (i, c) in trimmed[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = start + i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        if end > start {
            return Ok(trimmed[start..end].to_string());
        }
    }
    
    Err(FormatExtractError::ParseError("无法从响应中提取 JSON".to_string()))
}

/// 分析文档当前格式
pub fn analyze_document_format(file_path: &str) -> Result<DocumentFormatInfo, FormatExtractError> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(FormatExtractError::FileNotFound(file_path.to_string()));
    }
    
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match extension.as_str() {
        "docx" => analyze_docx_format(file_path),
        _ => Err(FormatExtractError::UnsupportedFormat(format!("不支持分析 {} 格式文件", extension))),
    }
}

/// 分析 DOCX 文档格式
fn analyze_docx_format(file_path: &str) -> Result<DocumentFormatInfo, FormatExtractError> {
    let file = fs::File::open(file_path)
        .map_err(|e| FormatExtractError::ReadError(format!("无法打开文件: {}", e)))?;
    
    let reader = std::io::BufReader::new(file);
    let mut archive = ZipArchive::new(reader)
        .map_err(|e| FormatExtractError::ReadError(format!("无效的 DOCX 文件: {}", e)))?;
    
    // 读取 styles.xml 获取样式信息
    let mut styles_content = String::new();
    if let Ok(mut styles_file) = archive.by_name("word/styles.xml") {
        std::io::Read::read_to_string(&mut styles_file, &mut styles_content)
            .map_err(|e| FormatExtractError::ReadError(format!("读取样式失败: {}", e)))?;
    }
    
    // 读取 document.xml 获取页面设置
    let mut document_content = String::new();
    if let Ok(mut document_file) = archive.by_name("word/document.xml") {
        std::io::Read::read_to_string(&mut document_file, &mut document_content)
            .map_err(|e| FormatExtractError::ReadError(format!("读取文档失败: {}", e)))?;
    }
    
    // 解析页面设置
    let page_settings = parse_page_settings(&document_content)?;
    
    // 解析字体设置
    let font_settings = parse_font_settings(&styles_content, &document_content)?;
    
    Ok(DocumentFormatInfo {
        paper_size: page_settings.paper_size,
        margin_top: page_settings.margin_top,
        margin_bottom: page_settings.margin_bottom,
        margin_left: page_settings.margin_left,
        margin_right: page_settings.margin_right,
        body_font: font_settings.body_font,
        body_font_size: font_settings.body_font_size,
        line_height: font_settings.line_height,
        orientation: page_settings.orientation,
    })
}

/// 页面设置信息
#[derive(Debug, Clone)]
struct PageSettings {
    paper_size: String,
    margin_top: f64,
    margin_bottom: f64,
    margin_left: f64,
    margin_right: f64,
    orientation: String,
}

/// 字体设置信息
#[derive(Debug, Clone)]
struct FontSettings {
    body_font: String,
    body_font_size: String,
    line_height: String,
}

/// 解析页面设置
fn parse_page_settings(xml: &str) -> Result<PageSettings, FormatExtractError> {
    // 默认值
    let mut settings = PageSettings {
        paper_size: "A4".to_string(),
        margin_top: 2.54,
        margin_bottom: 2.54,
        margin_left: 3.17,
        margin_right: 3.17,
        orientation: "portrait".to_string(),
    };
    
    // 使用正则表达式解析 sectPr
    let sect_pr_regex = regex::Regex::new(r"<w:sectPr[^>]*>(.*?)</w:sectPr>").unwrap();
    let pg_sz_regex = regex::Regex::new(r#"<w:pgSz[^>]*w:w="(\d+)"[^>]*w:h="(\d+)"[^>]*/>"#).unwrap();
    let pg_mar_regex = regex::Regex::new(r#"<w:pgMar[^>]*w:top="(\d+)"[^>]*w:bottom="(\d+)"[^>]*w:left="(\d+)"[^>]*w:right="(\d+)"[^>]*/>"#).unwrap();
    
    if let Some(sect_caps) = sect_pr_regex.captures(xml) {
        let sect_content = sect_caps.get(1).map_or("", |m| m.as_str());
        
        // 解析页面尺寸
        if let Some(sz_caps) = pg_sz_regex.captures(sect_content) {
            let width: i32 = sz_caps.get(1).map_or(11906, |m| m.as_str().parse().unwrap_or(11906));
            let height: i32 = sz_caps.get(2).map_or(16838, |m| m.as_str().parse().unwrap_or(16838));
            
            // 转换为纸张大小 (twips -> 约为 1/1440 英寸)
            settings.paper_size = if width > height {
                settings.orientation = "landscape".to_string();
                if height >= 11906 - 100 { "A4".to_string() } else { "A3".to_string() }
            } else {
                if width >= 11906 - 100 { "A4".to_string() } else if width >= 14173 - 100 { "A3".to_string() } else { "B5".to_string() }
            };
        }
        
        // 解析页边距 (单位: twips, 1cm ≈ 567 twips)
        if let Some(mar_caps) = pg_mar_regex.captures(sect_content) {
            settings.margin_top = mar_caps.get(1).map_or(2.54, |m| {
                m.as_str().parse::<i32>().unwrap_or(1440) as f64 / 567.0
            });
            settings.margin_bottom = mar_caps.get(2).map_or(2.54, |m| {
                m.as_str().parse::<i32>().unwrap_or(1440) as f64 / 567.0
            });
            settings.margin_left = mar_caps.get(3).map_or(3.17, |m| {
                m.as_str().parse::<i32>().unwrap_or(1800) as f64 / 567.0
            });
            settings.margin_right = mar_caps.get(4).map_or(3.17, |m| {
                m.as_str().parse::<i32>().unwrap_or(1800) as f64 / 567.0
            });
        }
    }
    
    Ok(settings)
}

/// 解析字体设置
fn parse_font_settings(styles_xml: &str, document_xml: &str) -> Result<FontSettings, FormatExtractError> {
    let mut settings = FontSettings {
        body_font: "宋体".to_string(),
        body_font_size: "小四".to_string(),
        line_height: "1.0倍".to_string(),
    };
    
    // 解析默认字体
    let rfonts_regex = regex::Regex::new(r#"w:ascii="([^"]+)""#).unwrap();
    let sz_regex = regex::Regex::new(r#"<w:sz[^>]*w:val="(\d+)""#).unwrap();
    let spacing_regex = regex::Regex::new(r#"<w:spacing[^>]*w:line="(\d+)""#).unwrap();
    
    // 从样式或文档中提取字体
    if let Some(caps) = rfonts_regex.captures(styles_xml).or_else(|| rfonts_regex.captures(document_xml)) {
        settings.body_font = caps.get(1).map_or("宋体", |m| m.as_str()).to_string();
    }
    
    // 提取字号 (单位: half-points)
    if let Some(caps) = sz_regex.captures(document_xml).or_else(|| sz_regex.captures(styles_xml)) {
        let half_points: i32 = caps.get(1).map_or(24, |m| m.as_str().parse().unwrap_or(24));
        settings.body_font_size = points_to_chinese_font_size(half_points as f64 / 2.0);
    }
    
    // 提取行间距
    if let Some(caps) = spacing_regex.captures(document_xml) {
        let line_twips: i32 = caps.get(1).map_or(240, |m| m.as_str().parse().unwrap_or(240));
        // 240 twips = single spacing
        let ratio = line_twips as f64 / 240.0;
        settings.line_height = format!("{:.1}倍", ratio);
    }
    
    Ok(settings)
}

/// 磅值转换为中文字号
fn points_to_chinese_font_size(points: f64) -> String {
    // 中国字号与磅值对照表
    let sizes = [
        (42.0, "初号"),
        (36.0, "小初"),
        (26.0, "一号"),
        (24.0, "小一"),
        (22.0, "二号"),
        (18.0, "小二"),
        (16.0, "三号"),
        (15.0, "小三"),
        (14.0, "四号"),
        (12.0, "小四"),
        (10.5, "五号"),
        (9.0, "小五"),
        (7.5, "六号"),
        (6.5, "小六"),
    ];
    
    for (pt, name) in sizes {
        if (points - pt).abs() < 0.5 {
            return name.to_string();
        }
    }
    
    format!("{}磅", points)
}

/// 文档格式信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentFormatInfo {
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

/// 格式提取错误
#[derive(Debug, thiserror::Error)]
pub enum FormatExtractError {
    #[error("文件不存在: {0}")]
    FileNotFound(String),
    
    #[error("读取错误: {0}")]
    ReadError(String),
    
    #[error("不支持的格式: {0}")]
    UnsupportedFormat(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("LLM 错误: {0}")]
    LlmError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_points_to_chinese_font_size() {
        assert_eq!(points_to_chinese_font_size(12.0), "小四");
        assert_eq!(points_to_chinese_font_size(14.0), "四号");
        assert_eq!(points_to_chinese_font_size(16.0), "三号");
    }
    
    #[test]
    fn test_extract_json() {
        let response = r#"这是一些说明文字
```json
{"test": "value"}
```
其他文字"#;
        let result = extract_json(response).unwrap();
        assert_eq!(result, r#"{"test": "value"}"#);
    }
}
