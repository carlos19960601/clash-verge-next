use tauri::App;

use crate::log_err;

pub fn resolve_setup(app: &mut App) {
    log_err!(CoreManager::global().init());
}
