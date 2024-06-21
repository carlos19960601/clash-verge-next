use std::path::PathBuf;

use anyhow::Result;
use tauri::api::path::data_dir;

pub static APP_ID: &str = "io.github.clash-verge-next.clash-verge-next";

static CLASH_CONFIG: &str = "config.yaml";

pub fn clash_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(CLASH_CONFIG))
}

pub fn app_home_dir() -> Result<PathBuf> {
    Ok(data_dir()
        .ok_or(anyhow::anyhow!("获取app home路径失败"))?
        .join(APP_ID))
}
