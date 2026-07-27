use crate::filesystem::node::FileNode;


//&[FileNode]   we are borrowing only
pub fn search(node:&[FileNode], query:&str) -> Vec<FileNode> {
    let mut results:Vec<FileNode> = Vec::new();

    for node in nodes {
        if node.name.to_lowercase().contains(&query.to_lowercase()) {
            results.extend(node.clone());
        }
        if node.is_dir {
           let child_results = search(&node.children, query);
           results.extend(child_results);
        }
    }
    
    results
}