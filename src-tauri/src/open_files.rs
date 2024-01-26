use crate::{get_ai::get_ai_text, get_commit_log::get_log};
use serde_json::json;
use std::thread;
use tauri::Window;

#[tauri::command]
pub fn save_path(window: Window, path: Vec<String>, current_date: [String; 2]) -> Vec<String> {
    // format!("Hello, {}! You've been greeted from Rust!", path)
    let _window = window.clone();
    if let Ok(v) = get_log(window, path, current_date) {
        let _ = _window.emit("error", json!(v));
        return v;
    } else {
        return vec![];
    }
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
