use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::utils::{dirs, help};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IVerge {
    pub app_singleton_port: Option<u16>,
    pub language: Option<String>,
    pub theme_mode: Option<String>,
    pub traffic_graph: Option<bool>,
    pub clash_core: Option<String>,
    pub startup_script: Option<String>,
    /// 是否启用随机端口
    pub enable_random_port: Option<bool>,
    pub verge_mixed_port: Option<u16>,

    pub enable_tun_mode: Option<bool>,
    /// 是否使用内部的脚本支持，默认为真
    pub enable_builtin_enhanced: Option<bool>,
    pub verge_socks_enabled: Option<bool>,
    pub verge_http_enabled: Option<bool>,

    /// verge 的各种 port 用于覆盖 clash 的各种 port
    #[cfg(not(target_os = "windows"))]
    pub verge_redir_port: Option<u16>,
    #[cfg(not(target_os = "windows"))]
    pub verge_redir_enabled: Option<bool>,
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
            ..Self::default()
        }
    }

    pub fn patch_config(&mut self, patch: IVerge) {
        macro_rules! patch {
            ($key: tt) => {
                if patch.$key.is_some() {
                    self.$key = patch.$key;
                }
            };
        }

        patch!(verge_mixed_port);
    }

    pub fn get_singleton_port() -> u16 {
        const SERVER_PORT: u16 = 33331;

        match dirs::verge_path().and_then(|path| help::read_yaml::<IVerge>(&path)) {
            Ok(config) => config.app_singleton_port.unwrap_or(SERVER_PORT),
            Err(_) => SERVER_PORT,
        }
    }

    pub fn save_file(&self) -> Result<()> {
        help::save_yaml(&dirs::verge_path()?, &self, Some("# Clash Verge Config"))
    }
}
