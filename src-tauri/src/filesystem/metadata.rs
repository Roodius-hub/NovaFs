use std::{
    path::{Path, PathBuf}, time::SystemTime,
};

use std::io;



#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: PathBuf,

    // Basic Information
    pub name: String,
    pub extension: Option<String>,
    pub is_dir: bool,

    // Size
    pub size: u64,

    // Time
    pub created_at: Option<SystemTime>,
    pub modified_at: Option<SystemTime>,
    pub accessed_at: Option<SystemTime>,

    // Permissions
    pub readonly: bool,

    // Platform specific
    pub hidden: bool,
}


pub fn get_metadata(path: &Path) -> io::Result<FileMetadata> {
    let fs_metadata = std::fs::metadata(path)?;

    Ok(FileMetadata { 
        name:path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        path:path.to_path_buf(),
        extension:path.extension().map(|ext| ext.to_string_lossy().to_string()),
        is_dir:fs_metadata.is_dir(),
        size:fs_metadata.len(),
        created_at:fs_metadata.created().ok(),
        modified_at:fs_metadata.modified().ok(),
        accessed_at:fs_metadata.accessed().ok(),
        readonly:fs_metadata.permissions().readonly(),
        hidden:is_hidden(path),
    })
}


pub fn is_hidden(path:&Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}