use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use utils::{Master, DATA_FOLDER};

use super::*;

/// Adds an entry to the list and saves the list immediately
pub fn add_entry(
    name: &str,
    user_name: &str,
    password: &str,
    mut current_list: Vec<Entry>,
) -> Result<Vec<Entry>, InternalError> {
    let new_entry = match Entry::new(name, user_name, password) {
        Ok(e) => e,
        Err(err) => return Err(err),
    };
    current_list.push(new_entry);

    match save_current_list(current_list.clone()) {
        Ok(_) => Ok(current_list),
        Err(err) => Err(err),
    }
}

/// Saves the Master Password as an encrypted String and creates the directory
/// for all the saved data.
///
/// The Password should only be given as is, since it will be encrypted
/// inside this function and when getting the password, it will be given in
/// the encrypted state.
pub fn save_master(master_password: &str) -> Result<(), InternalError> {
    match Path::new(DATA_FOLDER).try_exists() {
        Ok(exists) => {
            if !exists {
                let dir = fs::create_dir(DATA_FOLDER);

                match dir {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(InternalError::new(
                            "[DB_P_SM-1]",
                            &format!("Failed to create folder {}.", DATA_FOLDER),
                        ));
                    }
                }
            }
        }
        Err(_) => {
            return Err(InternalError::new(
                "[DB_P_SM-1]",
                &format!("Could not check if folder {} exists.", DATA_FOLDER),
            ));
        }
    }

    let master_encrypted = match encrypt_text(master_password, true, true) {
        Ok(s) => s,
        Err(err) => return Err(err),
    };

    let master = Master {
        master: master_encrypted,
    };
    let buffer = match serde_json::to_string_pretty(&master) {
        Ok(s) => s,
        Err(_) => return Err(InternalError::new("[DB_P_SM-1]", "Failed to write buffer")),
    };

    let file = File::options()
        .read(true)
        .write(true)
        .open(utils::MASTER_PATH);

    match file {
        Ok(mut f) => {
            // the unwrap() call is safe since we ensure in the
            // declaration that we have write access
            f.set_len(0).unwrap();
            match f.write_all(buffer.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(InternalError::new(
                    "[DB_P_SM-2]",
                    "Failed to save buffer in file",
                )),
            }
        }
        Err(_) => {
            let new_file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(utils::MASTER_PATH);

            match new_file {
                Ok(mut f) => match f.write_all(buffer.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(InternalError::new(
                        "[DB_P_SM-3]",
                        "Failed to save buffer in new file",
                    )),
                },
                Err(err) => {
                    dbg!(err);
                    Err(InternalError::new("[DB_P_SM-4]", "Failed to create file"))
                }
            }
        }
    }
}

/// Saves the current state of the entries
pub fn save_current_list(list: Vec<Entry>) -> Result<(), InternalError> {
    let file = File::options().read(true).write(true).open(utils::DB_PATH);

    let entries = utils::Entries { entries: list };
    let buffer = match serde_json::to_string_pretty(&entries) {
        Ok(s) => s,
        Err(_) => return Err(InternalError::new("[DB_P_SCL-1]", "Failed to write buffer")),
    };

    match file {
        Ok(mut f) => {
            // the unwrap() call is safe since we ensure in the
            // declaration that we have write access
            f.set_len(0).unwrap();
            match f.write_all(buffer.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(InternalError::new(
                    "[DB_P_SCL-2]",
                    "Failed to save buffer in file",
                )),
            }
        }
        Err(_) => {
            let new_file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(utils::DB_PATH);

            match new_file {
                Ok(mut f) => match f.write_all(buffer.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(InternalError::new(
                        "[DB_P_SCL-3]",
                        "Failed to save buffer in new file",
                    )),
                },
                Err(_) => Err(InternalError::new("[DB_P_SCL-4]", "Failed to create file")),
            }
        }
    }
}
