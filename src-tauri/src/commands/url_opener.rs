use tauri_plugin_opener::OpenerExt;

use crate::core::app_handle::app_handle;
use crate::commands::toggle_window::toggle_window;


#[tauri::command]
pub async fn open_url(url: String) {
    // Get the app handle
    let handle = app_handle();
    
    // Open the URL in the default web browser
    let _ = handle.opener().open_url(url, None::<&str>);

    // Hide the window after opening the URL
    let _ = toggle_window(Some(false)).await;
}
