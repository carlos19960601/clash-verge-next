use crate::{config::Config, core::*, utils::init};
use once_cell::sync::OnceCell;
use tauri::{App, Manager};

use crate::log_err;

pub static VERSION: OnceCell<String> = OnceCell::new();

pub fn resolve_setup(app: &mut App) {
    // log_err!(Config::init_config());
    let version = app.package_info().version.to_string();
    handle::Handle::global().init(app.app_handle());
    VERSION.get_or_init(|| version.clone());

    log_err!(init::init_resources());
    log_err!(init::startup_script());

    log_err!(CoreManager::global().init());
}
