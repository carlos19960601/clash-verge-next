use serde::{Deserialize, Serialize};

use crate::utils::{dirs, help};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IVerge {
    pub language: Option<String>,
    pub theme_mode: Option<String>,
    pub traffic_graph: Option<bool>,

    pub clash_core: Option<String>,

    pub startup_script: Option<String>,
}

impl IVerge {
    pub fn new() -> Self {
        match dirs::verge_path().and_then(|path| help::read_yaml::<IVerge>(&path)) {
            Ok(config) => config,
            Err(err) => {
                log::error!(target: "app", "{err}");
                Self::template()
            }
        }
    }

    pub fn template() -> Self {
        Self {
            clash_core: Some("verge-mihomo".into()),
            language: Some("zh".into()),
            theme_mode: Some("system".into()),
            traffic_graph: Some(true),
            startup_script: None,
        }
    }
}
