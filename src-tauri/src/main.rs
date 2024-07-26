// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmds;
mod config;
mod core;
mod enhance;
mod utils;

use utils::server;

use crate::utils::{init, resolve};

fn main() -> std::io::Result<()> {
    if server::check_singleton().is_err() {
        println!("app exists");
        return Ok(());
    }

    // 原始的配置yaml初始化
    crate::log_err!(init::init_config());

    let builder = tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            resolve::resolve_setup(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // clash
            cmds::get_clash_info
        ]);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, e| {});

    Ok(())
}
