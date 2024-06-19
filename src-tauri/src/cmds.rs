use crate::config::*;
use anyhow::{Ok, Result};

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn get_clash_info(name: &str) -> CmdResult<ClashInfo> {}
