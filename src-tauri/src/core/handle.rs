use std::sync::Arc;

use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use tauri::{AppHandle, Manager, Window};

use crate::log_err;

pub struct Handle {
    pub app_handle: Arc<Mutex<Option<AppHandle>>>,
}

impl Handle {
    pub fn global() -> &'static Self {
        static HANDLE: OnceCell<Handle> = OnceCell::new();
        HANDLE.get_or_init(|| Self {
            app_handle: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init(&self, app_handle: AppHandle) {
        *self.app_handle.lock() = Some(app_handle);
    }

    pub fn get_window(&self) -> Option<Window> {
        self.app_handle
            .lock()
            .as_ref()
            .and_then(|a| a.get_window("main"))
    }

    pub fn refresh_clash() {
        if let Some(window) = Self::global().get_window() {
            // 向js的listener发送事件
            log_err!(window.emit("verge:refresh-clash-config", "yes"));
        }
    }
}
