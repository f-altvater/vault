use std::{
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

use crate::helpers::InternalError;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    pub mode: super::Mode,
}

const SETTINGS_PATH: &str = "./data/settings.json";

pub fn save_settings(settings: &Settings) -> Result<(), InternalError> {
    let file = File::options().read(true).write(true).open(SETTINGS_PATH);

    let buffer = match serde_json::to_string_pretty(settings) {
        Ok(s) => s,
        Err(_) => return Err(InternalError::new("[UI_U_SS-1]", "Failed to write buffer")),
    };

    match file {
        Ok(mut f) => {
            // the unwrap() call is safe since we ensure in the
            // declaration that we have write access
            f.set_len(0).unwrap();
            match f.write_all(buffer.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(InternalError::new("[UI_U_SS-2]", "Failed to save buffer")),
            }
        }
        Err(_) => {
            let new_file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(SETTINGS_PATH);

            match new_file {
                Ok(mut f) => match f.write_all(buffer.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(InternalError::new(
                        "[UI_U_SS-3]",
                        "Failed to save buffer in new file",
                    )),
                },
                Err(_) => Err(InternalError::new("[UI_U_SS-4]", "Failed to create file")),
            }
        }
    }
}

pub fn load_settings() -> Settings {
    let file = File::options().read(true).open(SETTINGS_PATH);

    match file {
        Ok(_) => match fs::read_to_string(SETTINGS_PATH) {
            Ok(s) => match serde_json::from_str::<Settings>(&s) {
                Ok(settings) => settings,
                Err(_) => Settings::default(),
            },
            Err(_) => Settings::default(),
        },
        Err(_) => Settings::default(),
    }
}
