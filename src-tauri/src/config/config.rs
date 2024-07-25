use anyhow::{Ok, Result};
use once_cell::sync::OnceCell;

use crate::{config::IRuntime, utils::dirs};

use super::{Draft, IClashTemp, IVerge};

pub const RUNTIME_CONFIG: &str = "clash-verge.yaml";
pub const CHECK_CONFIG: &str = "clash-verge-check.yaml";

pub struct Config {
    clash_config: Draft<IClashTemp>,
    verge_config: Draft<IVerge>,
    runtime_config: Draft<IRuntime>,
}

impl Config {
    pub fn global() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::new();

        CONFIG.get_or_init(|| Config {
            clash_config: Draft::from(IClashTemp::new()),
            verge_config: Draft::from(IVerge::new()),
            runtime_config: Draft::from(IRuntime::new()),
        })
    }

    pub fn clash() -> Draft<IClashTemp> {
        Self::global().clash_config.clone()
    }

    pub fn verge() -> Draft<IVerge> {
        Self::global().verge_config.clone()
    }

    pub fn runtime() -> Draft<IRuntime> {
        Self::global().runtime_config.clone()
    }

    pub fn init_config() -> Result<()> {
        // crate::log_err!(Self::generate());

        //     if let Err(err) = Self::generate_file(ConfigType::Run) {
        //         log::error!(target: "app", "{err}");
        //         let runtime_path = dirs::app_home_dir()?.join(RUNTIME_CONFIG);
        //         if !runtime_path.exists() {
        //             help::save_yaml(
        //                 &runtime_path,
        //                 &Config::clash().latest().0,
        //                 Some("# Clash Verge Runtime"),
        //             )?;
        //         }
        //     }

        Ok(())
    }

    // 将订阅丢到对应的文件中
    // pub fn generate_file(typ: ConfigType) -> Result<PathBuf> {
    //     let path = match typ {
    //         ConfigType::Run => dirs::app_home_dir()?.join(RUNTIME_CONFIG),
    //         ConfigType::Check => dirs::app_home_dir()?.join(CHECK_CONFIG),
    //     };

    //     let runtime = Config::runtime();

    //     Ok(path)
    // }

    // pub fn generate() -> Result<()> {
    //     let (config) = enhance::enhance();
    //     *Config::runtime().draft() = IRuntime {
    //         config: Some(config),
    //     };
    //     Ok(())
    // }
}

#[derive(Debug)]
pub enum ConfigType {
    Run,
    Check,
}
