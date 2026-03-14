use std::path::{Path, PathBuf};
use std::fs;

// ==================== 安全配置 ====================

/// 最大文件大小限制：100MB
/// 防止恶意用户上传超大文件导致内存溢出
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;

/// 允许的文件扩展名白名单
/// 只允许处理特定类型的文档，防止处理敏感系统文件
const ALLOWED_EXTENSIONS: &[&str] = &["docx", "doc", "pdf", "xlsx", "xls", "txt"];

// ==================== 路径验证 ====================

/// 验证文件路径安全性
/// 
/// # 安全说明
/// 此函数防止路径遍历攻击（Path Traversal），确保用户只能访问
/// 应用程序工作目录内的文件，无法读取系统敏感文件如 /etc/passwd
/// 
/// # 参数
/// * `path` - 待验证的文件路径字符串
/// 
/// # 返回
/// * `Ok(PathBuf)` - 规范化后的安全路径
/// * `Err(String)` - 路径验证失败的原因
fn validate_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    
    // 规范化路径，解析所有符号链接和相对路径组件
    let canonical = path.canonicalize()
        .map_err(|e| format!("路径无效: {}", e))?;
    
    // 获取当前工作目录作为安全边界
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("获取当前目录失败: {}", e))?;
    
    // 安全检查：确保规范化后的路径在当前工作目录内
    // 防止路径遍历攻击（如 "../../../etc/passwd"）
    if !canonical.starts_with(&current_dir) {
        return Err("安全错误：路径遍历攻击检测，禁止访问工作目录外的文件".to_string());
    }
    
    Ok(canonical)
}

/// 验证文件扩展名是否在白名单中
fn validate_extension(path: &Path) -> Result<String, String> {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(format!(
            "不支持的文件类型: .{}，允许的类型: {}",
            ext,
            ALLOWED_EXTENSIONS.join(", ")
        ));
    }
    
    Ok(ext)
}

/// 验证文件大小是否超过限制
fn validate_file_size(path: &Path) -> Result<u64, String> {
    let metadata = fs::metadata(path)
        .map_err(|e| format!("读取文件信息失败: {}", e))?;
    
    let size = metadata.len();
    if size > MAX_FILE_SIZE {
        return Err(format!(
            "文件过大 ({:.2}MB)，最大支持 {}MB",
            size as f64 / 1024.0 / 1024.0,
            MAX_FILE_SIZE / 1024 / 1024
        ));
    }
    
    Ok(size)
}

// ==================== 命令实现 ====================

/// 打开文档
/// 
/// # 安全措施
/// 1. 路径验证：防止路径遍历攻击
/// 2. 文件类型检查：只允许处理白名单中的文件类型
/// 3. 文件大小限制：防止内存溢出
#[tauri::command]
pub async fn open_document(file_path: String) -> Result<DocumentInfo, String> {
    // 安全检查：验证路径
    let canonical_path = validate_path(&file_path)?;
    
    // 安全检查：验证文件扩展名
    let extension = validate_extension(&canonical_path)?;
    
    // 安全检查：验证文件大小
    let size = validate_file_size(&canonical_path)?;
    
    // 检查文件是否存在
    if !canonical_path.exists() {
        return Err("文件不存在".to_string());
    }
    
    let file_name = canonical_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    Ok(DocumentInfo {
        path: canonical_path.to_string_lossy().to_string(),
        name: file_name,
        extension,
        size,
    })
}

/// 保存文档
/// 
/// # 安全措施
/// 1. 源文件和目标文件都进行路径验证
/// 2. 文件类型检查
#[tauri::command]
pub async fn save_document(
    source_path: String,
    target_path: String,
) -> Result<(), String> {
    // 安全检查：验证源文件路径
    let source_canonical = validate_path(&source_path)?;
    
    // 安全检查：验证目标文件路径
    // 目标文件可能不存在，使用父目录验证
    let target = PathBuf::from(&target_path);
    if let Some(parent) = target.parent() {
        if parent.exists() {
            let parent_canonical = parent.canonicalize()
                .map_err(|e| format!("目标目录验证失败: {}", e))?;
            let current_dir = std::env::current_dir()
                .map_err(|e| format!("获取当前目录失败: {}", e))?;
            if !parent_canonical.starts_with(&current_dir) {
                return Err("安全错误：目标路径超出工作目录范围".to_string());
            }
        }
    }
    
    // 安全检查：验证目标文件扩展名
    validate_extension(&target)?;
    
    fs::copy(&source_canonical, &target)
        .map_err(|e| format!("保存文档失败: {}", e))?;
    
    Ok(())
}

/// 备份文档
/// 
/// # 安全措施
/// 1. 路径验证：防止路径遍历攻击
/// 2. 文件大小检查
#[tauri::command]
pub fn backup_document(file_path: String) -> Result<String, String> {
    // 安全检查：验证路径
    let canonical_path = validate_path(&file_path)?;
    
    // 安全检查：验证文件大小
    validate_file_size(&canonical_path)?;
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = format!(
        "{}.backup.{}",
        canonical_path.to_string_lossy(),
        timestamp
    );
    
    fs::copy(&canonical_path, &backup_path)
        .map_err(|e| format!("备份失败: {}", e))?;
    
    Ok(backup_path)
}

/// 文档信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
}
