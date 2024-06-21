use crate::utils::{dirs, help};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

#[derive(Debug, Clone)]
pub struct IClashTemp(pub Mapping);

impl IClashTemp {
    pub fn new() -> Self {
        let template: IClashTemp = Self::template();

        match dirs::clash_path().and_then(|path| help::read_merge_mapping(&path)) {
            Ok(mut map) => {
                template.0.keys().for_each(|key| {
                    if !map.contains_key(key) {
                        map.insert(key.clone(), template.0.get(key).unwrap().clone());
                    }
                });
                Self(map)
            }

            Err(err) => {
                log::error!("{}", err);
                template
            }
        }
    }

    pub fn template() -> Self {
        let mut map = Mapping::new();

        map.insert("mixed_port".into(), 7890.into());

        Self(map)
    }

    pub fn get_client_info(&self) -> ClashInfo {
        let config = &self.0;

        ClashInfo {
            mixed_port: Self::guard_mixed_port(config),
        }
    }

    pub fn guard_mixed_port(config: &Mapping) -> u16 {
        let mut port = config
            .get("mixed-port")
            .and_then(|value| match value {
                Value::String(val_str) => val_str.parse().ok(),
                Value::Number(val_num) => val_num.as_u64().map(|u| u as u16),
                _ => None,
            })
            .unwrap_or(7890);

        if port == 0 {
            port = 7890;
        }

        port
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClashInfo {
    pub mixed_port: u16,
}
