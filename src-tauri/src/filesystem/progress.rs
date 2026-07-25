use std::path::PathBuf;


#[derive(Debug, Default)]
pub struct ScanProgress {
    pub files_scanned: usize,
    pub folders_scanned: usize,
    pub current_path:PathBuf
}