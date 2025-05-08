use clipboard_rs::{Clipboard, ClipboardContext};
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use tauri::{Listener, Manager};

use crate::{
    commands::toggle_window::toggle_window,
    core::{
        app_handle::app_handle,
        database_api::{delete_item_by_id, get_all_ids, get_item_by_id, get_truncated_item_by_id},
        tasks::clipboard_watcher::{LAST_IMAGE, LAST_TEXT},
    },
    structures::{
        clipboard_entry::ClipboardEntry,
        config::{config, AppConfig},
    },
};

#[tauri::command]
pub fn get_clipboard_entries_ids() -> Vec<i64> {
    // Get all clipboard entry IDs from the database API
    return get_all_ids();
}

#[tauri::command]
pub fn get_clipboard_entry(id: i64) -> ClipboardEntry {
    // Get the clipboard entry from the database API using the provided ID
    return get_truncated_item_by_id(id).unwrap();
}

// Function to push a clipboard entry to the clipboard
#[tauri::command]
pub fn push_to_clipboard(id: i64) {
    // Get the clipboard from the app handle
    let app = app_handle();
    let clipboard = app.state::<tauri_plugin_clipboard::Clipboard>();

    // Get the configuration
    let configuration: AppConfig = config();

    // Get the full entry from the database
    let entry = get_item_by_id(id).unwrap();

    // If the clipboard history shouldn't be rewritten, set the current clipboard state to avoid duplicates
    if !configuration.window_rewrite_history_on_copy {
        if entry.entry_type == "image" {
            LAST_IMAGE.lock().unwrap().clone_from(&entry.content);
        } else {
            LAST_TEXT.lock().unwrap().clone_from(&entry.content);
        }
    }

    // If the entry is an image, write it to the clipboard as binary
    if entry.entry_type == "image" {
        let _ = clipboard.write_image_base64(entry.content);
    } else if entry.entry_type == "rich_text" {
        // If the entry is rich text, write it as RTF
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&entry.content) {
            let rtf = json["rtf"].as_str().unwrap_or_default().to_string();
            let plain = json["plain"].as_str().unwrap_or_default().to_string();

            // On windows
            #[cfg(target_os = "windows")]
            {
                // Open the clipboard
                let clip = ClipboardContext::new().expect("Failed to open clipboard");

                // Send the plain text
                clip.set_text(plain)
                    .expect("Failed to write plain text to clipboard");

                // Send the RTF data
                clip.set_rich_text(rtf)
                    .expect("Failed to write RTF to clipboard");

                println!("RTF data written to clipboard.");
            }

            // On macOS and Linux, write the RTF to the clipboard
            #[cfg(not(target_os = "windows"))]
            {
                let _ = clipboard.write_rtf(rtf);
            }
        } else {
            // If the RTF extraction fails, write it as text
            let _ = clipboard.write_text(entry.content);
        }
    } else {
        // Otherwise, write it as text
        let _ = clipboard.write_text(entry.content);
    }

    // If history should be rewritten, remove the old entry from the database
    if configuration.window_rewrite_history_on_copy {
        let _ = delete_item_by_id(id);
        println!("Deleted entry with ID: {}", id);
    }

    // If auto closing is enabled, close the window after a 700ms delay
    tauri::async_runtime::spawn(async move {
        if configuration.auto_hide_on_copy {
            // Compute the delay and sleep
            let delay = std::time::Duration::from_millis(700);
            tokio::time::sleep(delay).await;

            // Hide the window
            tauri::async_runtime::spawn(toggle_window(Some(false)));

            // If auto pasting is enabled, trigger the paste action once the clipboard is set and the window is hidden
            if configuration.auto_paste_on_copy {
                tauri::async_runtime::spawn(async move {
                    // Wait for the signal
                    let app = app_handle();
                    let _ = app.once("paste", handler);
                });
            }
        }
    });

    // If auto pasting is enabled, trigger the paste action once the clipboard is set and the window is hidden
    if configuration.auto_paste_on_copy {
        tauri::async_runtime::spawn(async move {
            // Wait for the signal
            let app = app_handle();
            let _ = app.once("paste", handler);
        });
    }

    return;
}

// Handler function to paste the content
fn handler(_: tauri::Event) {
    // Paste the content
    paste();
}

// Function to trigger the paste action
fn paste() {
    let enigo_settings = Settings::default();
    let mut enigo = Enigo::new(&enigo_settings).unwrap();
    #[cfg(target_os = "macos")]
    {
        // Simule un paste sur macOS avec Cmd+V
        enigo.key_down(Key::Meta); // Cmd
        enigo.key_click(Key::Layout('v')); // V
        enigo.key_up(Key::Meta); // Relâche Cmd
    }
    #[cfg(not(target_os = "macos"))]
    {
        // Simule un paste sur Windows et Linux avec Ctrl+V
        let _ = enigo.key(Key::Control, Press); // Ctrl
        let _ = enigo.key(Key::V, Click); // V
        let _ = enigo.key(Key::Control, Release); // Relâche Ctrl
    }
}
