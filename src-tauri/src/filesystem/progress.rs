use std::path::PathBuf;


#[derive(Debug, Default,Clone)]
pub struct ScanProgress {
    pub files_scanned: usize,
    pub folders_scanned: usize,
    pub current_path:PathBuf
}