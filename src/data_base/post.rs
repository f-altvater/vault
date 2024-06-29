use std::{fs::File, io::Write};

use utils::Master;

use super::*;

/// Adds an entry to the list and saves the list immediately
pub fn add_entry(name: &str, user_name: &str, password: &str, mut current_list: Vec<Entry>) -> Result<Vec<Entry>, InternalError> {
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

/// Saves the Master Password as an encrypted String
/// 
/// The Password should only be given as is, since it will be encrypted
/// inside this function and when getting the password, it will be given in
/// the encrypted state.
pub fn save_master(master_password: &str) -> Result<(), InternalError> {
    let master_encrypted = match encrypt_text(master_password, true, true) {
        Ok(s) => s,
        Err(err) => return Err(err)
    };

    let master = Master {
        master: master_encrypted,
    };
    let buffer = match serde_json::to_string_pretty(&master) {
        Ok(s) => s,
        Err(_) => return Err(InternalError::new(
            "[DB_P_SM-1]",
            "Failed to write buffer",
        ))
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
                Ok(_) => return Ok(()),
                Err(_) => return Err(InternalError::new(
                    "[DB_P_SM-2]",
                    "Failed to save buffer in file",
                )),
            }
        },
        Err(_) => {
            let new_file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(utils::MASTER_PATH);

            match new_file {
                Ok(mut f) => {
                    match f.write_all(buffer.as_bytes()) {
                        Ok(_) => return Ok(()),
                        Err(_) => return Err(InternalError::new(
                            "[DB_P_SM-3]",
                            "Failed to save buffer in new file",
                        )),
                    }
                },
                Err(err) => {
                    dbg!(err);
                    return Err(InternalError::new(
                        "[DB_P_SM-4]",
                        "Failed to create file",
                    ))
                }
            }
        }
    }
}

/// Saves the current state of the entries
pub fn save_current_list(list: Vec<Entry>) -> Result<(), InternalError> {

    let file = File::options()
        .read(true)
        .write(true)
        .open(utils::DB_PATH);

    let entries = utils::Entries {
        entries: list,
    };
    let buffer = match serde_json::to_string_pretty(&entries) {
        Ok(s) => s,
        Err(_) => return Err(InternalError::new(
            "[DB_P_SCL-1]",
            "Failed to write buffer",
        ))
    };

    match file {
        Ok(mut f) => {
            // the unwrap() call is safe since we ensure in the
            // declaration that we have write access
            f.set_len(0).unwrap();
            match f.write_all(buffer.as_bytes()) {
                Ok(_) => return Ok(()),
                Err(_) => return Err(InternalError::new(
                    "[DB_P_SCL-2]",
                    "Failed to save buffer in file",
                )),
            }
        },
        Err(_) => {
            let new_file = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(utils::DB_PATH);

            match new_file {
                Ok(mut f) => {
                    match f.write_all(buffer.as_bytes()) {
                        Ok(_) => return Ok(()),
                        Err(_) => return Err(InternalError::new(
                            "[DB_P_SCL-3]",
                            "Failed to save buffer in new file",
                        )),
                    }
                },
                Err(_) => {
                    return Err(InternalError::new(
                        "[DB_P_SCL-4]",
                        "Failed to create file",
                    ))
                }
            }
        }
    }
}