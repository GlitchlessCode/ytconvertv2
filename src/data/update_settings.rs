use super::*;
use nanoserde::{DeBin, SerBin};

#[derive(Debug, Data, Clone, SerBin, DeBin)]
pub struct UpdateSettings {
    pub auto_update: bool,
}

impl Default for UpdateSettings {
    fn default() -> Self {
        Self { auto_update: true }
    }
}
