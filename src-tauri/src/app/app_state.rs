

use std::{io, path::{PathBuf}};

use crate::filesystem::{self, events::ScanEvent, filter::{Filter, filter}, node::FileNode, search::search, size::size};

pub struct AppState {
    pub root: PathBuf,
    pub tree: Vec<FileNode>,
}

impl AppState {
    pub fn new(root:PathBuf) -> Self {
        AppState { root, tree: Vec::new() }
    }

    // scan AppState
    pub fn scan<F>(&mut self, mut emit:F) -> io::Result<()> where F:FnMut(ScanEvent) {
        let tree = filesystem::scanner::scan(&self.root, &mut emit)?;
        self.tree = tree;
        Ok(())
    }

    // refresh current root
    pub fn refresh<F>(
        &mut self,
        emit: F,
    ) -> io::Result<()>
    where
        F: FnMut(ScanEvent),
    {
        self.scan(emit)
    }

    // search
    pub fn search(&self, query:&str) -> Vec<FileNode> {
        search(&self.tree, query)
    }

    // filter
    pub fn filter(&self, fitler:&Filter) -> Vec<FileNode> {
        filter(&self.tree, fitler)
    }

    // size
    pub fn size(&self) -> u64 {
        size(&self.tree)
    }
    
}