use crate::core::database_api::{pin_item_by_id, unpin_all_items, unpin_item_by_id};

#[tauri::command]
pub fn toggle_pin(id: i64, state: bool) -> bool {
    // Set the state in the database
    if state {
        pin_item_by_id(id);
    } else {
        unpin_item_by_id(id);
    }

    return true;
}

#[tauri::command]
pub fn unpin_all() -> bool {
    // Set the state in the database
    let res = unpin_all_items();

    return res > 0;
}
