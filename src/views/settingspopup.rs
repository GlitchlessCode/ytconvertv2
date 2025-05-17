use super::*;

use crate::modifiers::ThemeModifiers;

pub struct SettingsPopup {}

impl SettingsPopup {
    pub fn new<T, F>(cx: &mut Context, theme: T, content: F) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
        F: FnOnce(&mut Context),
    {
        Self {}
            .build(cx, |cx| {
                content(cx);

                VStack::new(cx, |cx| {
                    Label::new(cx, "More settings coming soon...")
                        .with_text_light(theme)
                        .font_size("small");
                })
                .alignment(Alignment::Center)
                .padding(Pixels(12.0));
            })
            .gap(Pixels(3.0))
            .size(Stretch(1.0))
            .padding(Pixels(6.0))
            .layout_type(LayoutType::Column)
            .on_background(theme)
    }
}

impl View for SettingsPopup {
    fn element(&self) -> Option<&'static str> {
        Some("settingspopup")
    }
}
