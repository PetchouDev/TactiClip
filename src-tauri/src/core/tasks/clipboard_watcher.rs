use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use regex::Regex;
use std::time::Duration;

use once_cell::sync::Lazy;
use rusqlite::Connection;
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::sleep;

use crate::core::database_api::{get_last_item_copied, insert_clipboard_entry};
use crate::structures::clipboard_entry::ClipboardEntry;

pub static LAST_TEXT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new())); // Clipboard watcher control
pub static LAST_IMAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new())); // Clipboard watcher control
pub static PUSHED_COPY: AtomicBool = AtomicBool::new(false); // Flag to indicate if the clipboard was pushed


// Compile a regex pattern to match color formats thread-safely)
static COLOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)^\s*(#(?:[0-9a-f]{3}|[0-9a-f]{6})\b|rgba?\(\s*\d{1,3}\s*,\s*\d{1,3}\s*,\s*\d{1,3}(?:\s*,\s*(?:0|1|0?\.\d+))?\s*\)|hsla?\(\s*\d{1,3}(?:\.\d+)?\s*,\s*\d{1,3}%\s*,\s*\d{1,3}%(?:\s*,\s*(?:0|1|0?\.\d+))?\s*\))\s*$"
    ).unwrap()
});

// Compile regex patterns to match email
static MAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap()
});

// Compile regex patterns to match URL
static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^[a-z][a-z0-9+\-.]*://[^\s]+$").unwrap()
});


// Function to watch the clipboard for changes
pub async fn watch_clipboard(app: AppHandle, conn_mutex: &Mutex<Connection>) {
    let clipboard = app.state::<tauri_plugin_clipboard::Clipboard>();

    // Initialisation des derniers éléments copiés
    if let Some(last_image_entry) = get_last_item_copied("image") {
        LAST_IMAGE
            .lock()
            .unwrap()
            .clone_from(&last_image_entry.content);
    }
    if let Some(last_text_entry) = get_last_item_copied("text") {
        LAST_TEXT
            .lock()
            .unwrap()
            .clone_from(&last_text_entry.content);
    }

    loop {
        // Prevent the watcher from catching an elment that has just been pushed from the app
        if PUSHED_COPY.load(Ordering::Relaxed) {
            sleep(Duration::from_millis(20)).await;
            continue;
        }

        // Lecture du texte
        if let Ok(new_text) = clipboard.read_text() {
            if new_text != *LAST_TEXT.lock().unwrap() {

                // RICH TEXT support (Too experimental for now, only work an application to itself - Word to Word and
                // I can get the fallback to plain text to work)

                /* if clipboard.has_rtf().unwrap_or(false) {
                    if let Ok(rtf) = clipboard.read_rtf() {
                        let combined = serde_json::json!({
                            "plain": new_text,
                            "rtf": rtf
                        });
                        stored_content = combined.to_string();
                        format = "rich_text";
                    }
                } */


                // Check if the text is a color format
                let trimmed = new_text.trim_matches(|c: char| c.is_control() || c.is_whitespace());
                let is_color = COLOR_REGEX.is_match(trimmed);
                let is_email = MAIL_REGEX.is_match(trimmed);
                let is_url = URL_REGEX.is_match(trimmed);
                let format = if is_color {
                    "color"
                } else if is_email {
                    "email"
                } else if is_url {
                    "url"
                } else {
                    "text"
                };

                let clipboard_text = if format == "text" {new_text.clone()} else { trimmed.to_string() };

                *LAST_TEXT.lock().unwrap() = new_text.clone();
                let id = insert_clipboard_entry(format, &clipboard_text, 0);

                let conn = conn_mutex.lock().unwrap();
                if let Ok(row) = conn.query_row(
                    "SELECT id, type, content, added_at, pinned FROM clipboard_entries WHERE id = ?1",
                    (id,),
                    |row| {
                        Ok(ClipboardEntry {
                            id: row.get(0)?,
                            entry_type: row.get(1)?,
                            content: row.get(2)?,
                            added_at: row.get::<_, String>(3)?,
                            pinned: row.get::<_, i32>(4)? != 0,
                            forced_language: row.get::<_, Option<String>>(5).unwrap_or(None),
                        })
                    },
                ) {
                    let truncated = if row.content.len() > 250 {
                        format!("{}...", &row.content[..250])
                    } else {
                        row.content.clone()
                    };
                    let row = ClipboardEntry {
                        content: truncated,
                        ..row
                    };
                    let _ = app.emit("new-clipboard-item", row);
                }
            }
        }

        // Lecture de l'image en base64
        if let Ok(new_image_base64) = clipboard.read_image_base64() {
            if new_image_base64 != LAST_IMAGE.lock().unwrap().clone() {
                LAST_IMAGE.lock().unwrap().clone_from(&new_image_base64);

                let id = insert_clipboard_entry("image", &new_image_base64, 0);

                let conn = conn_mutex.lock().unwrap();
                if let Ok(row) = conn.query_row(
                    "SELECT id, type, content, added_at, pinned FROM clipboard_entries WHERE id = ?1",
                    (id,),
                    |row| {
                        Ok(ClipboardEntry {
                            id: row.get(0)?,
                            entry_type: row.get(1)?,
                            content: row.get(2)?,
                            added_at: row.get::<_, String>(3)?,
                            pinned: row.get::<_, i32>(4)? != 0,
                            forced_language: row.get::<_, Option<String>>(5).unwrap_or(None),
                        })
                    },
                ) {
                    let _ = app.emit("new-clipboard-item", row);
                }
            }
        }

        sleep(Duration::from_millis(20)).await;
    }
}
