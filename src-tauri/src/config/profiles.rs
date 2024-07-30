use std::{fs, io::Write};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

use crate::utils::{dirs, help};

use super::prfitem::PrfItem;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IProfiles {
    pub current: Option<String>,

    pub chain: Option<Vec<String>>,

    pub items: Option<Vec<PrfItem>>,
}

impl IProfiles {
    pub fn new() -> Self {
        match dirs::profiles_path().and_then(|path| help::read_yaml::<Self>(&path)) {
            Ok(profiles) => profiles,
            Err(err) => {
                log::error!(target: "app", "{err}");
                Self::template()
            }
        }
    }

    pub fn template() -> Self {
        Self {
            items: Some(vec![]),
            ..Self::default()
        }
    }

    pub fn current_mapping(&self) -> Result<Mapping> {
        match (self.current.as_ref(), self.items.as_ref()) {
            (Some(current), Some(items)) => {
                if let Some(item) = items.iter().find(|e| e.uid.as_ref() == Some(current)) {
                    let file_path = match item.file.as_ref() {
                        Some(file) => dirs::app_profiles_dir()?.join(file),
                        None => bail!("failed to get the file field"),
                    };
                    return help::read_merge_mapping(&file_path);
                };
                bail!("failed to find the current profile \"uid:{current}\"")
            }
            _ => Ok(Mapping::new()),
        }
    }

    pub fn get_item(&self, uid: &String) -> Result<&PrfItem> {
        if let Some(items) = self.items.as_ref() {
            let some_uid = Some(uid.clone());
            for each in items.iter() {
                if each.uid == some_uid {
                    return Ok(each);
                }
            }
        }

        bail!("获取pofile item \"uid:{uid}\"失败");
    }

    pub fn append_item(&mut self, mut item: PrfItem) -> Result<()> {
        if item.uid.is_none() {
            bail!("uid不能为null")
        }

        if let Some(file_data) = item.file_data.take() {
            if item.file.is_none() {
                bail!("文件不能为null")
            }

            let file = item.file.clone().unwrap();
            let path = dirs::app_profiles_dir()?.join(&file);

            fs::File::create(path)
                .with_context(|| format!("创建文件失败 \"{}\"", file))?
                .write(file_data.as_bytes())
                .with_context(|| format!("写文件失败 \"{}\"", file))?;
        }

        if self.items.is_none() {
            self.items = Some(vec![]);
        }

        if let Some(items) = self.items.as_mut() {
            items.push(item)
        }

        self.save_file()
    }

    pub fn save_file(&self) -> Result<()> {
        help::save_yaml(
            &dirs::profiles_path()?,
            self,
            Some("# Profiles Config for Clash Verge"),
        )
    }
}
