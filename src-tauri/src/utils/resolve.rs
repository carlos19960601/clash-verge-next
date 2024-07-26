use crate::{
    config::{Config, IVerge},
    core::*,
    utils::init,
};
use once_cell::sync::OnceCell;
use serde_yaml::Mapping;
use tauri::{App, Manager};

use crate::log_err;

pub static VERSION: OnceCell<String> = OnceCell::new();

pub fn resolve_setup(app: &mut App) {
    let version: String = app.package_info().version.to_string();
    handle::Handle::global().init(app.app_handle());
    VERSION.get_or_init(|| version.clone());

    // 拷贝resource(Country.mmdb, geiop.dat等)拷贝到app_dir
    log_err!(init::init_resources());
    log_err!(init::startup_script());

    let enable_random_port = Config::verge().latest().enable_random_port.unwrap_or(false);

    let port = Config::verge()
        .latest()
        .verge_mixed_port
        .unwrap_or(Config::clash().data().get_mixed_port());

    if enable_random_port {}

    Config::verge().data().patch_config(IVerge {
        verge_mixed_port: Some(port),
        ..IVerge::default()
    });
    let _ = Config::verge().data().save_file();

    let mut mapping = Mapping::new();
    mapping.insert("mixed-port".into(), port.into());
    Config::clash().data().patch_config(mapping);
    let _ = Config::clash().data().save_config();

    // 启动核心
    log::trace!("init config");
    log_err!(Config::init_config());

    log::trace!("launch core");
    log_err!(CoreManager::global().init());

    log_err!(sysopt::Sysopt::global().init_sysproxy());
}
