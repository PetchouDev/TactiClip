use serde::Serialize;

// Struct to represent a clipboard entry
#[derive(Serialize, Clone, Debug)]
pub struct ClipboardEntry {
    pub id: i64,
    pub entry_type: String,
    pub content: String,
    pub added_at: String,
    pub pinned: bool,
    pub forced_language: Option<String>
}