use crate::{get_ai::get_ai_text, get_commit_log::get_log};
use std::thread;
use tauri::Window;

#[tauri::command]
pub fn save_path(path: Vec<String>, current_date: [String; 2]) -> Vec<String> {
    // format!("Hello, {}! You've been greeted from Rust!", path)
    get_log(path, current_date)
}

#[tauri::command]
pub async fn get_ai_content(window: Window, c: String) {
    let join_handle = thread::spawn(|| async {
        if let Err(e) = get_ai_text(window, c).await {
            println!("{:?}", e);
        };
    });
    let _ = join_handle.join().unwrap().await;
}
