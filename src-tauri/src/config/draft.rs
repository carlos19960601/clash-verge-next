use super::IClashTemp;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Draft<T: Clone + ToOwned> {
    inner: Arc<Mutex<(T, Option<T>)>>,
}

macro_rules! draft_define {
    ($id: ident) => {
        impl Draft<$id> {
            pub fn latest(&self) -> MappedMutexGuard<$id> {
                MutexGuard::map(self.inner.lock(), |guard| &mut guard.0)
            }
        }

        impl From<$id> for Draft<$id> {
            fn from(value: $id) -> Self {
                Draft {
                    inner: Arc::new(Mutex::new((value, None))),
                }
            }
        }
    };
}

draft_define!(IClashTemp);
