// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod get_ai;
mod get_commit_log;
mod open_files;
mod parser_xml;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            open_files::save_path,
            open_files::get_ai_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
