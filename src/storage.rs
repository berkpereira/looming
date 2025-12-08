use crate::model::Item;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Storage {
    items: Vec<Item>,
}

// hard-coded for now!
fn data_file() -> PathBuf {
    PathBuf::from("looming-data.toml")
}

pub fn save(items: Vec<Item>) -> Result<()> {
    let storage = Storage { items };
    let contents = toml::to_string_pretty(&storage)?;
    fs::write(data_file(), contents)?;
    Ok(())
}

pub fn load() -> Result<Vec<Item>> {
    let contents = fs::read_to_string(data_file())?;
    let storage: Storage = toml::from_str(&contents)?;
    Ok(storage.items)
}
