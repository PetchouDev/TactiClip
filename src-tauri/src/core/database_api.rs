use std::{fs, path::PathBuf, sync::Mutex};

use directories::BaseDirs;
use once_cell::sync::OnceCell;
use rusqlite::Connection;

use crate::structures::{
    clipboard_entry::ClipboardEntry,
    config::{config, AppConfig},
};

pub static DATABASE_CONNECTION: OnceCell<Mutex<Connection>> = OnceCell::new(); // Database connection

// Function to get the database path based on the operating system
fn get_db_path() -> PathBuf {
    let base_dirs = BaseDirs::new().expect("Unable to access directories");

    // Détermine l'emplacement basé sur le système d'exploitation
    if cfg!(target_os = "windows") {
        // Sur Windows, utiliser AppData
        base_dirs
            .data_local_dir()
            .join("PetchouSoftware")
            .join("TactiClip")
            .join("database.db")
    } else {
        // Sur macOS et Linux, utiliser .local
        base_dirs
            .home_dir()
            .join(".local")
            .join("PetchouSoftware")
            .join("TactiClip")
            .join("database.db")
    }
}

// Function to initialize the database
pub fn init_db() {
    // Get the database path
    let db_path = get_db_path();
    print!("DB path: {:?}", db_path);

    // Create all directories leading to the database file
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directories");
    }

    // Open the database connection
    let conn = Connection::open(db_path).expect("Failed to open DB");

    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT NOT NULL,
            content TEXT NOT NULL,
            added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            pinned INTEGER NOT NULL DEFAULT 0,
            forced_language TEXT DEFAULT NULL
        )",
        [],
    )
    .expect("Failed to create table");

    let columns = vec![
        "type TEXT NOT NULL",
        "content TEXT NOT NULL",
        "added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP",
        "pinned INTEGER NOT NULL DEFAULT 0",
        "forced_language TEXT DEFAULT NULL",
    ];

    {
        // Scoped block pour forcer le drop de stmt avant le move
        let query = "PRAGMA table_info(clipboard_entries)";
        let mut stmt = conn.prepare(query).expect("Failed to prepare statement");
        let column_info: Vec<String> = stmt
            .query_map([], |row| row.get::<_, String>(1))
            .expect("Failed to query columns")
            .filter_map(Result::ok)
            .collect();

        for column in columns {
            let column_name = column.split_whitespace().next().unwrap();
            if !column_info.contains(&column_name.to_string()) {
                let add_column_query =
                    format!("ALTER TABLE clipboard_entries ADD COLUMN {}", column);
                conn.execute(&add_column_query, [])
                    .expect("Failed to add missing column");
                println!("Added missing column: {}", column);
            }
        }
    } // <- stmt est droppé ici

    // Maintenant on peut move `conn`
    DATABASE_CONNECTION
        .set(Mutex::new(conn))
        .expect("Failed to set database connection");
}

// Function to get the last item copied of a specific type
pub fn get_last_item_copied(item_type: &str) -> Option<ClipboardEntry> {
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();

    let entry: ClipboardEntry = conn.query_row(
        "SELECT id, type, content, added_at, pinned, forced_language FROM clipboard_entries WHERE type = ?1 ORDER BY added_at DESC LIMIT 1",
        [item_type],
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
    ).ok()?;
    Some(entry)
}

// Function to insert a new clipboard entry into the database
pub fn insert_clipboard_entry(entry_type: &str, content: &str, pinned: i32) -> i64 {
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();
    let _res = conn
        .execute(
            "INSERT INTO clipboard_entries (type, content, pinned) VALUES (?1, ?2, ?3)",
            &[&entry_type, &content, &(*pinned.to_string())],
        )
        .expect("Failed to insert clipboard entry");

    // Return the ID of the inserted entry
    conn.last_insert_rowid()
}

// Function to get all clipboard entries from the database
pub fn get_all_ids() -> Vec<i64> {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB not initialized")
        .lock()
        .unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM clipboard_entries ORDER BY added_at DESC")
        .unwrap();
    let ids_iter = stmt.query_map([], |row| row.get(0)).unwrap();
    let ids: Vec<i64> = ids_iter.filter_map(Result::ok).collect();

    ids
}

// Function to get a clipboard entry by its ID and truncate the content if necessary
pub fn get_truncated_item_by_id(id: i64) -> Option<ClipboardEntry> {
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB not initialized")
        .lock()
        .unwrap();
    let configuration: AppConfig = config();

    let entry: ClipboardEntry = conn.query_row(
        "SELECT id, type, content, added_at, pinned, forced_language FROM clipboard_entries WHERE id = ?1",
        [id],
        |row| {
            Ok(ClipboardEntry {
                id: row.get(0)?,
                entry_type: row.get(1)?,
                content: row.get(2)?,
                added_at: row.get(3)?,
                pinned: row.get::<_, i32>(4)? != 0,
                forced_language: row.get::<_, Option<String>>(5).unwrap_or(None),
            })
        },
    ).ok().expect("Failed to get clipboard entry.");

    let mut content = entry.content.clone();

    // Si c'est du rich_text, on extrait le plain uniquement
    if entry.entry_type == "rich_text" {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&entry.content) {
            content = json["plain"].as_str().unwrap_or_default().to_string();
        }
    }

    // Troncature si texte non image
    if entry.entry_type != "image"
        && content.len() > configuration.max_displayed_characters as usize
    {
        content = content
            .chars()
            .take(configuration.max_displayed_characters as usize)
            .collect::<String>()
            + "...";
    }

    Some(ClipboardEntry { content, ..entry })
}

// Function to get a clipboard entry by its ID
pub fn get_item_by_id(id: i64) -> Option<ClipboardEntry> {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();
    let entry: ClipboardEntry = conn.query_row(
        "SELECT id, type, content, added_at, pinned, forced_language FROM clipboard_entries WHERE id = ?1",
        [id],
        |row| {
            Ok(ClipboardEntry {
                id: row.get(0)?,
                entry_type: row.get(1)?,
                content: row.get(2)?,
                added_at: row.get(3)?,
                pinned: row.get::<_, i32>(4)? != 0,
                forced_language: row.get::<_, Option<String>>(5).unwrap_or(None),
            })
        },
    ).ok()?;
    Some(entry)
}

// Function to delete a clipboard entry by its ID
pub fn delete_item_by_id(id: i64) {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();
    let _res = conn
        .execute("DELETE FROM clipboard_entries WHERE id = ?1", [id])
        .unwrap();
}

// Function to delete all clipboard entries
pub fn delete_all_items() -> i64 {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();

    let res = conn.execute("DELETE FROM clipboard_entries", []).unwrap();
    if res == 0 {
        println!("Failed to delete all clipboard entries");
    }

    println!("Deleted {} clipboard entries", res);

    // Get all IDs of deleted entries
    res as i64
}

// Function to pin an entry by its ID
pub fn pin_item_by_id(id: i64) {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();
    let _res = conn
        .execute(
            "UPDATE clipboard_entries SET pinned = 1 WHERE id = ?1",
            [id],
        )
        .unwrap();
}

// Function to unpin an entry by its ID
pub fn unpin_item_by_id(id: i64) {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();
    let _res = conn
        .execute(
            "UPDATE clipboard_entries SET pinned = 0 WHERE id = ?1",
            [id],
        )
        .unwrap();
}

// Function to unpin all entries
pub fn unpin_all_items() -> i64 {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();
    let res = conn
        .execute("UPDATE clipboard_entries SET pinned = 0", [])
        .unwrap();

    // Return the number of rows affected
    res as i64
}

// Function to set the forced language of an entry by its ID
pub fn set_forced_language(id: i64, language: &str) {
    // Get the database connection
    let conn = DATABASE_CONNECTION
        .get()
        .expect("DB non initialisée")
        .lock()
        .unwrap();

    // Convert the id to a string
    let id = id.to_string();

    let res = conn
        .execute(
            "UPDATE clipboard_entries SET forced_language = ?1 WHERE id = ?2",
            [language, &id],
        )
        .unwrap();
    if res == 0 {
        println!(
            "Failed to set forced language for clipboard entry with ID: {}",
            id
        );
    }
}
