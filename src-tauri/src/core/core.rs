use std::sync::Arc;

use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use tauri::api::process::CommandChild;

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
}
