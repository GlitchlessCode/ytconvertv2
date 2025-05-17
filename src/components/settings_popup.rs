use super::*;

use ytconvertv2::{
    events::{
        update::{Update, UpdateTheme, UpdateUpdateSettings},
        AppEvent,
    },
    modifiers::{ThemeModifiers, ViewModifiers},
    views::settingsfield::{SettingsField, SettingsFieldModifiers},
};

pub fn settings_popup(cx: &mut Context) {
    Binding::new(cx, AppData::show_settings, |cx, show| {
        if show.get(cx) {
            Window::new(cx, |cx| {
                SettingsPopup::new(cx, AppData::theme, |cx| {
                    content(cx);
                });
            })
            .min_inner_size(Some((600, 400)))
            .title("Settings")
            .on_close(|ex| ex.emit(AppEvent::SetShowSettings(false)));
        }
    });
}

fn content(cx: &mut Context) {
    Collapsible::new(
        cx,
        |cx| {
            Label::new(cx, "Theme").with_text_primary(AppData::theme);
        },
        |cx| {
            SettingsField::new(cx, AppData::theme, |cx| {
                Button::new(cx, |cx| {
                    Label::new(cx, "Dark Mode").with_text_primary(AppData::theme)
                })
                .round_box(AppData::theme)
                .on_background_dark(AppData::theme)
                .width(Pixels(196.0))
                .on_press(|ex| ex.emit(Update::Theme(UpdateTheme::DarkDefault)));
                Button::new(cx, |cx| {
                    Label::new(cx, "Light Mode").with_text_primary(AppData::theme)
                })
                .round_box(AppData::theme)
                .on_background_light(AppData::theme)
                .width(Pixels(196.0))
                .on_press(|ex| ex.emit(Update::Theme(UpdateTheme::LightDefault)));
            })
            .set_name("Presets");

            group_label(cx, "Basic");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.primary),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::Primary(color))),
            )
            .set_name("Primary");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.border),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::Border(color))),
            )
            .set_name("Border");

            group_label(cx, "Text");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.text_primary),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::TextPrimary(color))),
            )
            .set_name("Primary");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.text_secondary),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::TextSecondary(color))),
            )
            .set_name("Secondary");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.text_light),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::TextLight(color))),
            )
            .set_name("Light");

            group_label(cx, "Background");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.background_dark),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::BackgroundDark(color))),
            )
            .set_name("Dark");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.background),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::Background(color))),
            )
            .set_name("Standard");
            SettingsField::new_color(
                cx,
                AppData::theme,
                AppData::theme.map(|theme| theme.background_light),
                |ex, color| ex.emit(Update::Theme(UpdateTheme::BackgroundLight(color))),
            )
            .set_name("Light");
        },
    )
    .round_box(AppData::theme)
    .on_background(AppData::theme);

    Collapsible::new(
        cx,
        |cx| {
            Label::new(cx, "Updates").with_text_primary(AppData::theme);
        },
        |cx| {
            SettingsField::new_toggle(
                cx,
                AppData::theme,
                AppData::update_settings.map(|settings| settings.auto_update),
                |ex| {
                    ex.emit(Update::UpdateSettings(
                        UpdateUpdateSettings::ToggleAutoUpdates,
                    ));
                },
            )
            .set_name("Automatically Update");
        },
    )
    .round_box(AppData::theme)
    .on_background(AppData::theme);
}

fn group_label(cx: &mut Context, name: &'static str) {
    Label::new(cx, name)
        .with_text_secondary(AppData::theme)
        .font_size("small")
        .padding_top(Pixels(3.0));
}
