use anyhow::{Ok, Result};
use std::path::PathBuf;
use tauri::{
    api::path::{data_dir, resource_dir},
    Env,
};

use crate::core::handle;

pub static APP_ID: &str = "io.github.clash-verge-next.clash-verge-next";

static CLASH_CONFIG: &str = "config.yaml";
static VERGE_CONFIG: &str = "verge.yaml";
static PROFILE_YAML: &str = "profiles.yaml";

pub fn clash_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(CLASH_CONFIG))
}

pub fn verge_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(VERGE_CONFIG))
}

pub fn profiles_path() -> Result<PathBuf> {
    Ok(app_home_dir()?.join(PROFILE_YAML))
}

pub fn app_home_dir() -> Result<PathBuf> {
    Ok(data_dir() // 对应的操作系统数据目录
        .ok_or(anyhow::anyhow!("获取app home路径失败"))?
        .join(APP_ID))
}

pub fn app_profiles_dir() -> Result<PathBuf> {
    Ok(app_home_dir()?.join("profiles"))
}

pub fn app_resources_dir() -> Result<PathBuf> {
    let handle = handle::Handle::global();
    let app_handle = handle.app_handle.lock();
    if let Some(app_handle) = app_handle.as_ref() {
        let res_dir = resource_dir(app_handle.package_info(), &Env::default())
            .ok_or(anyhow::anyhow!("获取app resource路径失败"))?
            .join("resources");

        return Ok(res_dir);
    }

    Err(anyhow::anyhow!("获取app resource路径失败"))
}

pub fn path_to_str(path: &PathBuf) -> Result<&str> {
    let path_str = path
        .as_os_str()
        .to_str()
        .ok_or(anyhow::anyhow!("failed to get path from {:?}", path))?;

    Ok(path_str)
}
