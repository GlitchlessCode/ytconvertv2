use super::*;

#[derive(Lens)]
pub struct AppAnimationData {
    pub fade_in: Animation,
    pub fade_out: Animation,
}

impl Model for AppAnimationData {}
