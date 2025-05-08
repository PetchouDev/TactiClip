use std::{sync::atomic::{AtomicBool, Ordering}, time::Duration};

use once_cell::sync::Lazy;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::sleep;

use crate::{
    core::app_handle::app_handle,
    structures::config::{
        config, AppConfig
    }
};


// Global variable to track the visibility state of the window
pub static VISIBLE: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false)); // State of the window visibility


// Function to slide the window up or down with an animation
pub async fn slide_window(visible: bool) {
    let app_handle: &AppHandle = app_handle();
    let configuration: AppConfig = config();

    if let Ok(Some(monitor)) = app_handle.primary_monitor() {
        let window = match app_handle.get_webview_window("main") {
            Some(w) => w,
            None => {
                eprintln!("Window not found");
                return;
            }
        };

        let size = monitor.size();
        let padding_x = configuration.window_padding_x;
        let padding_y = configuration.window_padding_y;
        let width = size.width;
        let height = size.height;

        let (x, y);
        let (target_x, target_y);

        match configuration.window_position.as_str() {
            "top" => {
                x = ((width as f64 * (1.0 - configuration.window_primary_factor)) / 2.0) as i32 + padding_x;
                y = if visible {
                    padding_y
                } else {
                    -configuration.window_secondary_size - padding_y
                };
                target_x = x;
                target_y = y;
            }
            "bottom" => {
                x = ((width as f64 * (1.0 - configuration.window_primary_factor)) / 2.0) as i32 + padding_x;
                y = if visible {
                    height as i32 - configuration.window_secondary_size - padding_y
                } else {
                    height as i32 + padding_y
                };
                target_x = x;
                target_y = y;
            }
            "left" => {
                x = if visible {
                    padding_x
                } else {
                    -configuration.window_secondary_size - padding_x
                };
                y = ((height as f64 * (1.0 - configuration.window_primary_factor)) / 2.0) as i32 + padding_y;
                target_x = x;
                target_y = y;
            }
            "right" => {
                x = if visible {
                    width as i32 - configuration.window_secondary_size - padding_x
                } else {
                    width as i32 + padding_x
                };
                y = ((height as f64 * (1.0 - configuration.window_primary_factor)) / 2.0) as i32 + padding_y;
                target_x = x;
                target_y = y;
            }
            _ => {
                eprintln!("Invalid window position");
                return;
            }
        }

        let current_pos = match window.outer_position() {
            Ok(pos) => pos,
            Err(_) => return,
        };

        if current_pos.x == target_x && current_pos.y == target_y {
            return;
        }

        let delta_x = target_x - current_pos.x;
        let delta_y = target_y - current_pos.y;
        let delay = Duration::from_millis(configuration.window_animation_duration / configuration.window_steps);        // If the scroll should be reset on show, reset it
        if configuration.reset_scroll_on_show {
            // Get the application from the app handle
            let _ = app_handle.emit("reset-scroll", {});
        }

        tokio::spawn(async move {
            // Show the window if it is not visible
            if visible {
                window.show().unwrap();
            }
            for step in 0..=configuration.window_steps {
                let t = step as f64 / configuration.window_steps as f64;
                let eased = if visible {
                    1.0 - (1.0 - t).powi(configuration.ease_factor)
                } else {
                    t.powi(configuration.ease_factor)
                };
                let new_x = current_pos.x + (delta_x as f64 * eased) as i32;
                let new_y = current_pos.y + (delta_y as f64 * eased) as i32;
                let _ = window.set_position(
                    tauri::Position::Physical(tauri::PhysicalPosition { x: new_x, y: new_y })
                );
                sleep(delay).await;
            }
            if !visible {
                window.hide().unwrap();
                app_handle.emit("paste", {}).unwrap();
            }
        });
    } else {
        eprintln!("No monitor found.");
    }
}

// Function to toggle the window visibility (mostly used by the hotkey listener, bu thought I could find a use for this from the UI)
#[tauri::command]
pub async fn toggle_window(target_visibility: Option<bool>) {
    let current_state = VISIBLE.load(Ordering::Relaxed);

    let new_state = match target_visibility {
        Some(target) if target == current_state => return,
        Some(target) => target,
        None => !current_state,
    };

    VISIBLE.store(new_state, Ordering::Relaxed);
    slide_window(new_state).await;
}