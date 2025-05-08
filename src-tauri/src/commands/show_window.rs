use tauri::Manager;

use crate::core::app_handle::app_handle;


#[tauri::command]
pub fn show_window() {
    // Get the app handle
    let app_handle = app_handle();
    let res = app_handle.get_webview_window("main").unwrap().show();
    if res.is_err() {
        println!("Failed to show window");
    } else {
        println!("Window shown successfully");
    }
    return;
}
