use crate::model::Item;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Storage {
    items: Vec<Item>,
}

// Global data location for macOS
fn data_file() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    let app_support = PathBuf::from(home)
        .join("Library")
        .join("Application Support")
        .join("looming");

    // Ensure directory exists
    if !app_support.exists() {
        fs::create_dir_all(&app_support).expect("Failed to create application support directory");
    }

    app_support.join("looming-data.toml")
}

pub fn save(items: Vec<Item>) -> Result<()> {
    let storage = Storage { items };
    let contents = toml::to_string_pretty(&storage)?;
    fs::write(data_file(), contents)?;
    Ok(())
}

pub fn load() -> Result<Vec<Item>> {
    let file_path = data_file();

    // If file doesn't exist, return empty list
    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(file_path)?;
    let storage: Storage = toml::from_str(&contents)?;
    Ok(storage.items)
}
