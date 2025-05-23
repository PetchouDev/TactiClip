// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/*  __  __  ___  ____  ____   */
/* |  \/  |/ _ \|  _ \/ ___|  */
/* | |\/| | | | | | | \___ \  */
/* | |  | | |_| | |_| |___) | */
/* |_|  |_|\___/|____/|____/  */

mod commands;
mod core;
mod structures;

/*  ___ __  __ ____   ___  ____ _____ ____   */
/* |_ _|  \/  |  _ \ / _ \|  _ \_   _/ ___|  */
/*  | || |\/| | |_) | | | | |_) || | \___ \  */
/*  | || |  | |  __/| |_| |  _ < | |  ___) | */
/* |___|_|  |_|_|    \___/|_| \_\|_| |____/  */

use tauri::{
    menu::{
        Menu, 
        MenuItem
    },
    tray::TrayIconBuilder,
    Manager,
};

use tauri_plugin_opener::OpenerExt;

use commands::{
    clipboard_api::{get_clipboard_entries_ids, get_clipboard_entry, push_to_clipboard},
    delete_item::{delete_all, delete_item},
    force_language::force_language,
    pin_item::{toggle_pin, unpin_all},
    resize_window::resize_window,
    settings_api::{
        cancel_config, get_config_value, open_settings, preview_config, reset_config, save_config,
    },
    show_window::show_window,
    toggle_window::{slide_window, toggle_window},
    url_opener::open_url
};
use core::{
    app_handle::{app_handle as get_app_handle, APP_HANDLE},
    database_api::{init_db, DATABASE_CONNECTION},
    tasks::{clipboard_watcher::watch_clipboard, hotkeys_listener::spawn_hotkey_listener},
};
use structures::config::config;

// For windows, tools to disable/enable native clipboard history
#[cfg(windows)]
use commands::manage_native_clipboard::{
    disable_windows_clipboard_history, enable_windows_clipboard_history,
};

/*  _____ _   _ _____ ______   ______   ___ ___ _   _ _____  */
/* | ____| \ | |_   _|  _ \ \ / /  _ \ / _ \_ _| \ | |_   _| */
/* |  _| |  \| | | | | |_) \ V /| |_) | | | | ||  \| | | |   */
/* | |___| |\  | | | |  _ < | | |  __/| |_| | || |\  | | |   */
/* |_____|_| \_| |_| |_| \_\|_| |_|    \___/___|_| \_| |_|   */

fn main() {
    // Initialize the database connection
    init_db();

    // Load the configuration
    let _ = config();

    // Initialize the Tauri application with the specified configuration
    let _application = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();
            // Create a system tray icon and menu
            let quit = MenuItem::with_id(app, "quit", "🚪 Quit", true, None::<&str>)?;
            let prefs = MenuItem::with_id(app, "settings", "🔧 Settings", true, None::<&str>)?;
            let feature = MenuItem::with_id(app, "feature", "🚀 Request a feature", true, None::<&str>)?;
            let bug = MenuItem::with_id(app, "bug", "🐞 Report a bug", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&prefs, &feature, &bug, &quit])?;
            let _tray = TrayIconBuilder::new()
                .title("TactiClip")
                .tooltip("TactiClip")
                .show_menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(|parent_app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        // Close all tokio tasks

                        parent_app.exit(0);
                    }
                    "settings" => {
                        tauri::async_runtime::spawn(async move {
                            // Open the settings window
                            open_settings().await;
                            // Hide the main window
                            slide_window(false).await;
                        });
                        println!("Preferences clicked!");
                    }
                    "bug" => {
                        let url = "https://github.com/PetchouDev/TactiClip/issues/new?template=%F0%9F%90%9E-bug-report.md";
                        let _ = parent_app.opener().open_url(url, None::<&str>);
                    }
                    "feature" => {
                        let url = "https://github.com/PetchouDev/TactiClip/issues/new?template=%F0%9F%9A%80-feature-request.md";
                        let _ = parent_app.opener().open_url(url, None::<&str>);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            
            let handle = app_handle.clone();

            APP_HANDLE.set(handle.to_owned()).unwrap();
            let db_conn = DATABASE_CONNECTION.get().expect("DB non initialisée");

            // Spwawn the window after 500 ms to hide the webview loading (white screen)
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                let app = get_app_handle();
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
            });

            // Spawn the clipboard watcher
            tauri::async_runtime::spawn(async move {
                watch_clipboard(handle, db_conn).await;
            });

            // Spawn the hotkey listener
            tauri::async_runtime::spawn(async move {
                spawn_hotkey_listener(toggle_window).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_clipboard_entries_ids,
            get_clipboard_entry,
            show_window,
            resize_window,
            toggle_window,
            push_to_clipboard,
            get_config_value,
            delete_item,
            toggle_pin,
            unpin_all,
            delete_all,
            open_settings,
            force_language,
            reset_config,
            save_config,
            preview_config,
            cancel_config,
            open_url,
            #[cfg(windows)]
            disable_windows_clipboard_history,
            #[cfg(windows)]
            enable_windows_clipboard_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // app_lib::run();
}
