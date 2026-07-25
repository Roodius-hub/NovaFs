

#[derive(Debug, Clone)]
pub enum ScanEvent {
    Started,
    Progress,
    Completed, 
    Cancelled,
    Error,
}