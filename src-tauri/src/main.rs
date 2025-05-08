// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


/*  __  __  ___  ____  ____   */
/* |  \/  |/ _ \|  _ \/ ___|  */
/* | |\/| | | | | | | \___ \  */
/* | |  | | |_| | |_| |___) | */
/* |_|  |_|\___/|____/|____/  */


mod structures;
mod core;
mod commands;


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

use core::{
    app_handle::{
        app_handle, 
        APP_HANDLE
    }, 
    database_api::{
        init_db, 
        DATABASE_CONNECTION
    }, 
    tasks::{
        clipboard_watcher::watch_clipboard, 
        hotkeys_listener::spawn_hotkey_listener
    }
};
use structures::config::config;
use commands::{
    clipboard_api::{
        get_clipboard_entries_ids, 
        get_clipboard_entry, 
        push_to_clipboard
    }, delete_item::{
        delete_all, 
        delete_item
    }, 
    pin_item::{
        toggle_pin, 
        unpin_all
    }, 
    resize_window::resize_window, 
    show_window::show_window, 
    toggle_window::{
        toggle_window,
        slide_window
    },
    force_language::force_language,
    settings_api::{
        open_settings,
        get_config_value,
        reset_config,
        save_config,
        preview_config,
        cancel_config
    }
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
        .plugin(tauri_plugin_clipboard::init())
        .setup(|app| {
          // Create a system tray icon and menu
          let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
          let prefs = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
          let tray_menu = Menu::with_items(app, &[&prefs, &quit])?;
          let _tray = TrayIconBuilder::new()
              .title("TactiClip")
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
                  _ => {
                      println!("menu item {:?} not handled", event.id);
                  }
              })
              .build(app)?;
      
          let handle = app.handle().clone();
          APP_HANDLE.set(handle.to_owned()).unwrap();
          let db_conn = DATABASE_CONNECTION.get().expect("DB non initialisée");

          // Spwawn the window after 500 ms to hide the webview loading (white screen)
          tauri::async_runtime::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let app = app_handle();
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
            show_window,              // Register exposed functions
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // app_lib::run();
}
