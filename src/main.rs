#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use vault::ui::setup;

fn main() {
    setup();
}