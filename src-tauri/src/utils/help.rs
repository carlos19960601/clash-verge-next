use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml::{Mapping, Value};

pub fn read_yaml<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    if !path.exists() {
        bail!("文件不存在 \"{}\"", path.display());
    }

    let yaml_str =
        fs::read_to_string(path).with_context(|| format!("读取文件失败 \"{}\"", path.display()))?;

    serde_yaml::from_str::<T>(&yaml_str)
        .with_context(|| format!("以yaml格式解析文件失败 \"{}\"", path.display()))
}

pub fn read_merge_mapping(path: &PathBuf) -> Result<Mapping> {
    let mut val: Value = read_yaml(path)?;
    val.apply_merge()
        .with_context(|| format!("apply 合并失败 \"{}\"", path.display()))?;

    Ok(val
        .as_mapping()
        .ok_or(anyhow!("转换成yaml Mapping格式失败 \"{}\"", path.display()))?
        .to_owned())
}

pub fn save_yaml<T: Serialize>(path: &PathBuf, data: &T, prefix: Option<&str>) -> Result<()> {
    let data_str = serde_yaml::to_string(data)?;

    let yaml_str = match prefix {
        Some(prefix) => format!("{prefix}\n\n{data_str}"),
        None => data_str,
    };

    let path_str = path.as_os_str().to_string_lossy().to_string();
    fs::write(path, yaml_str.as_bytes()).with_context(|| format!("保存文件 \"{path_str}\""))
}

#[macro_export]
macro_rules! log_err {
    ($result: expr) => {
        if let Err(err) = $result {
            log::error!(target: "app", "{err}");
        }
    };

    ($result: expr, $err_str: expr) => {
        if let Err(_) = $result {
            log::error!(target: "app", "{}", $err_str);
        }
    };
}
