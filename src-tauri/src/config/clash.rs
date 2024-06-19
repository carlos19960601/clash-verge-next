use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

pub struct IClashTemp(pub Mapping);

#[derive(Deserialize, Serialize)]
pub struct ClashInfo {
    pub mixed_port: u16,
    pub socks_port: u16,
    pub server: String,
    pub secret: Option<String>,
}
