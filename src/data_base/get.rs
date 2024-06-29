use std::fs::{self, File};

use crate::helpers::InternalError;

use super::*;

/// Gets the **encrypted** Master Password
pub fn get_master() -> Result<String, InternalError> {
    let file = File::options()
        .read(true)
        .open(utils::MASTER_PATH);

    match file {
        Ok(_) => {
            match fs::read_to_string(utils::MASTER_PATH) {
                Ok(s) => {
                    match serde_json::from_str::<utils::Master>(&s) {
                        Ok(json) => Ok(json.master),
                        Err(_) => Err(InternalError::new(
                            "[DB_G_GM-1]",
                            "Failed to convert buffer",
                        ))
                    }
                },
                Err(_) => Err(InternalError::new(
                    "[DB_G_GM-2]",
                    "Failed to read contents"
                ))
            }
        },
        Err(_) => Err(InternalError::new(
            "[DB_G_GM-3]",
            "Failed to load file",
        ))
    }
}

/// Gets the **encrypted** entries
pub fn get_entries() -> Result<Vec<Entry>, InternalError> {
    let file = File::options()
        .read(true)
        .open(utils::DB_PATH);

    match file {
        Ok(_) => {
            match fs::read_to_string(utils::DB_PATH) {
                Ok(s) => {
                    match serde_json::from_str::<utils::Entries>(&s) {
                        Ok(json) => Ok(json.entries),
                        Err(_) => Err(InternalError::new(
                            "[DB_G_GE-1]",
                            "Failed to convert buffer",
                        ))
                    }
                },
                Err(_) => Err(InternalError::new(
                    "[DB_G_GE-2]",
                    "Failed to read contents"
                ))
            }
        },
        Err(_) => Ok(Vec::<Entry>::new())
    }
}