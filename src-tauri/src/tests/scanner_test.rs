
use std::path::Path;
use crate::filesystem::{node, scanner::scan_directory};

pub fn test_scanner() {
    let nodes = scan_directory(Path::new("."));
    
    
    while let Ok(node) = &nodes {
        println!("{:#?}", node);   
    }      
}