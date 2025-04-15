use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub enum BookAction {
    Create { id: u32, title: String, year: u16 },
    Update { id: u32, title: String, year: u16 },
    Delete { id: u32 },
}

pub fn load_actions_from_json(file_path: &str) -> Vec<BookAction> {
    let content = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&content).unwrap()
}
