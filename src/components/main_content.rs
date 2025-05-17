use super::*;
use draw_export_settings::draw_export_settings;
use vizia::icons::ICON_FOLDER;
use ytconvertv2::{
    error::{Error, ErrorSeverity},
    events::AppEvent,
    helpers::{format_seconds, labelled},
    modifiers::{ThemeModifiers, ViewModifiers},
};

mod draw_export_settings;

pub fn main_content(cx: &mut Context) {
    // Main Toolbar
    Toolbar::new(cx, AppData::theme).on_exit(|ex| ex.emit(WindowEvent::WindowClose));

    AppData::theme
        .map(|theme| theme.primary)
        .view(cx.data().unwrap_or_else(|| panic!()));

    // Sub tool stack
    HStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            labelled(cx, AppData::theme, "Export Path", |cx| {
                HStack::new(cx, |cx| {
                    ScrollView::new(cx, |cx| {
                        Label::new(
                            cx,
                            AppData::current_location.map(|loc| {
                                if let Some(path) = loc {
                                    path.display().to_string()
                                } else {
                                    "No Path Chosen".to_string()
                                }
                            }),
                        )
                        .padding(Pixels(6.0))
                        .with_text_primary(AppData::theme);
                    })
                    .width(Stretch(1.0))
                    .show_horizontal_scrollbar(true);

                    Button::new(cx, |cx| Svg::new(cx, ICON_FOLDER))
                        .fill(AppData::theme.map(|theme| theme.background_dark))
                        .alignment(Alignment::Center)
                        .background_color(AppData::theme.map(|theme| theme.primary))
                        .on_press(|ex| ex.emit(AppEvent::RequestLocationChange));
                })
                .alignment(Alignment::Left)
                .height(Units::Auto)
                .round_box(AppData::theme)
                .on_background_dark(AppData::theme);
            })
            .height(Auto);

            labelled(cx, AppData::theme, "Export Settings", |cx| {
                draw_export_settings(cx);
            })
            .height(Stretch(1.0));
        })
        .gap(Pixels(3.0))
        .width(Stretch(1.0));

        VStack::new(cx, |cx| {
            labelled(cx, AppData::theme, "Tasks", |cx| {
                TaskQueueView::new(cx, AppData::theme, AppData::task_queue)
                    .round_box(AppData::theme)
                    .on_background_light(AppData::theme);
            })
            .height(Stretch(1.0));

            VStack::new(cx, |cx| {
                ActiveTaskView::new(cx, AppData::theme, AppData::active_task);
            })
            .round_box(AppData::theme)
            .height(Pixels(120.0))
            .on_background_light(AppData::theme);
        })
        .gap(Pixels(6.0))
        .width(Pixels(320.0));
    })
    .gap(Pixels(6.0));
}
