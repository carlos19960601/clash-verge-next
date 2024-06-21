use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Ok, Result};
use serde::de::DeserializeOwned;
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

#[macro_export]
macro_rules! log_err {
    ($result: expr) => {
        if let Err(err) = $result {
            log::error!(target: "app", "{err}");
        }
    };
}
