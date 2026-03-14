use serde::{Deserialize, Serialize};

/// 格式要求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatRequirement {
    // 页面设置
    pub paper_size: String,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub margin_right: f64,
    pub gutter: f64,
    pub gutter_position: String,
    pub orientation: String,
    
    // 字体格式
    pub body_font: String,
    pub body_font_size: String,
    pub body_line_height: String,
    pub body_paragraph_spacing: ParagraphSpacing,
    
    pub heading1: HeadingFormat,
    pub heading2: HeadingFormat,
    pub heading3: HeadingFormat,
    pub heading4: HeadingFormat,
    
    // 页眉页脚
    pub header: HeaderFormat,
    pub footer: FooterFormat,
    
    // 表格格式
    pub table_font: String,
    pub table_font_size: String,
    pub table_border: String,
    pub table_align: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphSpacing {
    pub before: f64,
    pub after: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingFormat {
    pub font: String,
    pub font_size: String,
    pub spacing: ParagraphSpacing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderFormat {
    pub content: String,
    pub font: String,
    pub font_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FooterFormat {
    pub position: String,
    pub format: String,
}

impl Default for FormatRequirement {
    fn default() -> Self {
        Self {
            paper_size: "A4".to_string(),
            margin_top: 2.54,
            margin_bottom: 2.54,
            margin_left: 3.17,
            margin_right: 3.17,
            gutter: 0.0,
            gutter_position: "left".to_string(),
            orientation: "portrait".to_string(),
            
            body_font: "宋体".to_string(),
            body_font_size: "小四".to_string(),
            body_line_height: "1.5倍".to_string(),
            body_paragraph_spacing: ParagraphSpacing { before: 0.0, after: 0.0 },
            
            heading1: HeadingFormat {
                font: "黑体".to_string(),
                font_size: "三号".to_string(),
                spacing: ParagraphSpacing { before: 0.5, after: 0.5 },
            },
            heading2: HeadingFormat {
                font: "黑体".to_string(),
                font_size: "四号".to_string(),
                spacing: ParagraphSpacing { before: 0.5, after: 0.5 },
            },
            heading3: HeadingFormat {
                font: "黑体".to_string(),
                font_size: "小四".to_string(),
                spacing: ParagraphSpacing { before: 0.0, after: 0.0 },
            },
            heading4: HeadingFormat {
                font: "黑体".to_string(),
                font_size: "小四".to_string(),
                spacing: ParagraphSpacing { before: 0.0, after: 0.0 },
            },
            
            header: HeaderFormat {
                content: "项目名称".to_string(),
                font: "宋体".to_string(),
                font_size: "小五".to_string(),
            },
            footer: FooterFormat {
                position: "居中".to_string(),
                format: "第X页 共X页".to_string(),
            },
            
            table_font: "宋体".to_string(),
            table_font_size: "小四".to_string(),
            table_border: "单线".to_string(),
            table_align: "居中".to_string(),
        }
    }
}
