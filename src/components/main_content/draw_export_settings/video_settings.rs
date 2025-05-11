use super::*;

use vizia::icons::{ICON_RELOAD, ICON_SQUARE_ROUNDED_CHEVRONS_RIGHT_FILLED};
use ytconvertv2::{
    data::{ExportType, Task, VideoExportSettings},
    events::{AppVideoEvent, VideoExportSettingsEvent},
};

pub fn video_settings(cx: &mut Context) {
    labelled(cx, AppData::theme, "Youtube Video Link", |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppVideoData::link)
                .disabled(AppVideoData::searching)
                .text_overflow(TextOverflow::Ellipsis)
                .round_box(AppData::theme)
                .background_color(AppData::theme.map(|theme| theme.background_dark))
                .color(AppData::theme.map(|theme| theme.text_primary))
                .width(Stretch(1.0))
                .on_submit(move |ex, text, _| {
                    ex.emit(AppVideoEvent::LinkSubmit(text));
                    ex.focus_next();
                });

            Button::new(cx, |cx| Svg::new(cx, ICON_RELOAD))
                .disabled(AppVideoData::searching)
                .class("std-btn")
                .on_press(|ex| ex.emit(AppVideoEvent::Reset));
        })
        .height(Auto)
        .gap(Pixels(6.0))
        .alignment(Alignment::Center);
    })
    .height(Auto);

    labelled(cx, AppData::theme, "Preview", |cx| {
        HStack::new(cx, |cx| {
            ZStack::new(cx, |cx| {
                let fade_in = AppAnimationData::fade_in.get(cx);

                AnimatedBinding::new(cx, AppVideoData::thumbnail_generation, |cx, _| {
                    Image::new(cx, "video_thumb")
                        .class("thumbnail-img")
                        .corner_radius(Pixels(8.0))
                        .border_width(Pixels(1.0))
                        .border_color(AppData::theme.map(|theme| theme.border));
                })
                .set_anim_in(AnimationDef::new(
                    fade_in,
                    Duration::from_millis(400),
                    Duration::ZERO,
                ));

                VStack::new(cx, |cx| {
                    Label::new(
                        cx,
                        AppVideoData::video.map(|video| {
                            if let Some(video) = video {
                                format_seconds(video.duration())
                            } else {
                                "??:??".to_string()
                            }
                        }),
                    )
                    .corner_radius(Pixels(2.0))
                    .font_size("small")
                    .padding(Pixels(2.0))
                    .color("white")
                    .background_color("black");
                })
                .width(Pixels(256.0))
                .height(Pixels(144.0))
                .padding(Pixels(8.0))
                .alignment(Alignment::BottomRight);
            })
            .width(Pixels(256.0))
            .height(Pixels(144.0));

            VStack::new(cx, |cx| {
                Label::new(
                    cx,
                    AppVideoData::video.map(|video| {
                        if let Some(video) = video {
                            video.title().clone()
                        } else {
                            "No Video Selected".to_string()
                        }
                    }),
                )
                .width(Stretch(1.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis)
                .color(AppData::theme.map(|theme| theme.text_primary));
                Label::new(
                    cx,
                    AppVideoData::video.map(|video| {
                        if let Some(video) = video {
                            video.author().clone()
                        } else {
                            "".to_string()
                        }
                    }),
                )
                .font_size("small")
                .color(AppData::theme.map(|theme| theme.text_primary));
            });
        })
        .overflowx(Overflow::Hidden)
        .gap(Pixels(6.0));
    })
    .height(Pixels(164.0))
    .top(Pixels(6.0));

    labelled(cx, AppData::theme, "Settings", |cx| {
        Binding::new(cx, AppVideoData::video, |cx, video| {
            if video.get(cx).is_some() {
                video_export_settings(cx);
            } else {
                VStack::new(cx, |cx| {
                    Label::new(cx, "No video selected...")
                        .color(AppData::theme.map(|theme| theme.text_light))
                        .font_size("small");
                })
                .alignment(Alignment::Center)
                .size(Stretch(1.0));
            }
        });
    })
    .gap(Pixels(6.0))
    .top(Pixels(6.0));
}

fn video_export_settings(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Label::new(cx, "File Name")
            .font_size("small")
            .color(AppData::theme.map(|theme| theme.text_secondary));

        Textbox::new(
            cx,
            AppVideoData::export_settings.then(VideoExportSettings::title),
        )
        .text_wrap(false)
        .text_overflow(TextOverflow::Ellipsis)
        .width(Stretch(1.0))
        .background_color(AppData::theme.map(|theme| theme.background_dark))
        .color(AppData::theme.map(|theme| theme.text_primary))
        .round_box(AppData::theme)
        .on_submit(|ex, state, _| {
            ex.emit(AppVideoEvent::ExportSettingsEvent(
                VideoExportSettingsEvent::ChangeTitle(state),
            ))
        });
    })
    .alignment(Alignment::Center)
    .height(Auto)
    .gap(Pixels(6.0));

    HStack::new(cx, |cx| {
        Label::new(cx, "Format")
            .font_size("small")
            .color(AppData::theme.map(|theme| theme.text_secondary));

        ToggleButtonPanel::new(
            cx,
            AppData::theme,
            AppVideoData::export_settings
                .then(VideoExportSettings::export_type)
                .map(|es| match es {
                    ExportType::Audio(_) => false,
                    ExportType::Video(_) => true,
                }),
            |cx| Label::new(cx, "Audio").color(AppData::theme.map(|theme| theme.text_primary)),
            |cx| Label::new(cx, "Video").color(AppData::theme.map(|theme| theme.text_primary)),
        )
        .width(Stretch(2.0))
        .padding(Pixels(3.0))
        .round_box(AppData::theme)
        .background_color(AppData::theme.map(|theme| theme.background_dark))
        .on_choose(|ex, choice| match choice {
            ToggleButtonChoice::Left => ex.emit(AppVideoEvent::ExportSettingsEvent(
                VideoExportSettingsEvent::SetToVideo(false),
            )),
            ToggleButtonChoice::Right => ex.emit(AppVideoEvent::ExportSettingsEvent(
                VideoExportSettingsEvent::SetToVideo(true),
            )),
        });

        Dropdown::new(
            cx,
            |cx| {
                Button::new(cx, |cx| {
                    Label::new(
                        cx,
                        AppVideoData::export_settings
                            .then(VideoExportSettings::export_type)
                            .map(|export_type| export_type.to_string()),
                    )
                    .color(AppData::theme.map(|theme| theme.text_primary))
                })
                .round_box(AppData::theme)
                .background_color(AppData::theme.map(|theme| theme.background_dark))
                .on_press(|ex| ex.emit(PopupEvent::Open));
            },
            |cx| {
                let export_type = AppVideoData::export_settings
                    .then(VideoExportSettings::export_type)
                    .get(cx);

                for extension in export_type.formats() {
                    Label::new(cx, extension.to_string())
                        .padding(Pixels(6.0))
                        .class("dropdown-btn")
                        .cursor(CursorIcon::Hand)
                        .width(Stretch(1.0))
                        .on_press(move |ex| {
                            ex.emit(AppVideoEvent::ExportSettingsEvent(
                                VideoExportSettingsEvent::SetExtension(extension.clone()),
                            ));
                            ex.emit(PopupEvent::Close);
                        });
                }
            },
        )
        .width(Stretch(1.0));
    })
    .alignment(Alignment::Center)
    .height(Auto)
    .gap(Pixels(6.0));

    Button::new(cx, |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "Create Task")
                .color(AppData::theme.map(|theme| theme.primary))
                .z_index(1)
                .padding_left(Pixels(4.0))
                .padding_right(Pixels(4.0))
                .font_size("large")
                .font_weight(700);

            HStack::new(cx, |cx| {
                Svg::new(cx, ICON_SQUARE_ROUNDED_CHEVRONS_RIGHT_FILLED)
                    .fill(AppData::theme.map(|theme| theme.primary));
            })
            .position_type(PositionType::Absolute)
            .alignment(Alignment::Center)
            .z_index(3)
            .size(Percentage(100.0));

            Element::new(cx)
                .position_type(PositionType::Absolute)
                .background_color(AppData::theme.map(|theme| theme.background_dark))
                .class("task-submit-btn-panel")
                .left(Pixels(0.0))
                .top(Pixels(0.0))
                .width(Percentage(100.0))
                .height(Percentage(100.0))
                .z_index(2)
                .corner_radius(Pixels(4.0));
        })
        .width(Stretch(1.0))
        .position_type(PositionType::Relative)
    })
    .overflow(Overflow::Hidden)
    .width(Stretch(1.0))
    .round_box(AppData::theme)
    .padding(Pixels(0.0))
    .top(Pixels(12.0))
    .background_color(AppData::theme.map(|theme| theme.background_dark))
    .class("task-submit-btn")
    .pointer_events(true)
    .on_press(|ex| {
        let location = AppData::current_location.get(ex);
        let data = AppVideoData::video.get(ex);
        let settings = AppVideoData::export_settings.get(ex);
        match (location, data) {
            (Some(location), Some(data)) => {
                ex.emit(AppEvent::SubmitTask(Task::video(location, data, settings)));
                ex.emit(
                    Notification::new()
                        .message("Task Created Successfully")
                        .level(NotificationLevel::Success)
                        .build(),
                )
            }
            (location, data) => {
                if location.is_none() {
                    ex.emit(
                        Error::new()
                            .code(4)
                            .title("Export Location Unexpectedly None")
                            .severity(ErrorSeverity::Error)
                            .build(),
                    );
                }
                if data.is_none() {
                    ex.emit(
                        Error::new()
                            .code(5)
                            .title("Video Data Unexpectedly None")
                            .severity(ErrorSeverity::Error)
                            .build(),
                    );
                }
            }
        }

        ex.emit(AppVideoEvent::Reset);
    });
}
