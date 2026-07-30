// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::path::Path;
use colored_text::Colorize;
use crate::filesystem::{events::ScanEvent, filter::Filter, scanner::scan};

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

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
     let  conn = db::connection::connect().unwrap();
     println!("Database Connected");
     db::migrations::migrate(&conn).unwrap();
     
     // tests::favorite_test::test_repository(); 
      // let tree = tests::scanner_test::test_scanner();

      let mut emit = |event: ScanEvent| {
          println!("{:?}", event);
      };
      
      let tree = scan(Path::new("."), &mut emit).unwrap();
      
      // let results = filesystem::search::search(&tree, "main");
      // println!("{}","This is Your result".red());
      // println!("{:?}", results);
      
      let result = filesystem::filter::filter(&tree, &Filter::Extension("json".to_string()));

      println!("{}", "yout filtered files: => ".blue());
      println!("{:?}", result);
      
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
