#![cfg_attr(windows, windows_subsystem = "windows")]

use components::{
    about_popup::about_popup, license_popup::license_popup, load_resources::load_resources,
    main_content::main_content, settings_popup::settings_popup, update_ffmpeg::update_ffmpeg,
    update_yt_dlp::update_yt_dlp,
};
use platform_dirs::AppDirs;
use velopack::VelopackApp;
use velopack_updates::check_for_updates;
use vizia::prelude::*;
use ytconvertv2::{
    async_logic::{run_event_loop, AsyncAppEvent},
    config::{ConfigEvent, ConfigModel},
    data::{FfmpegInstallState, TaskQueue},
    error::{error_popup, ErrorManager},
    events::GlobalEvent,
    include_bytes_safe,
    licenses::parse_licenses,
    models::all::*,
    modifiers::ThemeModifiers,
    theme::Theme,
    views::all::*,
};

#[cfg(windows)]
use ytconvertv2::events::AppEvent;
#[cfg(windows)]
use ytconvertv2::helpers::ContextProxyExt;

mod components;
mod velopack_updates;

static CORNER_SIZE: Units = Pixels(6.0);
static APP_ICON: &[u8] = include_bytes_safe!("img", "ytconvertv2.png");

fn main() -> Result<(), ApplicationError> {
    VelopackApp::build().run();

    let rt = tokio::runtime::Runtime::new().expect("Failed to boot tokio runtime");
    let (async_tx, rx) = tokio::sync::mpsc::unbounded_channel();

    let tokio_handle = std::thread::spawn(move || {
        rt.block_on(async {
            run_event_loop(rx).await;
        });
    });

    let dirs = AppDirs::new(Some("ytconvertv2"), false);

    let (update_tx, update_rx) = std::sync::mpsc::channel();

    let inner_async_tx = async_tx.clone();
    let mut app = Application::new(move |cx| {
        let async_tx = inner_async_tx;
        #[cfg(windows)]
        {
            // Check for maximization
            let maximized_timer = cx.add_timer(Duration::from_millis(200), None, |ex, _action| {
                let mut proxy = ex.get_proxy();
                ex.modify_window(|window| {
                    proxy.emit_print_err(AppEvent::Maximized(window.is_maximized()));
                });
            });

            cx.start_timer(maximized_timer);
        }

        load_resources(cx);

        ErrorManager::default().build(cx);

        error_popup(cx, AppData::theme);

        cx.add_global_listener(move |ex, event| {
            event.take(|event, _meta| match event {
                GlobalEvent::CheckForUpdates => check_for_updates(ex),
                GlobalEvent::RestartForUpdate(manager, update) => {
                    if let Err(_) = update_tx.send((manager, update)) {
                        eprintln!("Failed to emit update manager for use in restart");
                    }

                    ex.emit(WindowEvent::WindowClose);
                }
            })
        });

        let dependency_dir = dirs
            .clone()
            .map(|dirs| dirs.data_dir)
            .expect("Should have data directory")
            .join("dependencies");

        // Build in root data
        AppData {
            // Build app theme
            theme: Theme::dark(),

            task_queue: TaskQueue::default(),
            active_task: None,

            playlist_selected: false,

            current_location: None,
            yt_dlp_path: dependency_dir.clone().join("ytdlp"),
            ffmpeg_path: dependency_dir.clone().join("ffmpeg"),
            ffmpeg_install_state: FfmpegInstallState::Unknown,

            show_about: false,

            show_settings: false,

            show_licenses: false,
            licenses: parse_licenses(),

            update_settings: Default::default(),

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

        about_popup(cx);
        settings_popup(cx);
        license_popup(cx);

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
                .on_background(AppData::theme)
                .with_border(AppData::theme)
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
    .decorations(false);

    match image::load_from_memory(APP_ICON) {
        Ok(icon) => app = app.icon(icon.width(), icon.height(), icon.into_bytes()),
        Err(err) => {
            eprintln!("Failed to load window icon due to error: {err}")
        }
    }

    let app_result = app.run();

    if let Err(_) = async_tx.send(AsyncAppEvent::Shutdown) {
        eprintln!("Failed to send shutdown signal to tokio runtime");
    }
    if let Err(_) = tokio_handle.join() {
        eprintln!("Failed to join on tokio runtime thread");
    }

    if let Ok((manager, update)) = update_rx.try_recv() {
        if let Err(_) = manager.apply_updates_and_restart(&update) {
            eprintln!("Failed to apply updates");
        }
    }

    return app_result;
}
