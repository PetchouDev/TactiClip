use std::sync::OnceLock;

use tauri::AppHandle;


pub static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new(); // App handle

pub // Get the app handle from anywhere in the application
fn app_handle<'a>() -> &'a AppHandle {
  APP_HANDLE.get().unwrap()
}