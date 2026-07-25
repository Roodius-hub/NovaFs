
use std::fs::DirEntry;
use std::io;
use std::path::Path;

use crate::filesystem::node::{FileNode};
use crate::filesystem::progress::ScanProgress;

pub fn scan_directory(path: &Path, progress:&mut ScanProgress) -> io::Result<Vec<FileNode>> {
    let entries = std::fs::read_dir(path)?;
    let mut nodes = Vec::new();
    
    for entry in entries {
        progress.folders_scanned += 1;
        let mut node = scan_entry(entry?)?;
        if node.is_dir {
             match scan_directory(&node.path, progress) {
                Ok(children) => node.children = children,
                Err(err) => {
                    eprintln!("Skipping {}: {}", node.path.display(), err);
                }
            } 
        } else {
            progress.files_scanned += 1;
        }
        nodes.push(node);
    }
    Ok(nodes)
}

pub fn scan_entry(entry:DirEntry) -> io::Result<FileNode> {
    let entry = entry;
    let name = entry.file_name().to_string_lossy().to_string();
    let path = entry.path();
    let is_dir = entry.file_type()?.is_dir();
    let metadata = entry.metadata()?;
    let size = metadata.len();
    let extension = path
        .extension()
        .map(|ext| ext.to_string_lossy().to_string());
    let created_at = metadata.created().ok();
    let modified_at = metadata.modified().ok();

    let node = FileNode {
        name,
          path,
          is_dir,
          extension,
          size,
          created_at,
          modified_at,
          children:Vec::new()
    }; 

    Ok(node)
}
