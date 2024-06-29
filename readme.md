# Vault

This is a pretty simple Password Manager written for personal use and as a simple Rust project.

---

On first use you will have to enter a "Master Password" which has to be entered every time you
open the app again.

Inside the app your entries will be listed and can be selected in order to check the credentials.
The password and username can be copied to your clipboard with the click of a button.
The entries can be edited if something has changed or can be deleted if they are not needed anymore.

## Upcoming Features

[ ] Search Function

The ability to search for the given name of an entry and have only search hits listed

[ ] Grouping of credentials

The ability to give credentials tags and search for credentials with the selected tag

[ ] Password Generator

when adding or editing an Entry you can let the app generate a password with given parameters

---

## Storage

The Entries will be stored localy on the users device inside a `json` file in which the username and password are stored
in an encrypted state.

The "Master Password" is stored in a separate `json` file.

The ecnryption for the passwords, master-password and usernames all use different key strings.
