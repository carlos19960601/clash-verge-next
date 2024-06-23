use std::sync::Arc;

use anyhow::{Ok, Result};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
// use sysinfo::{ProcessExt, System, SystemExt};
use tauri::api::process::{Command, CommandChild};

use crate::{
    // config::{Config, ConfigType},
    log_err,
};

#[derive(Debug)]
pub struct CoreManager {
    sidecar: Arc<Mutex<Option<CommandChild>>>,
}

impl CoreManager {
    pub fn global() -> &'static Self {
        static CORE_MANAGER: OnceCell<CoreManager> = OnceCell::new();

        CORE_MANAGER.get_or_init(|| CoreManager {
            sidecar: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init(&self) -> Result<()> {
        // tauri::async_runtime::spawn(async { logerr!(Self::global().run_core().await) });
        Ok(())
    }

    // pub async fn run_core(&self) -> Result<()> {
    //     let config_path = Config::generate_file(ConfigType::Run)?;

    //     let mut system: System = System::new();
    //     system.refresh_all();

    //     let procs = system.processes_by_name("verge-mihomo");
    //     for proc in procs {
    //         proc.kill();
    //     }

    //     let clash_core = { Config::verge().latest().clash_core.clone() };

    //     let cmd = Command::new_sidecar(clash_core)?;

    //     Ok(())
    // }
}
