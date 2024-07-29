use serde_yaml::Mapping;

#[derive(Debug, Clone, Default)]
pub struct IRuntime {
    pub config: Option<Mapping>,
    pub exists_keys: Vec<String>,
}

impl IRuntime {
    pub fn new() -> Self {
        Self::default()
    }
}
