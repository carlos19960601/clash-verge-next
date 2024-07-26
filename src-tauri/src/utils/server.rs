use anyhow::Result;
use port_scanner::local_port_available;

use crate::config::IVerge;

pub fn check_singleton() -> Result<()> {
    let port = IVerge::get_singleton_port();

    if !local_port_available(port) {
        tauri::async_runtime::block_on(async { Ok(()) })
    } else {
        Ok(())
    }
}
