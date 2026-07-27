

use std::path::PathBuf;
use std::time::SystemTime;

use serde::Serialize;

#[derive(Debug, Clone,Serialize)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,

    pub is_dir: bool,
    pub children: Vec<FileNode>,
    pub extension: Option<String>,
    pub size: u64,

    pub created_at: Option<SystemTime>,
    pub modified_at: Option<SystemTime>,
}

