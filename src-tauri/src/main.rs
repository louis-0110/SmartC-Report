// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_ai;
mod get_commit_log;
mod open_files;
mod parser_xml;
mod read_file;
use std::{path::PathBuf, sync::OnceLock};
use tauri::api::path;

pub(crate) static CONFIG_DIR: std::sync::OnceLock<PathBuf> = OnceLock::new();
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let _ = CONFIG_DIR.set(path::app_config_dir(&app.config()).unwrap());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_files::save_path,
            open_files::get_ai_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
