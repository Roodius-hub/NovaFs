

use crate::filesystem::progress::ScanProgress;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    Started,
    Progress(ScanProgress),
    Completed, 
    Cancelled,
    Error(String),
}