mod chain;
pub mod field;

use self::chain::*;
use crate::config::Config;
use field::use_keys;
use serde_yaml::Mapping;
use std::collections::{HashMap, HashSet};

type ResultLog = Vec<(String, String)>;

pub fn enhance() -> (Mapping, Vec<String>, HashMap<String, ResultLog>) {
    // config.yaml 的订阅
    let clash_config = { Config::clash().latest().0.clone() };

    let (clash_core, enable_tun, enable_builtin, socks_enabled, http_enabled) = {
        let verge = Config::verge();
        let verge: parking_lot::lock_api::MappedMutexGuard<
            parking_lot::RawMutex,
            crate::config::IVerge,
        > = verge.latest();
        (
            verge.clash_core.clone(),
            verge.enable_tun_mode.unwrap_or(false),
            verge.enable_builtin_enhanced.unwrap_or(true),
            verge.verge_socks_enabled.unwrap_or(true),
            verge.verge_http_enabled.unwrap_or(true),
        )
    };

    #[cfg(not(target_os = "windows"))]
    let redir_enabled = {
        let verge = Config::verge();
        let verge = verge.latest();

        verge.verge_redir_enabled.unwrap_or(true)
    };

    // 从profiles里拿东西
    let (mut config, chain) = {
        let profiles = Config::profiles();
        let profiles = profiles.latest();

        let current = profiles.current_mapping().unwrap_or_default();

        let chain = match profiles.chain.as_ref() {
            Some(chain) => chain
                .iter()
                .filter_map(|uid: &String| profiles.get_item(uid).ok())
                .filter_map(<Option<ChainItem>>::from)
                .collect::<Vec<ChainItem>>(),
            None => vec![],
        };

        (current, chain)
    };

    let mut result_map = HashMap::new(); // 保存脚本日志
    let mut exists_keys = use_keys(&config); // 保存出现过的keys

    for (key, value) in clash_config.into_iter() {
        if key.as_str() == Some("tun") {
        } else {
            if key.as_str() == Some("socks-port") && !socks_enabled {
                config.remove("socks-port");
                continue;
            }
            if key.as_str() == Some("port") && !http_enabled {
                config.remove("port");
                continue;
            }
            #[cfg(not(target_os = "windows"))]
            {
                if key.as_str() == Some("redir-port") && !redir_enabled {
                    config.remove("redir-pirt");
                    continue;
                }
            }

            config.insert(key, value);
        }
    }

    let mut exist_set = HashSet::new();
    exist_set.extend(exists_keys);
    exists_keys = exist_set.into_iter().collect();

    (config, exists_keys, result_map)
}
