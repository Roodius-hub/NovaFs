

use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,

    pub is_dir: bool,

    pub extension: Option<String>,
    pub size: u64,

    pub created_at: Option<SystemTime>,
    pub modified_at: Option<SystemTime>,
}