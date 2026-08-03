

use std::path::PathBuf;
use crate::filesystem::node::FileNode;

pub struct AppState {
    pub root: PathBuf,
    pub tree: Vec<FileNode>,
}