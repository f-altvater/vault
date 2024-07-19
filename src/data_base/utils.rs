use serde::{Deserialize, Serialize};

use super::*;

pub const DB_PATH: &str = "./data/db.json";
pub const MASTER_PATH: &str = "./data/m.json";
pub const DATA_FOLDER: &str = "./data";

#[derive(Debug, Deserialize, Serialize)]
pub struct Entries {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Master {
    pub master: String,
}
