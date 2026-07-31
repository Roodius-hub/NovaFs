use crate::filesystem::node::FileNode;


pub fn size(nodes:&[FileNode]) -> u64 {
    let mut totol_size = 0;

    for node in nodes {
        if node.is_dir {
            totol_size += size(&node.children);
        } else {
            totol_size += node.size;
        }
    }
    totol_size
}

pub fn format_size(bytes:u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;
        const TB: f64 = GB * 1024.0;

        let bytes = bytes as f64;
        
            if bytes >= TB {
                format!("{:.2} TB", bytes / TB)
            } else if bytes >= GB {
                format!("{:.2} GB", bytes / GB)
            } else if bytes >= MB {
                format!("{:.2} MB", bytes / MB)
            } else if bytes >= KB {
                format!("{:.2} KB", bytes / KB)
            } else {
                format!("{} B", bytes as u64)
            }

}