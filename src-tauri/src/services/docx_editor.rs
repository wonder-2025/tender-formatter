// Word/WPS 文档编辑器
// 使用 docx-rs 库操作 Word 文档
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use crate::models::format::{FormatRequirement, ParagraphSpacing, HeadingFormat, HeaderFormat, FooterFormat};
use std::path::Path;
use std::fs;
use std::io::{Read, Write, Cursor};
use zip::{ZipArchive, ZipWriter};
use zip::write::SimpleFileOptions;
use quick_xml::{Reader, Writer, events::Event};

/// Word 文档编辑器
pub struct DocxEditor {
    content: Vec<u8>,
}

impl DocxEditor {
    /// 从文件创建编辑器
    pub fn from_file(path: &str) -> Result<Self, DocxError> {
        let content = fs::read(path)
            .map_err(|e| DocxError::ReadError(format!("无法读取文件: {}", e)))?;
        Ok(Self { content })
    }
    
    /// 从内存创建编辑器
    pub fn from_bytes(content: Vec<u8>) -> Result<Self, DocxError> {
        Ok(Self { content })
    }
    
    /// 应用格式修改
    pub fn apply_format(&mut self, format: &FormatRequirement) -> Result<(), DocxError> {
        // 读取 ZIP 内容
        let reader = Cursor::new(&self.content);
        let mut archive = ZipArchive::new(reader)
            .map_err(|e| DocxError::InvalidFormat(format!("无效的 DOCX 文件: {}", e)))?;
        
        // 修改 document.xml
        let document_xml = self.modify_document_xml(&mut archive, format)?;
        
        // 修改 styles.xml
        let styles_xml = self.modify_styles_xml(&mut archive, format)?;
        
        // 重新打包
        let mut output = Vec::new();
        {
            let cursor = Cursor::new(&mut output);
            let mut writer = ZipWriter::new(cursor);
            
            // 复制所有文件
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let name = file.name().to_string();
                
                if name == "word/document.xml" {
                    writer.start_file(&name, SimpleFileOptions::default())
                        .map_err(|e| DocxError::WriteError(e.to_string()))?;
                    writer.write_all(&document_xml)
                        .map_err(|e| DocxError::WriteError(e.to_string()))?;
                } else if name == "word/styles.xml" {
                    writer.start_file(&name, SimpleFileOptions::default())
                        .map_err(|e| DocxError::WriteError(e.to_string()))?;
                    writer.write_all(&styles_xml)
                        .map_err(|e| DocxError::WriteError(e.to_string()))?;
                } else {
                    writer.start_file(&name, SimpleFileOptions::default())
                        .map_err(|e| DocxError::WriteError(e.to_string()))?;
                    std::io::copy(&mut file, &mut writer)
                        .map_err(|e| DocxError::WriteError(e.to_string()))?;
                }
            }
            
            writer.finish().map_err(|e| DocxError::WriteError(e.to_string()))?;
        }
        
        self.content = output;
        Ok(())
    }
    
    /// 保存到文件
    pub fn save_to_file(&self, path: &str) -> Result<(), DocxError> {
        // 确保父目录存在
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| DocxError::WriteError(format!("创建目录失败: {}", e)))?;
        }
        
        fs::write(path, &self.content)
            .map_err(|e| DocxError::WriteError(format!("写入文件失败: {}", e)))?;
        
        Ok(())
    }
    
    /// 获取内容
    pub fn into_bytes(self) -> Vec<u8> {
        self.content
    }
    
    /// 修改 document.xml
    fn modify_document_xml(
        &self,
        archive: &mut ZipArchive<Cursor<&Vec<u8>>>,
        format: &FormatRequirement,
    ) -> Result<Vec<u8>, DocxError> {
        let mut document_xml = String::new();
        {
            let mut file = archive.by_name("word/document.xml")
                .map_err(|e| DocxError::ReadError(format!("无法读取 document.xml: {}", e)))?;
            file.read_to_string(&mut document_xml)
                .map_err(|e| DocxError::ReadError(format!("读取内容失败: {}", e)))?;
        }
        
        // 修改页面设置
        document_xml = self.modify_page_settings(&document_xml, format)?;
        
        // 修改段落格式
        document_xml = self.modify_paragraph_properties(&document_xml, format)?;
        
        Ok(document_xml.into_bytes())
    }
    
    /// 修改页面设置
    fn modify_page_settings(&self, xml: &str, format: &FormatRequirement) -> Result<String, DocxError> {
        // 构建新的 sectPr
        let paper_size = match format.paper_size.as_str() {
            "A3" => (16838, 23814),
            "B5" => (14175, 19985),
            _ => (11906, 16838), // A4
        };
        
        let (width, height) = if format.orientation == "landscape" {
            (paper_size.1, paper_size.0)
        } else {
            paper_size
        };
        
        // 转换 cm 到 twips (1cm ≈ 567 twips)
        let margin_top = (format.margin_top * 567.0) as i32;
        let margin_bottom = (format.margin_bottom * 567.0) as i32;
        let margin_left = (format.margin_left * 567.0) as i32;
        let margin_right = (format.margin_right * 567.0) as i32;
        let gutter = (format.gutter * 567.0) as i32;
        
        let gutter_pos = if format.gutter_position == "top" { "top" } else { "left" };
        
        let new_sect_pr = format!(
            r#"<w:sectPr>
                <w:pgSz w:w="{}" w:h="{}"/>
                <w:pgMar w:top="{}" w:right="{}" w:bottom="{}" w:left="{}" w:gutter="{}" w:header="851" w:footer="992"/>
                <w:cols w:space="425"/>
                <w:docGrid w:type="lines" w:linePitch="312"/>
            </w:sectPr>"#,
            width, height, margin_top, margin_right, margin_bottom, margin_left, gutter
        );
        
        // 替换 sectPr
        let sect_pr_regex = regex::Regex::new(r"<w:sectPr>.*?</w:sectPr>").unwrap();
        let result = sect_pr_regex.replace(xml, &new_sect_pr).to_string();
        
        Ok(result)
    }
    
    /// 修改段落属性
    fn modify_paragraph_properties(&self, xml: &str, format: &FormatRequirement) -> Result<String, DocxError> {
        let mut result = xml.to_string();
        
        // 计算行间距值 (单位: twips)
        // 1.0倍行距 ≈ 240 twips (12pt = 240 twips)
        let line_spacing = match format.body_line_height.as_str() {
            "1.0倍" => 240,
            "1.25倍" => 300,
            "1.5倍" => 360,
            "2.0倍" => 480,
            _ => 360, // 默认 1.5倍
        };
        
        // 修改所有段落的行间距
        let p_pr_regex = regex::Regex::new(r"<w:pPr>(.*?)</w:pPr>").unwrap();
        let spacing_regex = regex::Regex::new(r"<w:spacing[^>]*/>").unwrap();
        
        result = p_pr_regex.replace_all(&result, |caps: &regex::Captures| {
            let content = caps.get(1).map_or("", |m| m.as_str());
            
            // 检查是否已有 spacing
            if spacing_regex.is_match(content) {
                // 替换现有的 spacing
                let new_spacing = format!(r#"<w:spacing w:line="{}" w:lineRule="auto"/>"#, line_spacing);
                let new_content = spacing_regex.replace(content, &new_spacing);
                format!("<w:pPr>{}</w:pPr>", new_content)
            } else {
                // 添加新的 spacing
                format!(r#"<w:pPr>{}<w:spacing w:line="{}" w:lineRule="auto"/></w:pPr>"#, content, line_spacing)
            }
        }).to_string();
        
        Ok(result)
    }
    
    /// 修改 styles.xml
    fn modify_styles_xml(
        &self,
        archive: &mut ZipArchive<Cursor<&Vec<u8>>>,
        format: &FormatRequirement,
    ) -> Result<Vec<u8>, DocxError> {
        let mut styles_xml = String::new();
        {
            let mut file = archive.by_name("word/styles.xml")
                .map_err(|e| DocxError::ReadError(format!("无法读取 styles.xml: {}", e)))?;
            file.read_to_string(&mut styles_xml)
                .map_err(|e| DocxError::ReadError(format!("读取内容失败: {}", e)))?;
        }
        
        // 修改正文样式
        styles_xml = self.modify_normal_style(&styles_xml, format)?;
        
        // 修改标题样式
        styles_xml = self.modify_heading_style(&styles_xml, "1", &format.heading1)?;
        styles_xml = self.modify_heading_style(&styles_xml, "2", &format.heading2)?;
        styles_xml = self.modify_heading_style(&styles_xml, "3", &format.heading3)?;
        styles_xml = self.modify_heading_style(&styles_xml, "4", &format.heading4)?;
        
        Ok(styles_xml.into_bytes())
    }
    
    /// 修改正文样式
    fn modify_normal_style(&self, xml: &str, format: &FormatRequirement) -> Result<String, DocxError> {
        let font_size = chinese_font_size_to_half_points(&format.body_font_size);
        
        // 替换 Normal 样式中的字体和字号
        let normal_regex = regex::Regex::new(
            r#"(<w:style[^>]*w:styleId="Normal"[^>]*>.*?</w:style>)"#
        ).unwrap();
        
        let result = xml.to_string();
        
        // 修改文档默认字体
        let rfonts_regex = regex::Regex::new(r#"w:ascii="[^"]*""#).unwrap();
        let east_asia_regex = regex::Regex::new(r#"w:eastAsia="[^"]*""#).unwrap();
        let sz_regex = regex::Regex::new(r#"<w:sz w:val="\d+"/>"#).unwrap();
        let sz_cs_regex = regex::Regex::new(r#"<w:szCs w:val="\d+"/>"#).unwrap();
        
        let mut result = rfonts_regex.replace_all(&result, &format!(r#"w:ascii="{}""#, format.body_font)).to_string();
        result = east_asia_regex.replace_all(&result, &format!(r#"w:eastAsia="{}""#, format.body_font)).to_string();
        result = sz_regex.replace_all(&result, &format!(r#"<w:sz w:val="{}"/>"#, font_size)).to_string();
        result = sz_cs_regex.replace_all(&result, &format!(r#"<w:szCs w:val="{}"/>"#, font_size)).to_string();
        
        Ok(result)
    }
    
    /// 修改标题样式
    fn modify_heading_style(
        &self,
        xml: &str,
        level: &str,
        heading: &HeadingFormat,
    ) -> Result<String, DocxError> {
        let font_size = chinese_font_size_to_half_points(&heading.font_size);
        let spacing_before = (heading.spacing.before * 100.0) as i32; // 磅到磅的百分比
        let spacing_after = (heading.spacing.after * 100.0) as i32;
        
        // 使用正则表达式匹配并替换标题样式
        let style_id = format!("Heading{}", level);
        let pattern = format!(
            r#"(<w:style[^>]*w:styleId="{}"[^>]*>.*?</w:style>)"#,
            style_id
        );
        let heading_regex = regex::Regex::new(&pattern).unwrap();
        
        let mut result = xml.to_string();
        
        // 修改字体
        let rfonts_regex = regex::Regex::new(r#"w:eastAsia="[^"]*""#).unwrap();
        let sz_regex = regex::Regex::new(r#"<w:sz w:val="\d+"/>"#).unwrap();
        let spacing_regex = regex::Regex::new(r#"<w:spacing[^>]*/>"#).unwrap();
        
        // 在标题样式区域进行替换
        if let Some(caps) = heading_regex.captures(&result) {
            let style_content = caps.get(1).unwrap().as_str();
            
            let mut new_style = style_content.to_string();
            new_style = rfonts_regex.replace_all(&new_style, &format!(r#"w:eastAsia="{}""#, heading.font)).to_string();
            new_style = sz_regex.replace_all(&new_style, &format!(r#"<w:sz w:val="{}"/>"#, font_size)).to_string();
            
            // 修改间距
            let new_spacing = format!(
                r#"<w:spacing w:before="{}" w:after="{}"/>"#,
                spacing_before * 10, // 转换为 twips
                spacing_after * 10
            );
            new_style = spacing_regex.replace(&new_style, &new_spacing).to_string();
            
            result = result.replace(style_content, &new_style);
        }
        
        Ok(result)
    }
}

/// 中文字号转换为半磅值
fn chinese_font_size_to_half_points(size: &str) -> u32 {
    // 中文字号对应的磅值
    let sizes: [(&str, f64); 14] = [
        ("初号", 42.0),
        ("小初", 36.0),
        ("一号", 26.0),
        ("小一", 24.0),
        ("二号", 22.0),
        ("小二", 18.0),
        ("三号", 16.0),
        ("小三", 15.0),
        ("四号", 14.0),
        ("小四", 12.0),
        ("五号", 10.5),
        ("小五", 9.0),
        ("六号", 7.5),
        ("小六", 6.5),
    ];
    
    for (name, points) in sizes {
        if size.contains(name) {
            return (points * 2.0) as u32; // 返回半磅值
        }
    }
    
    // 尝试解析数字
    if let Ok(pt) = size.replace("磅", "").parse::<f64>() {
        return (pt * 2.0) as u32;
    }
    
    24 // 默认小四
}

/// 应用格式到文档
pub fn apply_format(
    input_path: &str,
    output_path: &str,
    format: &FormatRequirement,
) -> Result<(), DocxError> {
    let mut editor = DocxEditor::from_file(input_path)?;
    editor.apply_format(format)?;
    editor.save_to_file(output_path)?;
    Ok(())
}

/// 备份文档
pub fn backup_document(input_path: &str) -> Result<String, DocxError> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = format!("{}.backup.{}.docx", input_path, timestamp);
    
    fs::copy(input_path, &backup_path)
        .map_err(|e| DocxError::WriteError(format!("备份失败: {}", e)))?;
    
    Ok(backup_path)
}

/// 获取文档信息
pub fn get_document_info(path: &str) -> Result<DocumentInfo, DocxError> {
    let file_path = Path::new(path);
    
    if !file_path.exists() {
        return Err(DocxError::FileNotFound(path.to_string()));
    }
    
    let metadata = fs::metadata(path)
        .map_err(|e| DocxError::ReadError(format!("读取文件信息失败: {}", e)))?;
    
    let name = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let extension = file_path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    
    Ok(DocumentInfo {
        path: path.to_string(),
        name,
        extension,
        size: metadata.len(),
    })
}

/// 文档信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
}

/// 文档操作错误
#[derive(Debug, thiserror::Error)]
pub enum DocxError {
    #[error("文件不存在: {0}")]
    FileNotFound(String),
    
    #[error("读取错误: {0}")]
    ReadError(String),
    
    #[error("写入错误: {0}")]
    WriteError(String),
    
    #[error("无效格式: {0}")]
    InvalidFormat(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chinese_font_size_conversion() {
        assert_eq!(chinese_font_size_to_half_points("小四"), 24);
        assert_eq!(chinese_font_size_to_half_points("四号"), 28);
        assert_eq!(chinese_font_size_to_half_points("三号"), 32);
    }
}
