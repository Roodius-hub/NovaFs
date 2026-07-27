// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::path::Path;

use crate::filesystem::{events::ScanEvent, node::FileNode, scanner::scan};

pub mod db;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;
pub mod tests;
pub mod filesystem;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn check_file(
    path: String,
    query: String,
) -> Result<Vec<FileNode>, String> {
    let mut emit = |event: ScanEvent| {
        println!("{:?}", event);
    };

    // Scan the directory only once
    let tree = scan(Path::new(&path), &mut emit)
        .map_err(|err| err.to_string())?;

    // Search inside the scanned tree
    let results = filesystem::search::search(&tree, &query);

    Ok(results)
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
     let  conn = db::connection::connect().unwrap();
     println!("Database Connected");
     db::migrations::migrate(&conn).unwrap();
     
     tests::favorite_test::test_repository(); 
      // let tree = tests::scanner_test::test_scanner();


    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, check_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
