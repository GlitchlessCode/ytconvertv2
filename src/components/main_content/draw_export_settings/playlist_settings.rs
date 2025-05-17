use super::*;

use vizia::icons::{
    ICON_EDIT, ICON_HEXAGON_MINUS, ICON_HEXAGON_PLUS, ICON_RELOAD,
    ICON_SQUARE_ROUNDED_CHEVRONS_RIGHT_FILLED,
};
use ytconvertv2::{
    data::{ExportType, PlaylistExportSettings, Task},
    events::{AppPlaylistEvent, PlaylistExportSettingsEvent, PlaylistVideoSettingsEvent},
};

pub fn playlist_settings(cx: &mut Context) {
    labelled(cx, AppData::theme, "Youtube Playlist Link", |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppPlaylistData::link)
                .disabled(AppPlaylistData::searching)
                .text_overflow(TextOverflow::Ellipsis)
                .round_box(AppData::theme)
                .on_background_dark(AppData::theme)
                .with_text_primary(AppData::theme)
                .width(Stretch(1.0))
                .on_submit(move |ex, text, _| {
                    ex.emit(AppPlaylistEvent::LinkSubmit(text));
                    ex.focus_next();
                });

            Button::new(cx, |cx| {
                Svg::new(cx, ICON_RELOAD).fill(AppData::theme.map(|theme| theme.primary))
            })
            .disabled(AppPlaylistData::searching)
            .class("std-btn")
            .on_press(|ex| ex.emit(AppPlaylistEvent::Reset));
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

                AnimatedBinding::new(cx, AppPlaylistData::thumbnail_generation, |cx, _| {
                    Image::new(cx, "playlist_thumb")
                        .class("thumbnail-img")
                        .corner_radius(Pixels(8.0))
                        .with_border(AppData::theme);
                })
                .set_anim_in(AnimationDef::new(
                    fade_in,
                    Duration::from_millis(400),
                    Duration::ZERO,
                ));

                VStack::new(cx, |cx| {
                    Label::new(
                        cx,
                        AppPlaylistData::playlist.map(|playlist| {
                            if let Some(playlist) = playlist {
                                format!("{} Videos", playlist.video_count())
                            } else {
                                "????".to_string()
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
                    AppPlaylistData::playlist.map(|playlist| {
                        if let Some(playlist) = playlist {
                            playlist.title().clone()
                        } else {
                            "No Playlist Selected".to_string()
                        }
                    }),
                )
                .width(Stretch(1.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis)
                .with_text_primary(AppData::theme);
                Label::new(
                    cx,
                    AppPlaylistData::playlist.map(|playlist| {
                        if let Some(playlist) = playlist {
                            playlist.creator().clone()
                        } else {
                            "".to_string()
                        }
                    }),
                )
                .font_size("small")
                .with_text_primary(AppData::theme);
            });
        })
        .overflowx(Overflow::Hidden)
        .gap(Pixels(6.0));
    })
    .height(Pixels(164.0))
    .top(Pixels(6.0));

    labelled(cx, AppData::theme, "Settings", |cx| {
        Binding::new(cx, AppPlaylistData::playlist, |cx, playlist| {
            if playlist.get(cx).is_some() {
                playlist_export_settings(cx);
            } else {
                VStack::new(cx, |cx| {
                    Label::new(cx, "No playlist selected...")
                        .with_text_light(AppData::theme)
                        .font_size("small");
                })
                .alignment(Alignment::Center)
                .size(Stretch(1.0));
            }
        });

        ScrollView::new(cx, |cx| {
            videos_box(cx);
        })
        .overflow(Overflow::Hidden)
        .class("scroll-gap")
        .alignment(Alignment::BottomCenter)
        .size(Stretch(1.0))
        .display(AppPlaylistData::playlist.map(|playlist| playlist.is_some()));
    })
    .gap(Pixels(6.0))
    .top(Pixels(6.0));
}

fn playlist_export_settings(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Label::new(cx, "Folder Name")
            .font_size("small")
            .with_text_secondary(AppData::theme);

        Textbox::new(
            cx,
            AppPlaylistData::export_settings.then(PlaylistExportSettings::title),
        )
        .text_wrap(false)
        .text_overflow(TextOverflow::Ellipsis)
        .width(Stretch(1.0))
        .on_background_dark(AppData::theme)
        .with_text_primary(AppData::theme)
        .round_box(AppData::theme)
        .on_submit(|ex, state, _| {
            ex.emit(AppPlaylistEvent::ExportSettingsEvent(
                PlaylistExportSettingsEvent::ChangeTitle(state),
            ))
        });
    })
    .alignment(Alignment::Center)
    .height(Auto)
    .gap(Pixels(6.0));

    HStack::new(cx, |cx| {
        Label::new(cx, "Format")
            .font_size("small")
            .with_text_secondary(AppData::theme);

        ToggleButtonPanel::new(
            cx,
            AppData::theme,
            AppPlaylistData::export_settings
                .then(PlaylistExportSettings::export_type)
                .map(|es| match es {
                    ExportType::Audio(_) => false,
                    ExportType::Video(_) => true,
                }),
            |cx| Label::new(cx, "Audio").with_text_primary(AppData::theme),
            |cx| Label::new(cx, "Video").with_text_primary(AppData::theme),
        )
        .width(Stretch(2.0))
        .padding(Pixels(3.0))
        .round_box(AppData::theme)
        .on_background_dark(AppData::theme)
        .on_choose(|ex, choice| match choice {
            ToggleButtonChoice::Left => ex.emit(AppPlaylistEvent::ExportSettingsEvent(
                PlaylistExportSettingsEvent::SetToVideo(false),
            )),
            ToggleButtonChoice::Right => ex.emit(AppPlaylistEvent::ExportSettingsEvent(
                PlaylistExportSettingsEvent::SetToVideo(true),
            )),
        });

        Dropdown::new(
            cx,
            |cx| {
                Button::new(cx, |cx| {
                    Label::new(
                        cx,
                        AppPlaylistData::export_settings
                            .then(PlaylistExportSettings::export_type)
                            .map(|export_type| export_type.to_string()),
                    )
                    .with_text_primary(AppData::theme)
                })
                .round_box(AppData::theme)
                .on_background_dark(AppData::theme)
                .on_press(|ex| ex.emit(PopupEvent::Open));
            },
            |cx| {
                let export_type = AppPlaylistData::export_settings
                    .then(PlaylistExportSettings::export_type)
                    .get(cx);

                for extension in export_type.formats() {
                    Label::new(cx, extension.to_string())
                        .padding(Pixels(6.0))
                        .class("dropdown-btn")
                        .cursor(CursorIcon::Hand)
                        .width(Stretch(1.0))
                        .on_press(move |ex| {
                            ex.emit(AppPlaylistEvent::ExportSettingsEvent(
                                PlaylistExportSettingsEvent::SetExtension(extension.clone()),
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
                .on_background_dark(AppData::theme)
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
    .on_background_dark(AppData::theme)
    .class("task-submit-btn")
    .pointer_events(true)
    .on_press(|ex| {
        let location = AppData::current_location.get(ex);
        let data = AppPlaylistData::playlist.get(ex);
        let settings = AppPlaylistData::export_settings.get(ex);
        match (location, data) {
            (Some(location), Some(data)) => {
                ex.emit(AppEvent::SubmitTask(Task::playlist(
                    location, data, settings,
                )));
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

        ex.emit(AppPlaylistEvent::Reset);
    });
}

fn videos_box(cx: &mut Context) {
    Collapsible::new(
        cx,
        |cx| {
            Label::new(cx, "Videos").with_text_primary(AppData::theme);
        },
        |cx| {
            Binding::new(cx, AppPlaylistData::playlist, |cx, data| {
                let data = match data.get(cx) {
                    Some(data) => data,
                    None => return,
                };

                for (idx, (_, settings)) in data.iter().enumerate() {
                    let include = settings.include;
                    let editing = settings.editing;
                    let title = settings.title.clone();

                    HStack::new(cx, |cx| {
                        Button::new(cx, move |cx| {
                            if include {
                                Svg::new(cx, ICON_HEXAGON_MINUS)
                                    .fill(AppData::theme.map(|theme| theme.background_dark))
                            } else {
                                Svg::new(cx, ICON_HEXAGON_PLUS)
                                    .fill(AppData::theme.map(|theme| theme.primary))
                            }
                        })
                        .padding(Pixels(6.0))
                        .round_box(AppData::theme)
                        .background_color(AppData::theme.map(move |theme| {
                            if include {
                                theme.primary
                            } else {
                                theme.background_dark
                            }
                        }))
                        .on_press(move |ex| {
                            ex.emit(AppPlaylistEvent::PlaylistVideoSettingsEvent(
                                idx,
                                PlaylistVideoSettingsEvent::SetInclusion(!include),
                            ))
                        });

                        if !editing {
                            Label::new(cx, title)
                                .color(AppData::theme.map(move |theme| {
                                    if include {
                                        theme.text_primary
                                    } else {
                                        theme.text_secondary
                                    }
                                }))
                                .width(Stretch(1.0))
                                .text_overflow(TextOverflow::Ellipsis);

                            Button::new(cx, |cx| {
                                Svg::new(cx, ICON_EDIT)
                                    .fill(AppData::theme.map(|theme| theme.primary))
                            })
                            .round_box(AppData::theme)
                            .on_background_light(AppData::theme)
                            .on_press(move |ex| {
                                ex.emit(AppPlaylistEvent::PlaylistVideoSettingsEvent(
                                    idx,
                                    PlaylistVideoSettingsEvent::StartEdit,
                                ))
                            });
                        } else {
                            let textbox = Textbox::new(
                                cx,
                                AppPlaylistData::playlist.map(move |data| {
                                    if let Some((_, settings)) =
                                        data.as_ref().and_then(|data| data.video(idx))
                                    {
                                        settings.title.clone()
                                    } else {
                                        "".to_string()
                                    }
                                }),
                            )
                            .width(Stretch(1.0))
                            .with_text_primary(AppData::theme)
                            .text_wrap(false)
                            .text_overflow(TextOverflow::Ellipsis)
                            .round_box(AppData::theme)
                            .on_background_dark(AppData::theme)
                            .on_submit(move |ex, text, _| {
                                ex.emit(AppPlaylistEvent::PlaylistVideoSettingsEvent(
                                    idx,
                                    PlaylistVideoSettingsEvent::EndEdit(text),
                                ))
                            })
                            .on_focus_out(|ex| ex.emit(TextEvent::Submit(false)));
                            let textbox = textbox.entity();

                            EventContext::new_with_current(cx, textbox).focus();
                        }
                    })
                    .alignment(Alignment::Left)
                    .height(Auto)
                    .width(Stretch(1.0))
                    .gap(Pixels(6.0));
                }
            });
        },
    )
    .round_box(AppData::theme)
    .on_background(AppData::theme);
}

// https://www.youtube.com/watch?v=LJLgAX85CbM&list=PLt9E_E6JvXA1gfikKmURgo71PIlWGLAzl&pp=gAQB
