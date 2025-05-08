use tauri::Manager;

use crate::{
    core::app_handle::app_handle, 
    structures::config::{
        config,
        AppConfig
    },
    commands::toggle_window::slide_window
};


#[tauri::command]
pub fn resize_window() {
    let app_handle = app_handle();
    let configuration: AppConfig = config();

    if let Ok(Some(monitor)) = app_handle.primary_monitor() {
        let size = monitor.size();
        let window = app_handle.get_webview_window("main").unwrap();
        let _ = window.set_skip_taskbar(true).unwrap();
        let _ = window.set_visible_on_all_workspaces(true).unwrap();

        if configuration.window_position == "bottom" || configuration.window_position == "top" {
            let width = (((size.width as i32) - configuration.window_padding_x.abs() * 2) as f64
                * configuration.window_primary_factor) as u32;

            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width,
                height: configuration.window_secondary_size as u32,
            }));

            let x = (size.width as f64 * (1.0 - configuration.window_primary_factor) / 2.0) as i32
                + configuration.window_padding_x;

            let y = if configuration.window_position == "bottom" {
                size.height as i32
                    - configuration.window_secondary_size
                    - configuration.window_padding_y
            } else {
                configuration.window_padding_y
            };

            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x,
                y,
            }));
        } else {
            let height = ((size.height as i32 - configuration.window_padding_y.abs() * 2) as f64
                * configuration.window_primary_factor) as u32;

            let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                width: configuration.window_secondary_size as u32,
                height,
            }));

            let y = (size.height as f64 * (1.0 - configuration.window_primary_factor) / 2.0) as i32
                + configuration.window_padding_y;

            let x = if configuration.window_position == "left" {
                configuration.window_padding_x
            } else {
                size.width as i32
                    - configuration.window_secondary_size
                    - configuration.window_padding_x
            };

            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x,
                y,
            }));
        }

        tauri::async_runtime::spawn(async move {
            slide_window(false).await;
        });
    } else {
        eprintln!("Unable to get primary monitor");
    }
}