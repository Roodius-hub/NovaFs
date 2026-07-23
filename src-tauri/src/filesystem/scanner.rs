

use std::io;
use std::path::Path;

use crate::filesystem::node::Filenode;

pub fn scan_directoy(path: &path) -> io::Result<Vec<FileNode>> {
    let entries = std::fs::read_dir(path)?;
    
    Ok(Vec::new())
}