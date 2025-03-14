use super::*;

pub struct AnimatedBinding {}

impl AnimatedBinding {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {})
    }
}

impl View for AnimatedBinding {
    // No element provided
    fn element(&self) -> Option<&'static str> {
        None
    }
}
