use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{cryptography::{decrypt_text, encrypt_text}, helpers::InternalError};

pub mod post;
pub mod get;
mod utils;

/// This structure holds the data of one Password Entry.
/// 
/// The `password` and `user_name` are only in the **encrypted** state. In order to
/// get the plain-text versions of those, they have to be decrypted using its implemented
/// method.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    id: String,
    name: String,
    user_name: String,
    password: String,
    created_at: i64,
    last_edited: i64,
}
impl Entry {
    pub fn new(name: &str, user_name: &str, password: &str) -> Result<Self, InternalError> {
        let id = Uuid::new_v4().to_string();
        let name = String::from(name);

        let user_name = match encrypt_text(user_name, false, false) {
            Ok(n) => n,
            Err(err) => return Err(err),
        };
        let password = match encrypt_text(password, false, true) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };

        let now = chrono::Local::now().timestamp();
        let created_at = now;
        let last_edited = now;

        Ok(Self {
            id,
            name,
            user_name,
            password,
            created_at,
            last_edited,
        })
    }
    
    pub fn name(&self) -> String {
        String::from(&self.name)
    }

    pub fn id(&self) -> String {
        String::from(&self.id)
    }

    /// Gets the informations needed to display.
    /// 
    /// The Password and Usernames are here already **decrypted**
    pub fn get_details(&self) -> Result<EntryDisplay, InternalError> {
        let user_name = match decrypt_text(&self.user_name, false, false) {
            Ok(u) => u,
            Err(err) => return Err(err),
        };
        let password = match decrypt_text(&self.password, false, true) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };

        Ok(EntryDisplay {
            id: self.id.clone(),
            name: String::from(&self.name),
            user_name,
            password,
            created_at: self.created_at,
            last_edited: self.last_edited,
        })
        
    }

    pub fn edit(&mut self, name: &str, user_name: &str, password: &str) -> Result<(), InternalError> {
        
        match encrypt_text(user_name, false, false) {
            Ok(u) => {
                self.user_name = u;
                self.edited();
            },
            Err(err) => return Err(err),
        }
        
        match encrypt_text(password, false, true) {
            Ok(p) => {
                self.password = p;
                self.edited();
            },
            Err(err) => return Err(err),
        }

        self.name = String::from(name);
        self.edited();

        Ok(())
    }

    fn edited(&mut self) {
        self.last_edited = chrono::Local::now().timestamp();
    }
}

#[derive(Debug, Clone)]
pub struct EntryDisplay {
    pub id: String,
    pub name: String,
    pub user_name: String,
    pub password: String,
    pub created_at: i64,
    pub last_edited: i64,
}