use serde_yaml::Mapping;

use crate::config::PrfItem;

#[derive(Debug, Clone)]
pub enum ChainType {
    Merge(Mapping),
    Script(String),
}

#[derive(Debug, Clone)]
pub struct ChainItem {
    pub uid: String,
    pub data: ChainType,
}

impl From<&PrfItem> for Option<ChainItem> {
    fn from(item: &PrfItem) -> Self {
        None
    }
}
