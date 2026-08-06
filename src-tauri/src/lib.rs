// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::path::Path;
use crate::{filesystem::{events::{ScanEvent, WatchEvent}, operations::{create_file, delete}, scanner::scan, watcher::watch}};

pub mod db;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;
pub mod tests;
pub mod filesystem;
pub mod app;


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
      
      // let result = filesystem::filter::filter(&tree, &Filter::Extension("json".to_string()));

      // println!("{}", "yout filtered files: => ".blue());
      // println!("{:?}", result);


      let size = filesystem::size::size(&tree);
      let size_in_string = filesystem::size::format_size(size);
      println!("size: => {}", size_in_string);

      let meta = filesystem::metadata::get_metadata(Path::new("."));
      for val in meta.iter() {
          println!("{:?}", val);
      }

      let makefile = Path::new("/home/roodi/Documents/hello.txt");
      
      match filesystem::operations::delete(makefile) {
          Ok(()) => println!("File deleted successfully."),
          Err(err) => println!("Failed to delete file: {}", err),
      }
      
      let emit = |event: WatchEvent| {
          println!("{:?}", event);
      };
      
      watch(Path::new("."), emit);
      
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
