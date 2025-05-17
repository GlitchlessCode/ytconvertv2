use crate::modifiers::ThemeModifiers;

use super::*;

pub struct CustomProgressBar {}

impl CustomProgressBar {
    pub fn new<T, F>(cx: &mut Context, theme: T, progress: F) -> Handle<Self>
    where
        F: Lens<Target = f32>,
        T: Lens<Target = Theme>,
    {
        Self {}
            .build(cx, |cx| {
                let progress = progress.map(|progress| Percentage(progress * 100.0));
                HStack::new(cx, |cx| {
                    Element::new(cx)
                        .width(progress)
                        .corner_bottom_right_radius(Percentage(50.0))
                        .corner_top_right_radius(Percentage(50.0))
                        .background_color(theme.map(|theme| theme.primary));
                });
            })
            .with_border(theme)
            .on_background_dark(theme)
    }
}

impl View for CustomProgressBar {
    fn element(&self) -> Option<&'static str> {
        Some("progressbar")
    }
}
