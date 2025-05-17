use super::*;

use crate::{
    helpers::{format_color_hex, parse_color_hex},
    modifiers::{ThemeModifiers, ViewModifiers},
};

#[derive(Lens)]
pub struct SettingsField {
    name: Option<&'static str>,
}

impl SettingsField {
    pub fn new<T, F>(cx: &mut Context, theme: T, content: F) -> Handle<'_, Self>
    where
        T: Lens<Target = Theme>,
        F: FnOnce(&mut Context),
    {
        Self { name: None }
            .build(cx, |cx| {
                Label::new(cx, Self::name.map(|name| name.unwrap_or_default()))
                    .with_text_primary(theme)
                    .width(Auto)
                    .text_wrap(false)
                    .text_overflow(TextOverflow::Ellipsis);

                Element::new(cx)
                    .background_color(theme.map(|theme| theme.border))
                    .height(Pixels(1.0))
                    .width(Stretch(1.0));

                HStack::new(cx, |cx| content(cx))
                    .width(Auto)
                    .alignment(Alignment::Center)
                    .gap(Pixels(6.0));
            })
            .padding(Pixels(2.0))
            .gap(Pixels(10.0))
            .alignment(Alignment::Center)
            .layout_type(LayoutType::Row)
            .height(Auto)
            .width(Stretch(1.0))
    }

    pub fn new_toggle<'a, T, L, F>(
        cx: &'a mut Context,
        theme: T,
        lens: L,
        handler: F,
    ) -> Handle<'a, Self>
    where
        T: Lens<Target = Theme>,
        L: Lens<Target = bool>,
        F: Fn(&mut EventContext) + Send + Sync + 'static,
    {
        Self::new(cx, theme, |cx| {
            Switch::new(cx, lens).on_toggle(move |ex| handler(ex));
        })
    }

    pub fn new_color<'a, T, L, F>(
        cx: &'a mut Context,
        theme: T,
        lens: L,
        handler: F,
    ) -> Handle<'a, Self>
    where
        T: Lens<Target = Theme>,
        L: Lens<Target = Color>,
        F: Fn(&mut EventContext, Color) + Send + Sync + 'static,
    {
        Self::new(cx, theme, |cx| {
            Element::new(cx)
                .width(Pixels(36.0))
                .height(Pixels(12.0))
                .with_border(theme)
                .corner_radius(Pixels(6.0))
                .background_color(lens);

            Textbox::new(cx, lens.map(|lens| format_color_hex(*lens)))
                .width(Pixels(80.0))
                .text_align(TextAlign::Center)
                .round_box(theme)
                .on_background_dark(theme)
                .with_text_primary(theme)
                .on_submit(move |ex, hex, _| {
                    if let Some(color) = parse_color_hex(&hex) {
                        handler(ex, color)
                    }
                });
        })
    }
}

impl View for SettingsField {
    fn element(&self) -> Option<&'static str> {
        Some("settingsfield")
    }
}

pub trait SettingsFieldModifiers {
    fn set_name(self, name: &'static str) -> Self;
}

impl SettingsFieldModifiers for Handle<'_, SettingsField> {
    fn set_name(self, name: &'static str) -> Self {
        self.modify(|field| field.name = Some(name))
    }
}
