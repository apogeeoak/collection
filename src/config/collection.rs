use serde::Deserialize;

// Struct to hold the [[collections]] table.
#[derive(Debug, Deserialize)]
pub struct Collection {
    pub label: String,
    pub width: u8,
    pub items: Vec<String>,
}
