use std::sync::Arc;

use anyhow::Result;
use auto_launch::AutoLaunch;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use sysproxy::Sysproxy;

use crate::config::{Config, IVerge};

pub struct Sysopt {
    // 当前代理设置
    cur_sysproxy: Arc<Mutex<Option<Sysproxy>>>,

    old_sysproxy: Arc<Mutex<Option<Sysproxy>>>,

    cur_autoproxy: Arc<Mutex<Option<Sysproxy>>>,

    old_autoproxy: Arc<Mutex<Option<Sysproxy>>>,

    auto_launch: Arc<Mutex<Option<AutoLaunch>>>,
}

impl Sysopt {
    pub fn global() -> &'static Sysopt {
        static SYSOPT: OnceCell<Sysopt> = OnceCell::new();

        SYSOPT.get_or_init(|| Sysopt {
            cur_sysproxy: Arc::new(Mutex::new(None)),
            old_sysproxy: Arc::new(Mutex::new(None)),
            cur_autoproxy: Arc::new(Mutex::new(None)),
            old_autoproxy: Arc::new(Mutex::new(None)),
            auto_launch: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init_sysproxy(&self) -> Result<()> {
        let port = Config::verge()
            .latest()
            .verge_mixed_port
            .unwrap_or(Config::clash().data().get_mixed_port());
        let pac_port = IVerge::get_singleton_port();

        let (enable, pac) = {
            let verge = Config::verge();
            let verge = verge.latest();
            (
                verge.enable_system_proxy.unwrap_or(false),
                verge.proxy_auto_config.unwrap_or(false),
            )
        };

        let mut sys = Sysproxy {
            enable,
            host: String::from("127.0.0.1"),
            port,
            bypass: get_bypass(),
        };

        if pac {
            sys.enable = false;
        } else {
        }

        Ok(())
    }
}

#[cfg(target_os = "windows")]
static DEFAULT_BYPASS: &str = "localhost;127.*;192.168.*;10.*;172.16.*;172.17.*;172.18.*;172.19.*;172.20.*;172.21.*;172.22.*;172.23.*;172.24.*;172.25.*;172.26.*;172.27.*;172.28.*;172.29.*;172.30.*;172.31.*;<local>";
#[cfg(target_os = "linux")]
static DEFAULT_BYPASS: &str = "localhost,127.0.0.1,192.168.0.0/16,10.0.0.0/8,172.16.0.0/12,::1";
#[cfg(target_os = "macos")]
static DEFAULT_BYPASS: &str =
    "127.0.0.1,192.168.0.0/16,10.0.0.0/8,172.16.0.0/12,localhost,*.local,*.crashlytics.com,<local>";

fn get_bypass() -> String {
    let bypass = DEFAULT_BYPASS.to_string();

    bypass
}
