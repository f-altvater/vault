use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

use crate::helpers::InternalError;

mod test;

const MASTER_KEY_STR: &str = super::env::MASTER_KEY_STR;
const PW_KEY_STR: &str = super::env::PW_KEY_STR;
const GENERIC_KEY_STR: &str = super::env::GENERIC_KEY_STR;

pub fn encrypt_text(text: &str, is_master: bool, is_password: bool) -> Result<String, InternalError> {
    let use_key = 
        if is_master {MASTER_KEY_STR}
        else if is_password {PW_KEY_STR}
        else {GENERIC_KEY_STR};
    
    let key = Key::<Aes256Gcm>::from_slice(use_key.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);
    let ciphered_data = match cipher.encrypt(&nonce, text.as_bytes()) {
        Ok(data) => data,
        Err(_) => {
            return Err(
                InternalError::new(
                    "[CR_EP-1]",
                    "Failed to encrypt",
                )
            );
        }
    };

    // combining nonce and encrypted data together
    // for storage purposes
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    Ok(hex::encode(encrypted_data))
}

pub fn decrypt_text(text: &str, is_master: bool, is_password: bool) -> Result<String, InternalError> {
    let encrypted_data = match hex::decode(text) {
        Ok(data) => data,
        Err(_) => {
            return Err(
                InternalError::new(
                    "[CR_DP-1]",
                    "Failed to decode HEX String into Vec.",
                )
            );
        }
    };

    let use_key = 
        if is_master {MASTER_KEY_STR}
        else if is_password {PW_KEY_STR}
        else {GENERIC_KEY_STR};
    
    let key = Key::<Aes256Gcm>::from_slice(use_key.as_bytes());

    // we split the vector at 12th position because we used
    // nonce of length 12
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);

    let plaintext = match cipher.decrypt(nonce, ciphered_data) {
        Ok(text) => text,
        Err(_) => {
            return Err(
                InternalError::new(
                    "[CR_DP-2]",
                    "Failed to decrypt data.",
                )
            );
        }
    };

    match String::from_utf8(plaintext) {
        Ok(s) => return Ok(s),
        Err(_) => {
            return Err(
                InternalError::new(
                    "[CR_DP-3]",
                    "Failed to convert Vector of Bytes into String.",
                )
            );
        }
    }
}