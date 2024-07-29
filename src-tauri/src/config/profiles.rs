use anyhow::{bail, Result};
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
}
