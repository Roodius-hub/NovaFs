

use std::path::PathBuf;

use crate::filesystem::progress::ScanProgress;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    Started,
    Progress(ScanProgress),
    Completed, 
    Cancelled,
    Error(String),
}

#[derive(Debug, Clone)]
pub enum WatchEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed {
        from: PathBuf,
        to: PathBuf,
    },
}

// #[derive(Debug, Clone)]
// pub enum FsEvent {
//     Scan(ScanEvent),
//     Watch(WatchEvent),
// }