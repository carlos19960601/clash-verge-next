use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use sysproxy::Sysproxy;

pub struct Sysopt {
    // 当前代理设置
    cur_sysproxy: Arc<Mutex<Option<Sysproxy>>>,

    old_sysproxy: Arc<<Mutex<Option<Sysproxy>>>,
}


impl Sysopt {
    pub fn global() -> &'static Sysopt {
        static  SYSOPT: OnceCell<Sysopt> = OnceCell::new();

        SYSOPT.get_or_init(|| Sysopt {
            cur_sysproxy: Arc::new(Mutex::new(None)),
            old_sysproxy: Arc::new(Mutex::new(None)),
        })
    }

    pub fn init_sysproxy(&self) ->Result<()> {
        Ok(())
    }
}