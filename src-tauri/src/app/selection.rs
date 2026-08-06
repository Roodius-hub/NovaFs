use std::path::{Path, PathBuf};


pub struct Selection {
    selected:Vec<PathBuf>
}

impl Selection {
    // new
    pub fn new() -> Self {
        Selection { selected: Vec::new() }
    }

    // selected
    pub fn select(&mut self, path:PathBuf) {
        self.selected.push(path);
    }

    pub fn deselect(&mut self, path: &Path) {
        self.selected.retain(|p| p != path);
    }
    
    pub fn clear(&mut self) {
        self.selected.clear();
    }
    
    pub fn is_selected(&self, path: &Path) -> bool {
        self.selected.iter().any(|p| p == path )
    }

    pub fn selected_item(&self) -> &[PathBuf] {
        &self.selected
    }
}

