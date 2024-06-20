use crate::config::*;
use anyhow::Result;

type CmdResult<T = ()> = Result<T, String>;
#[tauri::command]
pub fn get_clash_info() -> CmdResult<ClashInfo> {
    Ok(ClashInfo {
        mixed_port: 7890,
        socks_port: 7890,
        server: String::new(),
        secret: Some(String::new()),
    })
}
