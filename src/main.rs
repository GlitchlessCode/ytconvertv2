use components::{
    load_resources::load_resources, main_content::main_content, update_ffmpeg::update_ffmpeg,
    update_yt_dlp::update_yt_dlp,
};
use platform_dirs::AppDirs;
use vizia::prelude::*;
use ytconvertv2::{
    async_logic::{run_event_loop, AsyncAppEvent},
    config::{ConfigEvent, ConfigModel},
    data::TaskQueue,
    error::{error_popup, ErrorManager},
    models::{all::*, app::FfmpegInstallState},
    theme::Theme,
    views::all::*,
};

mod components;

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
            active_task: None,

            playlist_selected: false,

            current_location: None,
            yt_dlp_path: dependency_dir.clone().join("ytdlp"),
            ffmpeg_install_state: FfmpegInstallState::Unknown,

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
