
use std::path::Path;
use crate::filesystem::{events::ScanEvent, scanner::scan};

pub fn test_scanner() {
    let mut emit = |event:ScanEvent| {
        println!("{:?}", event);
    };
    
    match scan(Path::new("."), &mut emit) {
        Ok(nodes) => println!("{:#?}", nodes),
        Err(err) => println!("Scan failed: {}",err),
    }
}