// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::utils::resolve;

mod cmds;
mod config;
mod core;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            resolve::resolve_setup(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // clash
            cmds::get_clash_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
