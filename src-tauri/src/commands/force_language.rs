use crate::core::database_api::set_forced_language;

#[tauri::command]
pub fn force_language(id: i64, language: &str) {
    // Call the function to set the forced language in the database
    set_forced_language(id, language);
}
