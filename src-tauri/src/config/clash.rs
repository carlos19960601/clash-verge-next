use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use crate::utils::{dirs, help};
use serde::{ Deserialize, Serialize};
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
            server: Self::guard_client_ctrl(config),
            secret: config.get("secret").and_then(|value| match value {
                Value::String(val_str) => Some(val_str.clone()),
                Value::Bool(val_bool) => Some(val_bool.to_string()),
                Value::Number(val_num) => Some(val_num.to_string()),
                _ => None,
            }),
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

    pub fn guard_server_ctrl(config: &Mapping) -> String {
        config
            .get("external-controller")
            .and_then(|value| match value.as_str() {
                Some(val_str) => {
                    let val_str = val_str.trim();

                    let val = match val_str.starts_with(":") {
                        true => format!("127.0.0.1{}", val_str),
                        false => val_str.to_string(),
                    };

                    SocketAddr::from_str(val.as_str())
                        .ok()
                        .map(|s| s.to_string())
                }
                None => None,
            })
            .unwrap_or("127.0.0.1:9090".into())
    }

    pub fn guard_client_ctrl(config: &Mapping) -> String {
        let value = Self::guard_server_ctrl(config);
        match SocketAddr::from_str(value.as_str()) {
            Ok(mut socket) => {
                if socket.ip().is_unspecified() {
                    socket.set_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
                }
                socket.to_string()
            }
            Err(_) => "127.0.0.1:9090".into(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClashInfo {
    pub mixed_port: u16,
    // 同`external-controller`
    pub server: String,
    pub secret: Option<String>,
}
