use tauri::Emitter;

use crate::core::{
    app_handle::app_handle,
    database_api::{delete_all_items, delete_item_by_id},
};

#[tauri::command]
pub fn delete_item(id: i64) {
    println!("Deleting item with ID: {}", id);
    let res = delete_item_by_id(id);
    let _ = app_handle().emit("delete-item", id);

    return res;
}

#[tauri::command]
pub fn delete_all() {
    println!("Deleting all items");
    let _ = delete_all_items();

    let app = app_handle();

    let _ = app.emit("delete-all-items", {}).unwrap();
}
