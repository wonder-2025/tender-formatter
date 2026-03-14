// 差异对比引擎
// 分析原文档格式属性，对比目标格式要求，生成差异列表
//
// 设计者: wonder-宏 (产品设计) | JARVIS AI Assistant (架构设计 & 开发实现)

use crate::models::format::{FormatRequirement, ParagraphSpacing, HeadingFormat};
use std::collections::HashMap;

/// 差异类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DiffType {
    /// 需要修改
    Modify,
    /// 需要添加
    Add,
    /// 需要删除
    Delete,
}

/// 格式差异项
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FormatDiffItem {
    /// 分类
    pub category: String,
    /// 属性名
    pub property: String,
    /// 显示名称
    pub display_name: String,
    /// 当前值
    pub current_value: String,
    /// 目标值
    pub target_value: String,
    /// 差异类型
    pub diff_type: DiffType,
    /// 优先级 (1-5, 1最高)
    pub priority: u8,
    /// 是否可自动修复
    pub auto_fixable: bool,
}

/// 文档格式信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CurrentDocumentFormat {
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
    
    // 标题格式
    pub heading1: Option<HeadingFormat>,
    pub heading2: Option<HeadingFormat>,
    pub heading3: Option<HeadingFormat>,
    pub heading4: Option<HeadingFormat>,
    
    // 页眉页脚
    pub header_content: Option<String>,
    pub footer_format: Option<String>,
    
    // 表格格式
    pub table_font: Option<String>,
    pub table_font_size: Option<String>,
}

impl Default for CurrentDocumentFormat {
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
            body_line_height: "1.0倍".to_string(),
            body_paragraph_spacing: ParagraphSpacing { before: 0.0, after: 0.0 },
            
            heading1: None,
            heading2: None,
            heading3: None,
            heading4: None,
            
            header_content: None,
            footer_format: None,
            
            table_font: None,
            table_font_size: None,
        }
    }
}

/// 差异对比引擎
pub struct DiffEngine {
    /// 精度阈值
    precision: f64,
}

impl DiffEngine {
    /// 创建新的差异引擎
    pub fn new() -> Self {
        Self {
            precision: 0.01, // 0.01cm
        }
    }
    
    /// 设置精度
    pub fn with_precision(mut self, precision: f64) -> Self {
        self.precision = precision;
        self
    }
    
    /// 对比格式差异
    pub fn compare(
        &self,
        current: &CurrentDocumentFormat,
        target: &FormatRequirement,
    ) -> Vec<FormatDiffItem> {
        let mut diffs = Vec::new();
        
        // 比较页面设置
        self.compare_page_settings(current, target, &mut diffs);
        
        // 比较字体格式
        self.compare_font_settings(current, target, &mut diffs);
        
        // 比较标题格式
        self.compare_heading_settings(current, target, &mut diffs);
        
        // 比较页眉页脚
        self.compare_header_footer(current, target, &mut diffs);
        
        // 比较表格格式
        self.compare_table_settings(current, target, &mut diffs);
        
        // 按优先级排序
        diffs.sort_by(|a, b| a.priority.cmp(&b.priority));
        
        diffs
    }
    
    /// 比较页面设置
    fn compare_page_settings(
        &self,
        current: &CurrentDocumentFormat,
        target: &FormatRequirement,
        diffs: &mut Vec<FormatDiffItem>,
    ) {
        // 纸张大小
        if current.paper_size != target.paper_size {
            diffs.push(FormatDiffItem {
                category: "页面设置".to_string(),
                property: "paper_size".to_string(),
                display_name: "纸张大小".to_string(),
                current_value: current.paper_size.clone(),
                target_value: target.paper_size.clone(),
                diff_type: DiffType::Modify,
                priority: 1,
                auto_fixable: true,
            });
        }
        
        // 页面方向
        if current.orientation != target.orientation {
            diffs.push(FormatDiffItem {
                category: "页面设置".to_string(),
                property: "orientation".to_string(),
                display_name: "页面方向".to_string(),
                current_value: if current.orientation == "portrait" { "纵向" } else { "横向" }.to_string(),
                target_value: if target.orientation == "portrait" { "纵向" } else { "横向" }.to_string(),
                diff_type: DiffType::Modify,
                priority: 1,
                auto_fixable: true,
            });
        }
        
        // 页边距
        self.compare_margin("top", current.margin_top, target.margin_top, "页边距-上", diffs);
        self.compare_margin("bottom", current.margin_bottom, target.margin_bottom, "页边距-下", diffs);
        self.compare_margin("left", current.margin_left, target.margin_left, "页边距-左", diffs);
        self.compare_margin("right", current.margin_right, target.margin_right, "页边距-右", diffs);
        
        // 装订线
        if (current.gutter - target.gutter).abs() > self.precision {
            diffs.push(FormatDiffItem {
                category: "页面设置".to_string(),
                property: "gutter".to_string(),
                display_name: "装订线".to_string(),
                current_value: format!("{:.2}cm", current.gutter),
                target_value: format!("{:.2}cm", target.gutter),
                diff_type: DiffType::Modify,
                priority: 2,
                auto_fixable: true,
            });
        }
        
        // 装订位置
        if current.gutter_position != target.gutter_position && target.gutter > 0.0 {
            diffs.push(FormatDiffItem {
                category: "页面设置".to_string(),
                property: "gutter_position".to_string(),
                display_name: "装订位置".to_string(),
                current_value: if current.gutter_position == "left" { "左" } else { "上" }.to_string(),
                target_value: if target.gutter_position == "left" { "左" } else { "上" }.to_string(),
                diff_type: DiffType::Modify,
                priority: 2,
                auto_fixable: true,
            });
        }
    }
    
    /// 比较页边距
    fn compare_margin(
        &self,
        position: &str,
        current: f64,
        target: f64,
        display_name: &str,
        diffs: &mut Vec<FormatDiffItem>,
    ) {
        if (current - target).abs() > self.precision {
            diffs.push(FormatDiffItem {
                category: "页面设置".to_string(),
                property: format!("margin_{}", position),
                display_name: display_name.to_string(),
                current_value: format!("{:.2}cm", current),
                target_value: format!("{:.2}cm", target),
                diff_type: DiffType::Modify,
                priority: 2,
                auto_fixable: true,
            });
        }
    }
    
    /// 比较字体设置
    fn compare_font_settings(
        &self,
        current: &CurrentDocumentFormat,
        target: &FormatRequirement,
        diffs: &mut Vec<FormatDiffItem>,
    ) {
        // 正文字体
        if current.body_font != target.body_font {
            diffs.push(FormatDiffItem {
                category: "字体格式".to_string(),
                property: "body_font".to_string(),
                display_name: "正文字体".to_string(),
                current_value: current.body_font.clone(),
                target_value: target.body_font.clone(),
                diff_type: DiffType::Modify,
                priority: 3,
                auto_fixable: true,
            });
        }
        
        // 正文字号
        if current.body_font_size != target.body_font_size {
            diffs.push(FormatDiffItem {
                category: "字体格式".to_string(),
                property: "body_font_size".to_string(),
                display_name: "正文字号".to_string(),
                current_value: current.body_font_size.clone(),
                target_value: target.body_font_size.clone(),
                diff_type: DiffType::Modify,
                priority: 3,
                auto_fixable: true,
            });
        }
        
        // 行间距
        if current.body_line_height != target.body_line_height {
            diffs.push(FormatDiffItem {
                category: "字体格式".to_string(),
                property: "body_line_height".to_string(),
                display_name: "行间距".to_string(),
                current_value: current.body_line_height.clone(),
                target_value: target.body_line_height.clone(),
                diff_type: DiffType::Modify,
                priority: 3,
                auto_fixable: true,
            });
        }
        
        // 段落间距
        if (current.body_paragraph_spacing.before - target.body_paragraph_spacing.before).abs() > self.precision {
            diffs.push(FormatDiffItem {
                category: "字体格式".to_string(),
                property: "paragraph_spacing_before".to_string(),
                display_name: "段前间距".to_string(),
                current_value: format!("{:.1}行", current.body_paragraph_spacing.before),
                target_value: format!("{:.1}行", target.body_paragraph_spacing.before),
                diff_type: DiffType::Modify,
                priority: 4,
                auto_fixable: true,
            });
        }
        
        if (current.body_paragraph_spacing.after - target.body_paragraph_spacing.after).abs() > self.precision {
            diffs.push(FormatDiffItem {
                category: "字体格式".to_string(),
                property: "paragraph_spacing_after".to_string(),
                display_name: "段后间距".to_string(),
                current_value: format!("{:.1}行", current.body_paragraph_spacing.after),
                target_value: format!("{:.1}行", target.body_paragraph_spacing.after),
                diff_type: DiffType::Modify,
                priority: 4,
                auto_fixable: true,
            });
        }
    }
    
    /// 比较标题设置
    fn compare_heading_settings(
        &self,
        current: &CurrentDocumentFormat,
        target: &FormatRequirement,
        diffs: &mut Vec<FormatDiffItem>,
    ) {
        self.compare_heading(1, &current.heading1, &target.heading1, diffs);
        self.compare_heading(2, &current.heading2, &target.heading2, diffs);
        self.compare_heading(3, &current.heading3, &target.heading3, diffs);
        self.compare_heading(4, &current.heading4, &target.heading4, diffs);
    }
    
    /// 比较单个标题
    fn compare_heading(
        &self,
        level: u8,
        current: &Option<HeadingFormat>,
        target: &HeadingFormat,
        diffs: &mut Vec<FormatDiffItem>,
    ) {
        let level_name = match level {
            1 => "一级标题",
            2 => "二级标题",
            3 => "三级标题",
            4 => "四级标题",
            _ => return,
        };
        
        if let Some(ref current_fmt) = current {
            // 比较字体
            if current_fmt.font != target.font {
                diffs.push(FormatDiffItem {
                    category: "标题格式".to_string(),
                    property: format!("heading{}_font", level),
                    display_name: format!("{}字体", level_name),
                    current_value: current_fmt.font.clone(),
                    target_value: target.font.clone(),
                    diff_type: DiffType::Modify,
                    priority: 3,
                    auto_fixable: true,
                });
            }
            
            // 比较字号
            if current_fmt.font_size != target.font_size {
                diffs.push(FormatDiffItem {
                    category: "标题格式".to_string(),
                    property: format!("heading{}_font_size", level),
                    display_name: format!("{}字号", level_name),
                    current_value: current_fmt.font_size.clone(),
                    target_value: target.font_size.clone(),
                    diff_type: DiffType::Modify,
                    priority: 3,
                    auto_fixable: true,
                });
            }
        } else {
            // 当前没有标题样式，需要添加
            diffs.push(FormatDiffItem {
                category: "标题格式".to_string(),
                property: format!("heading{}", level),
                display_name: level_name.to_string(),
                current_value: "未设置".to_string(),
                target_value: format!("{} {}", target.font, target.font_size),
                diff_type: DiffType::Add,
                priority: 3,
                auto_fixable: true,
            });
        }
    }
    
    /// 比较页眉页脚
    fn compare_header_footer(
        &self,
        current: &CurrentDocumentFormat,
        target: &FormatRequirement,
        diffs: &mut Vec<FormatDiffItem>,
    ) {
        // 页眉内容
        if let Some(ref header_content) = current.header_content {
            if header_content != &target.header.content {
                diffs.push(FormatDiffItem {
                    category: "页眉页脚".to_string(),
                    property: "header_content".to_string(),
                    display_name: "页眉内容".to_string(),
                    current_value: header_content.clone(),
                    target_value: target.header.content.clone(),
                    diff_type: DiffType::Modify,
                    priority: 4,
                    auto_fixable: true,
                });
            }
        }
        
        // 页脚格式
        if let Some(ref footer_format) = current.footer_format {
            if footer_format != &target.footer.format {
                diffs.push(FormatDiffItem {
                    category: "页眉页脚".to_string(),
                    property: "footer_format".to_string(),
                    display_name: "页码格式".to_string(),
                    current_value: footer_format.clone(),
                    target_value: target.footer.format.clone(),
                    diff_type: DiffType::Modify,
                    priority: 4,
                    auto_fixable: true,
                });
            }
        }
    }
    
    /// 比较表格设置
    fn compare_table_settings(
        &self,
        current: &CurrentDocumentFormat,
        target: &FormatRequirement,
        diffs: &mut Vec<FormatDiffItem>,
    ) {
        // 表格字体
        if let Some(ref table_font) = current.table_font {
            if table_font != &target.table_font {
                diffs.push(FormatDiffItem {
                    category: "表格格式".to_string(),
                    property: "table_font".to_string(),
                    display_name: "表格字体".to_string(),
                    current_value: table_font.clone(),
                    target_value: target.table_font.clone(),
                    diff_type: DiffType::Modify,
                    priority: 5,
                    auto_fixable: true,
                });
            }
        }
        
        // 表格字号
        if let Some(ref table_font_size) = current.table_font_size {
            if table_font_size != &target.table_font_size {
                diffs.push(FormatDiffItem {
                    category: "表格格式".to_string(),
                    property: "table_font_size".to_string(),
                    display_name: "表格字号".to_string(),
                    current_value: table_font_size.clone(),
                    target_value: target.table_font_size.clone(),
                    diff_type: DiffType::Modify,
                    priority: 5,
                    auto_fixable: true,
                });
            }
        }
    }
    
    /// 生成差异摘要
    pub fn generate_summary(diffs: &[FormatDiffItem]) -> DiffSummary {
        let mut by_category: HashMap<String, usize> = HashMap::new();
        let mut auto_fixable_count = 0;
        
        for diff in diffs {
            *by_category.entry(diff.category.clone()).or_insert(0) += 1;
            if diff.auto_fixable {
                auto_fixable_count += 1;
            }
        }
        
        DiffSummary {
            total_count: diffs.len(),
            by_category,
            auto_fixable_count,
            needs_manual_fix: diffs.len() - auto_fixable_count,
        }
    }
}

impl Default for DiffEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 差异摘要
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiffSummary {
    /// 总差异数
    pub total_count: usize,
    /// 按分类统计
    pub by_category: HashMap<String, usize>,
    /// 可自动修复数
    pub auto_fixable_count: usize,
    /// 需手动修复数
    pub needs_manual_fix: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_diff_engine() {
        let engine = DiffEngine::new();
        let current = CurrentDocumentFormat::default();
        let target = FormatRequirement::default();
        
        // 默认值应该相同，没有差异
        let diffs = engine.compare(&current, &target);
        assert!(diffs.is_empty());
    }
    
    #[test]
    fn test_margin_diff() {
        let engine = DiffEngine::new();
        let mut current = CurrentDocumentFormat::default();
        current.margin_top = 2.0;
        
        let target = FormatRequirement::default();
        let diffs = engine.compare(&current, &target);
        
        assert!(diffs.iter().any(|d| d.display_name == "页边距-上"));
    }
    
    #[test]
    fn test_summary() {
        let diffs = vec![
            FormatDiffItem {
                category: "页面设置".to_string(),
                property: "margin_top".to_string(),
                display_name: "页边距-上".to_string(),
                current_value: "2.00cm".to_string(),
                target_value: "2.54cm".to_string(),
                diff_type: DiffType::Modify,
                priority: 2,
                auto_fixable: true,
            },
        ];
        
        let summary = DiffEngine::generate_summary(&diffs);
        assert_eq!(summary.total_count, 1);
        assert_eq!(summary.auto_fixable_count, 1);
    }
}
