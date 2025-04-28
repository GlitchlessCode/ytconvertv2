use platform_dirs::AppDirs;
use vizia::{
    icons::{ICON_FOLDER, ICON_LIST, ICON_PLAYER_PLAY, ICON_RELOAD},
    prelude::*,
};
use ytconvertv2::{
    async_logic::{run_event_loop, AsyncAppEvent},
    config::{ConfigEvent, ConfigModel},
    data::TaskQueue,
    error::{error_popup, Error, ErrorManager, ErrorSeverity},
    helpers::{format_seconds, labelled, ContextProxyExt},
    include_bytes_safe,
    models::{all::*, video::LARGE_PLACEHOLDER},
    modifiers::ViewModifiers,
    theme::Theme,
    views::all::*,
};

static CORNER_SIZE: Units = Pixels(6.0);

fn main() -> Result<(), ApplicationError> {
    let rt = tokio::runtime::Runtime::new().expect("Failed to boot tokio runtime");
    let (async_tx, rx) = tokio::sync::mpsc::unbounded_channel();

    let tokio_handle = std::thread::spawn(move || {
        rt.block_on(async {
            run_event_loop(rx).await;
        });
    });

    let dirs = AppDirs::new(Some("ytconvertv2"), false);

    let inner_async_tx = async_tx.clone();
    let app_result = Application::new(move |cx| {
        let async_tx = inner_async_tx;
        #[cfg(windows)]
        {
            // Check for maximization
            let maximized_timer = cx.add_timer(Duration::from_millis(200), None, |ex, _action| {
                let mut proxy = ex.get_proxy();
                ex.modify_window(|window| {
                    if let Err(error) = proxy.emit(AppEvent::Maximized(window.is_maximized())) {
                        eprintln!("{error}")
                    }
                });
            });

            cx.start_timer(maximized_timer);
        }

        load_resources(cx);

        ErrorManager::default().build(cx);

        error_popup(cx, AppData::theme);

        let dependency_dir = dirs
            .clone()
            .map(|dirs| dirs.data_dir)
            .expect("Should have data directory")
            .join("dependencies");

        // Build in root data
        AppData {
            // Build app theme
            theme: Theme::builder()
                .background("#252525")
                .background_dark("#151515")
                .background_light("#2f2f2f")
                .border("#444444")
                .primary("gold")
                .text_primary("#eeeeee")
                .text_secondary("#aaaaaa")
                .text_light("#575757")
                .build(),

            task_queue: TaskQueue::default(),

            playlist_selected: false,

            current_location: None,
            yt_dlp_path: dependency_dir.clone().join("ytdlp"),

            #[cfg(windows)]
            maximized: false,
        }
        .build(cx); // Build the data into the app

        // Build Animation model in
        AppAnimationData {
            fade_in: cx.add_animation(
                AnimationBuilder::new()
                    .keyframe(0.0, |kf| kf.opacity(0.0))
                    .keyframe(1.0, |kf| kf.opacity(1.0)),
            ),
            fade_out: cx.add_animation(
                AnimationBuilder::new()
                    .keyframe(0.0, |kf| kf.opacity(1.0))
                    .keyframe(1.0, |kf| kf.opacity(0.0)),
            ),
        }
        .build(cx);

        // Build Config model in
        ConfigModel::open_config_path(dirs).build(cx);

        // Build notification model in
        NotificationService::new(cx);

        cx.emit(ConfigEvent::RequestSetup);

        let submission_tx = async_tx.clone();
        update_yt_dlp(cx, submission_tx);

        update_ffmpeg(cx);

        // Layer Stack
        ZStack::new(cx, |cx| {
            // Main element stack
            #[allow(unused)]
            let mut window = VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    main_content(cx);
                })
                .background_color(AppData::theme.map(|theme| theme.background))
                .border_color(AppData::theme.map(|theme| theme.border))
                .border_width(Pixels(1.0))
                .padding(CORNER_SIZE)
                .size(Percentage(100.0))
                .gap(Pixels(6.0))
                .corner_radius(CORNER_SIZE);

                NotificationPopups::new(cx, AppData::theme);
            })
            .position_type(PositionType::Relative);

            #[cfg(windows)]
            {
                window.padding(Pixels(4.0));
                ResizerGroup::new(cx).display(AppData::maximized.map(|max| !max));
            }
        });
    })
    .min_inner_size(Some((600, 400)))
    .inner_size((800, 600))
    .transparent(true)
    .decorations(false)
    .run();

    if let Err(_) = async_tx.send(AsyncAppEvent::Shutdown) {
        eprintln!("Failed to send shutdown signal to tokio runtime");
    }
    if let Err(_) = tokio_handle.join() {
        eprintln!("Failed to join on tokio runtime thread");
    }

    return app_result;
}

fn load_resources(cx: &mut Context) {
    // Add font
    let bytes = include_bytes_safe!("font", "Quicksand-Medium.ttf");
    cx.add_font_mem(bytes);

    // Add stylesheet
    cx.add_stylesheet(include_style!("src/style/style.css"))
        .expect("Failed to load stylesheet");

    // Add default thumbnails
    cx.load_image(
        "video_thumb",
        LARGE_PLACEHOLDER,
        ImageRetentionPolicy::Forever,
    );

    cx.load_image(
        "playlist_thumb",
        LARGE_PLACEHOLDER,
        ImageRetentionPolicy::Forever,
    );
}

fn update_yt_dlp(
    cx: &mut Context,
    submission_tx: tokio::sync::mpsc::UnboundedSender<AsyncAppEvent>,
) {
    let yt_dlp = AppData::yt_dlp_path.get(cx);
    cx.emit(
        Notification::new()
            .message("Updating yt-dlp...")
            .level(NotificationLevel::Info)
            .build(),
    );
    cx.spawn(move |cx| {
        let (tx, rx) = tokio::sync::oneshot::channel();
        if let Err(_) = submission_tx.send(AsyncAppEvent::UpdateYtdlp(yt_dlp, tx)) {
            if let Err(_) = cx.emit(
                Error::new()
                    .code(2)
                    .description("Failed to send yt-dlp download signal to tokio task")
                    .title("MPSC Channel Send Failure")
                    .severity(ErrorSeverity::Error)
                    .build(),
            ) {
                eprintln!("Could not submit error event, failed to send event");
            }
        }
        match rx.blocking_recv() {
            Err(_) => {
                if let Err(_) = cx.emit(
                    Error::new()
                        .code(1)
                        .description("Failed to recieve from yt-dlp downloader oneshot channel")
                        .title("Oneshot Channel Reciever Failure")
                        .severity(ErrorSeverity::Error)
                        .build(),
                ) {
                    eprintln!("Could not submit error event, failed to send event");
                }
            }
            Ok(result) => {
                if let Err(error) = result {
                    let msg = format!("Failed to download yt-dlp due to error: {error}");
                    if let Err(_) = cx.emit(
                        Error::new()
                            .code(300)
                            .description(msg)
                            .title("yt-dlp Download Failure")
                            .severity(ErrorSeverity::Error)
                            .build(),
                    ) {
                        eprintln!("Could not submit error event, failed to send event");
                    }
                } else {
                    if let Err(_) = cx.emit(
                        Notification::new()
                            .message("Updated yt-dlp successfully")
                            .level(NotificationLevel::Success)
                            .build(),
                    ) {
                        eprintln!("Could not submit error event, failed to send event");
                    };
                }
            }
        }
    });
}

fn update_ffmpeg(cx: &mut Context) {
    use ffmpeg_sidecar::command::ffmpeg_is_installed;

    cx.emit(
        Notification::new()
            .message("Checking ffmpeg for updates")
            .level(NotificationLevel::Info)
            .build(),
    );

    cx.spawn(|cx| {
        match update_ffmpeg_inner(cx) {
            Ok(_) => (),
            Err(_) => cx.emit_print_err(
                Error::new()
                    .code(3)
                    .title("Dependency ffmpeg Installation Failed")
                    .severity(ErrorSeverity::Error)
                    .description("Failed to install ffmpeg due to an unknown error.")
                    .build(),
            ),
        }

        // Error, ffmpeg should now be installed
        if !ffmpeg_is_installed() {
            let msg =
                "Cannot detect an ffmpeg installation after completing the installation process.";

            cx.emit_print_err(
                Error::new()
                    .code(4)
                    .title("Dependency ffmpeg Unexpectedly Missing")
                    .severity(ErrorSeverity::Error)
                    .description(msg)
                    .footer(|cx| install_ffmpeg_footer(cx))
                    .build(),
            );
            cx.emit_print_err(
                Notification::new()
                    .message("Please try installing ffmpeg manually")
                    .level(NotificationLevel::Error)
                    .build(),
            );
        } else {
            cx.emit_print_err(
                Notification::new()
                    .message("Updated ffmpeg successfully")
                    .level(NotificationLevel::Success)
                    .build(),
            );
        }
    });
}

fn install_ffmpeg_footer(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Download ffmpeg...")).on_press(|_| {
            if let Err(_) = open::that("https://www.ffmpeg.org/download.html") {
                eprintln!("Error opening url");
            }
        });
    });
}

fn update_ffmpeg_inner(cx: &mut ContextProxy) -> Result<(), ()> {
    use ffmpeg_sidecar::command::ffmpeg_is_installed;
    use ffmpeg_sidecar::download::{
        check_latest_version, download_ffmpeg_package, ffmpeg_download_url, unpack_ffmpeg,
    };
    use ffmpeg_sidecar::paths::sidecar_dir;
    use ffmpeg_sidecar::version::ffmpeg_version;

    if ffmpeg_is_installed() {
        let current_version = ffmpeg_version().map_err(|_| ())?;
        let latest_version = check_latest_version().map_err(|_| ())?;

        if !current_version.starts_with(&latest_version) {
            cx.emit_print_err(
                Notification::new()
                    .message("Updating ffmpeg...")
                    .level(NotificationLevel::Info)
                    .build(),
            );
            let download_url = ffmpeg_download_url().map_err(|_| ())?;
            let destination = sidecar_dir().map_err(|_| ())?;
            let archive_path =
                download_ffmpeg_package(download_url, &destination).map_err(|_| ())?;
            unpack_ffmpeg(&archive_path, &destination).map_err(|_| ())?;
        } else {
            cx.emit_print_err(
                Notification::new()
                    .message("ffmpeg is already up to date")
                    .level(NotificationLevel::Info)
                    .build(),
            );
        }
    } else {
        cx.emit_print_err(
            Notification::new()
                .message("Updating ffmpeg...")
                .level(NotificationLevel::Info)
                .build(),
        );
        let download_url = ffmpeg_download_url().map_err(|_| ())?;
        let destination = sidecar_dir().map_err(|_| ())?;
        let archive_path = download_ffmpeg_package(download_url, &destination).map_err(|_| ())?;
        unpack_ffmpeg(&archive_path, &destination).map_err(|_| ())?;
    }

    Ok(())
}

fn main_content(cx: &mut Context) {
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
                //ProgressBar goes here
            })
            .round_box(AppData::theme)
            .height(Pixels(120.0))
            .background_color(AppData::theme.map(|theme| theme.background_light));
        })
        .gap(Pixels(6.0))
        .width(Pixels(290.0));
    })
    .gap(Pixels(6.0));
}

fn draw_export_settings(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ToggleButtonPanel::new(
            cx,
            AppData::theme,
            AppData::playlist_selected,
            |cx| {
                HStack::new(cx, |cx| {
                    Svg::new(cx, ICON_PLAYER_PLAY);
                    Label::new(cx, "Video")
                        .font_size("large")
                        .color(AppData::theme.map(|theme| theme.text_primary));
                })
                .alignment(Alignment::Center)
            },
            |cx| {
                HStack::new(cx, |cx| {
                    Svg::new(cx, ICON_LIST);
                    Label::new(cx, "Playlist")
                        .font_size("large")
                        .color(AppData::theme.map(|theme| theme.text_primary));
                })
                .alignment(Alignment::Center)
            },
        )
        .on_choose(|ex, side| match side {
            ToggleButtonChoice::Left => ex.emit(AppEvent::ToggleVideo),
            ToggleButtonChoice::Right => ex.emit(AppEvent::TogglePlaylist),
        })
        .round_box(AppData::theme)
        .padding(Pixels(3.0))
        .background_color(AppData::theme.map(|theme| theme.background_dark));

        let anim_out = cx.add_animation(
            AnimationBuilder::new()
                .keyframe(0.0, |kf| kf.opacity(1.0).scale((1.0, 1.0)))
                .keyframe(1.0, |kf| kf.opacity(0.0).scale((0.983, 0.983))),
        );
        let anim_in = cx.add_animation(
            AnimationBuilder::new()
                .keyframe(0.0, |kf| kf.opacity(0.0).scale((0.983, 0.983)))
                .keyframe(1.0, |kf| kf.opacity(1.0).scale((1.0, 1.0))),
        );

        AppVideoData::new(AppData::yt_dlp_path.get(cx).join("yt-dlp.exe")).build(cx);

        AnimatedBinding::new(cx, AppData::playlist_selected, |cx, pl_selected| {
            VStack::new(cx, move |cx| {
                if pl_selected.get(cx) {
                    playlist_settings(cx);
                } else {
                    video_settings(cx);
                }
            })
            .gap(Pixels(6.0))
            .padding_top(Pixels(10.0));
        })
        .set_anim_out(AnimationDef::new(
            anim_out,
            Duration::from_millis(150),
            Duration::ZERO,
        ))
        .set_anim_in(AnimationDef::new(
            anim_in,
            Duration::from_millis(150),
            Duration::ZERO,
        ));
    })
    .round_box(AppData::theme)
    .background_color(AppData::theme.map(|theme| theme.background_light))
    .padding(Pixels(6.0));
}

fn video_settings(cx: &mut Context) {
    labelled(cx, AppData::theme, "Youtube Link", |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppVideoData::link)
                .round_box(AppData::theme)
                .background_color(AppData::theme.map(|theme| theme.background_dark))
                .color(AppData::theme.map(|theme| theme.text_primary))
                .width(Stretch(1.0))
                .on_submit(move |ex, text, _| ex.emit(AppVideoEvent::LinkSubmit(text)));

            Button::new(cx, |cx| Svg::new(cx, ICON_RELOAD))
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
                        .corner_radius(Pixels(8.0));
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

    labelled(cx, AppData::theme, "Settings", |cx| {}).top(Pixels(6.0));
}

fn playlist_settings(cx: &mut Context) {
    Label::new(cx, "Playlist").color("white");
}
