use crate::config::*;

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn get_clash_info() -> CmdResult<ClashInfo> {
    Ok(Config::clash().latest().get_client_info())
}
