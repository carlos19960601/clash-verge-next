use once_cell::sync::OnceCell;

use super::{Draft, IClashTemp};

pub struct Config {
    clash_config: Draft<IClashTemp>,
}

impl Config {
    pub fn global() -> &'static Config {
        static CONFIG: OnceCell<Config> = OnceCell::new();

        CONFIG.get_or_init(|| Config {
            clash_config: Draft::from(IClashTemp::new()),
        })
    }

    pub fn clash() -> Draft<IClashTemp> {
        Self::global().clash_config.clone()
    }
}

#[derive(Debug)]
pub enum ConfigType {
    Run,
    Check,
}
