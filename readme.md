# Vault

This is a pretty simple Password Manager written for personal use and as a simple Rust project.

---

On first use you will have to enter a "Master Password" which has to be entered every time you
open the app again.
Inside the app your entries will be listed and can be selected in order to check the credentials. The password and username can be copied to your clipboard with the click of a button. The entries can be edited if something has changed or can be deleted if they are not needed anymore.

## Upcoming Features

[ ] Search Function - The ability to search for the given name of an entry and have only search hits listed

[ ] Grouping of credentials - The ability to give credentials tags and search for credentials with the selected tag

[ ] Password Generator - when adding or editing an Entry you can let the app generate a password with given parameters

---

## Storage

The Entries will be stored localy on the users device inside a `json` file in which the username and password are stored
in an encrypted state.
The "Master Password" is stored in a separate `json` file.
The ecnryption for the passwords, master-password and usernames all use different key strings.

---

## Executable

The current version of the executable can be found [here](https://github.com/f-altvater/vault_executable).
It is only build for Windows atm and it is not planned to build it for anything else.

It can be build almost purely from this repo. But inside the **src** folder you have to add 3 constants inside a `env.rs` file:

```Rust
// has to be 32 char long, because AES-256 requires a key length of 32 bytes.
pub const PW_KEY_STR: &str = "thiskeystrmustbe32charlongtowork";
pub const MASTER_KEY_STR: &str = "thiskeystrmustbe32charlongtowork";
pub const GENERIC_KEY_STR: &str = "thiskeystrmustbe32charlongtowork";
```

Suggestion would be to use 3 diferent keystrings for them.
