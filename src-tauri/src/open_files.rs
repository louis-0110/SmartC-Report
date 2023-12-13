use std::thread;

use crate::{get_ai::get_ai_text, get_commit_log::get_log};

#[tauri::command]
pub fn save_path(path: Vec<String>, current_date: String) -> Vec<String> {
    // format!("Hello, {}! You've been greeted from Rust!", path)
    get_log(path, current_date)
}

#[tauri::command]
pub fn get_ai_content(c: String) -> String {
    let resulter = thread::spawn(|| match get_ai_text(c) {
        Ok(s) => s,
        Err(e) => format!("Error: {}", e),
    });
    resulter.join().unwrap()
}
