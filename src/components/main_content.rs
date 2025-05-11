use super::*;
use draw_export_settings::draw_export_settings;
use vizia::icons::ICON_FOLDER;
use ytconvertv2::{
    error::{Error, ErrorSeverity},
    events::AppEvent,
    helpers::{format_seconds, labelled},
    modifiers::ViewModifiers,
};

mod draw_export_settings;

pub fn main_content(cx: &mut Context) {
    // Main Toolbar
    Toolbar::new(cx, AppData::theme).on_exit(|ex| ex.emit(WindowEvent::WindowClose));

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
                        .color(AppData::theme.map(|theme| theme.text_primary));
                    })
                    .width(Stretch(1.0))
                    .show_horizontal_scrollbar(true);

                    Button::new(cx, |cx| Svg::new(cx, ICON_FOLDER))
                        .alignment(Alignment::Center)
                        .background_color(AppData::theme.map(|theme| theme.primary))
                        .on_press(|ex| ex.emit(AppEvent::RequestLocationChange));
                })
                .alignment(Alignment::Left)
                .height(Units::Auto)
                .round_box(AppData::theme)
                .background_color(AppData::theme.map(|theme| theme.background_dark));
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
                    .background_color(AppData::theme.map(|theme| theme.background_light));
            })
            .height(Stretch(1.0));

            VStack::new(cx, |cx| {
                ActiveTaskView::new(cx, AppData::theme, AppData::active_task);
            })
            .round_box(AppData::theme)
            .height(Pixels(120.0))
            .background_color(AppData::theme.map(|theme| theme.background_light));
        })
        .gap(Pixels(6.0))
        .width(Pixels(320.0));
    })
    .gap(Pixels(6.0));
}
