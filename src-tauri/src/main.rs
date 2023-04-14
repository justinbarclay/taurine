// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
  fs,
  path::{Path, PathBuf},
};

use walkdir::WalkDir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
fn search_file(location: Option<&str>, guess: &str) -> Vec<String> {
  if let Some(path) = location {
    let texas_ranger = WalkDir::new(Path::new(path)).follow_links(true);
    let files: Vec<String> = texas_ranger
      .into_iter()
      .map(|entry| {
        fs::canonicalize(entry.unwrap().path())
          .expect("Unable to read as file")
          .into_os_string()
          .into_string()
          .ok()
          .unwrap_or("".to_string())
      })
      .filter(|file| !file.is_empty())
      .filter(|file| file.contains(guess))
      .collect();
    files
  } else {
    vec![]
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
