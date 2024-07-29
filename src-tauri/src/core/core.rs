use crate::config::*;
use crate::core::clash_api;
use crate::log_err;
use crate::utils::dirs;
use anyhow::bail;
use anyhow::Result;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::api::process::CommandEvent;
use tauri::api::process::{Command, CommandChild};

#[derive(Debug)]
pub struct CoreManager {
    sidecar: Arc<Mutex<Option<CommandChild>>>,

    use_service_mode: Arc<Mutex<bool>>,
}

impl CoreManager {
    pub fn global() -> &'static Self {
        static CORE_MANAGER: OnceCell<CoreManager> = OnceCell::new();

        CORE_MANAGER.get_or_init(|| CoreManager {
            sidecar: Arc::new(Mutex::new(None)),
            use_service_mode: Arc::new(Mutex::new(false)),
        })
    }

    pub fn init(&self) -> Result<()> {
        tauri::async_runtime::spawn(async {
            // 启动clash
            log_err!(Self::global().run_core().await)
        });
        Ok(())
    }

    pub async fn run_core(&self) -> Result<()> {
        let config_path = Config::generate_file(ConfigType::Run)?;

        let mut system = System::new();
        system.refresh_all();
        let procs: Box<dyn Iterator<Item = &sysinfo::Process>> =
            system.processes_by_name("verge-mihomo");
        for proc in procs {
            log::debug!(target: "app", "kill all clash process");
            proc.kill_with(sysinfo::Signal::Interrupt);
        }

        if *self.use_service_mode.lock() {}

        let app_dir = dirs::app_home_dir()?;
        let app_dir = dirs::path_to_str(&app_dir)?;

        let clash_core = { Config::verge().latest().clash_core.clone() };
        let clash_core = clash_core.unwrap_or("verge-mihomo".into());

        let config_path = dirs::path_to_str(&config_path)?;
        let args = vec!["-d", app_dir, "-f", config_path];

        let cmd = Command::new_sidecar(clash_core)?;
        let (mut rx, cmd_child) = cmd.args(args).spawn()?;

        let mut sidecar = self.sidecar.lock();
        *sidecar = Some(cmd_child);
        drop(sidecar);

        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        log::info!(target: "app", "[mihomo]: {line}");
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    pub fn check_config(&self) -> Result<()> {
        let config_path = Config::generate_file(ConfigType::Run)?;
        let config_path = dirs::path_to_str(&config_path)?;

        let clash_core = { Config::verge().latest().clash_core.clone() };
        let clash_core = clash_core.unwrap_or("clash".into());

        let app_dir = dirs::app_home_dir()?;
        let app_dir = dirs::path_to_str(&app_dir)?;

        let output = Command::new_sidecar(clash_core)?
            .args(["-t", "-d", app_dir, "-f", config_path])
            .output()?;

        if !output.status.success() {
            let error = clash_api::parse_check_output(output.stdout.clone());
            bail!("{error}");
        }

        Ok(())
    }
}
