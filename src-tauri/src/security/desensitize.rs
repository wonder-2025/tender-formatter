// 敏感信息脱敏
// 复用检查工具的脱敏逻辑

use regex::Regex;

/// 脱敏规则
pub struct DesensitizeRule {
    pub name: String,
    pub pattern: Regex,
    pub replacement: String,
}

/// 默认脱敏规则
pub fn get_default_rules() -> Vec<DesensitizeRule> {
    vec![
        // 手机号
        DesensitizeRule {
            name: "手机号".to_string(),
            pattern: Regex::new(r"1[3-9]\d{9}").unwrap(),
            replacement: "[手机号]".to_string(),
        },
        // 身份证号
        DesensitizeRule {
            name: "身份证号".to_string(),
            pattern: Regex::new(r"\d{17}[\dXx]").unwrap(),
            replacement: "[身份证号]".to_string(),
        },
        // 金额
        DesensitizeRule {
            name: "金额".to_string(),
            pattern: Regex::new(r"\d{1,3}(,\d{3})*(\.\d{2})?\s*(元|万元|亿元)").unwrap(),
            replacement: "[金额]".to_string(),
        },
        // 公司名称
        DesensitizeRule {
            name: "公司名称".to_string(),
            pattern: Regex::new(r"[\u4e00-\u9fa5]{2,}(公司|集团|有限|股份)").unwrap(),
            replacement: "[公司名]".to_string(),
        },
    ]
}

/// 对文本进行脱敏
pub fn desensitize(text: &str, rules: &[DesensitizeRule]) -> String {
    let mut result = text.to_string();
    for rule in rules {
        result = rule.pattern.replace_all(&result, &rule.replacement).to_string();
    }
    result
}
