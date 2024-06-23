use serde_yaml::Mapping;

#[derive(Debug, Clone, Default)]
pub struct IRuntime {
    pub config: Option<Mapping>,
}

impl IRuntime {
    pub fn new() -> Self {
        Self::default()
    }
}
