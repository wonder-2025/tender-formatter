use std::path::Path;
use std::fs;

/// 打开文档
#[tauri::command]
pub async fn open_document(file_path: String) -> Result<DocumentInfo, String> {
    let path = Path::new(&file_path);
    
    if !path.exists() {
        return Err("文件不存在".to_string());
    }
    
    let metadata = fs::metadata(&file_path)
        .map_err(|e| format!("读取文件信息失败: {}", e))?;
    
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    
    Ok(DocumentInfo {
        path: file_path,
        name: file_name,
        extension,
        size: metadata.len(),
    })
}

/// 保存文档
#[tauri::command]
pub async fn save_document(
    source_path: String,
    target_path: String,
) -> Result<(), String> {
    fs::copy(&source_path, &target_path)
        .map_err(|e| format!("保存文档失败: {}", e))?;
    
    Ok(())
}

/// 备份文档
#[tauri::command]
pub fn backup_document(file_path: String) -> Result<String, String> {
    let path = Path::new(&file_path);
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = format!(
        "{}.backup.{}",
        file_path,
        timestamp
    );
    
    fs::copy(&file_path, &backup_path)
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
