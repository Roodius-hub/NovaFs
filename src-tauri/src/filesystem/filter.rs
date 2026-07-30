
use tauri::webview::cookie::time::ext;

use crate::filesystem::node::FileNode;

pub enum Filter {
    Directory,
    File,
    Extension(String),
}

pub fn filter(nodes:&[FileNode],filters: &Filter) -> Vec<FileNode> {
    let mut result:Vec<FileNode> = Vec::new();

    for node in nodes {
        match filters {
            Filter::Directory =>  {if node.is_dir {result.push(node.clone())}},
            Filter::File => {
                if !node.is_dir {
                    result.push(node.clone());
                }
            },
            Filter::Extension(ext) => {
                if node.extension.as_deref() ==  Some(&ext.as_str()) {
                    result.push(node.clone());
                }
            }
        }

        if node.is_dir {
            let child_results = filter(&node.children, &filters);
            result.extend(child_results)
        }
    }

    result
}