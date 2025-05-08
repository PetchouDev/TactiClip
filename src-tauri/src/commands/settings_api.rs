use tauri::{Listener, Manager, Result, WebviewWindowBuilder};

use crate::{
    commands::toggle_window::toggle_window,
    core::app_handle::app_handle,
    structures::config::{config, get_config_path, AppConfig},
};

fn reload_main() {
    // Get the app handle
    let app = app_handle();

    // Get the main window
    let w = app.get_window("main").unwrap();

    // Reload the main window
    w.webviews()[0].reload().unwrap();

    // Listen for the end of the sliding animation to trigger it back so th window becomes visible again
    app.once("paste", move |_| {
        tauri::async_runtime::spawn(toggle_window(Some(true)));
    });
}

#[tauri::command]
pub async fn open_settings() {
    println!("Opening settings window");

    let handle = app_handle().clone();

    tauri::async_runtime::spawn_blocking(move || {
        if let Some(window) = handle.get_webview_window("settings") {
            let _ = window.show();
            let _ = window.set_focus();
            return;
        }

        let win = WebviewWindowBuilder::new(
            &handle,
            "settings",
            tauri::WebviewUrl::App("/settings".parse().unwrap()),
        )
        .inner_size(650.0, 700.0)
        .center()
        .resizable(false)
        .maximizable(false)
        .build()
        .expect("failed to create settings window");

        let _ = win.show();
        let _ = win.set_focus();
    })
    .await
    .unwrap();

    let _ = toggle_window(Some(false)).await;
}

#[tauri::command]
pub fn get_config_value(property: Option<&str>) -> Option<String> {
    let config = config();
    if property.is_none() {
        return Some(serde_json::to_string(&config).unwrap());
    }
    let property = property.unwrap();
    match property {
        "window_position" => Some(config.window_position.clone()),
        "window_primary_factor" => Some(config.window_primary_factor.to_string()),
        "window_secondary_size" => Some(config.window_secondary_size.to_string()),
        "window_padding_x" => Some(config.window_padding_x.to_string()),
        "window_padding_y" => Some(config.window_padding_y.to_string()),
        "window_animation_duration" => Some(config.window_animation_duration.to_string()),
        "window_steps" => Some(config.window_steps.to_string()),
        "window_rewrite_history_on_copy" => Some(config.window_rewrite_history_on_copy.to_string()),
        "reset_scroll_on_show" => Some(config.reset_scroll_on_show.to_string()),
        "scroll_factor" => Some(config.scroll_factor.to_string()),
        "smooth_scroll" => Some(config.smooth_scroll.to_string()),
        _ => Some(serde_json::to_string(&config).unwrap()),
    }
}

// Reset the config to default values
#[tauri::command]
pub fn reset_config() -> Result<()> {
    // Get the default config as a JSON string
    let default = AppConfig::default();
    let default_json = serde_json::to_string(&default).unwrap();

    // Get the current config
    let mut config = config();

    // Write it instead of the current config (in memory only)
    config.update_from_json(&default_json).unwrap();

    // Reload the main window
    reload_main();

    // Reload the settings window
    if let Some(window) = app_handle().get_window("settings") {
        window.webviews()[0].reload()?;
    }

    Ok(())
}

#[tauri::command]
pub fn preview_config(payload: String) {
    // Get the current config
    let mut config = config();

    // Update the config with the new values
    let _ = config.update_from_json(&payload);

    // Reload the main window with the new config
    reload_main();
}

#[tauri::command]
pub fn cancel_config(app: tauri::AppHandle) -> Result<()> {
    // Reload the config from the file
    config();

    // Reload the main window
    app.get_window("main").unwrap().webviews()[0].reload()?;

    // Close the settings window
    if let Some(window) = app.get_window("settings") {
        window.close().unwrap();
    }

    Ok(())
}

#[tauri::command]
pub fn save_config(payload: String) {
    // Get the current config
    let mut config = config();

    // Update the config with the new values
    config.update_from_json(&payload).unwrap();

    // Write the config to the file
    let path = get_config_path();
    config.save_to_file(path.to_str().unwrap()).unwrap();

    // Reload the main window with the new config
    reload_main();
}
