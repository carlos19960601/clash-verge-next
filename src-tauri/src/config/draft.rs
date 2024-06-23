use super::{IClashTemp, IRuntime, IVerge};
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
                MutexGuard::map(self.inner.lock(), |inner| {
                    if inner.1.is_none() {
                        &mut inner.0
                    } else {
                        inner.1.as_mut().unwrap()
                    }
                })
            }

            pub fn draft(&self) -> MappedMutexGuard<$id> {
                MutexGuard::map(self.inner.lock(), |inner| {
                    if inner.1.is_none() {
                        inner.1 = Some(inner.0.clone());
                    }

                    inner.1.as_mut().unwrap()
                })
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
draft_define!(IRuntime);
draft_define!(IVerge);
