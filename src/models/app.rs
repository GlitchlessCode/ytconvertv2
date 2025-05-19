use super::*;
use crate::{
    config::ConfigEvent,
    data::{
        license::License, task::TaskData, update_settings::UpdateSettings, ActiveTask,
        FfmpegInstallState, Task, TaskQueue,
    },
    error::{Error, ErrorSeverity},
    events::{
        update::{Update, UpdateTheme, UpdateUpdateSettings},
        AppEvent, GlobalEvent, ToolbarEvent,
    },
    export::{
        playlist::{PlaylistExporter, PlaylistProgress},
        video::{VideoExporter, VideoProgress},
    },
    helpers::ContextProxyExt,
    views::taskqueue::TaskEvent,
};
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Lens)]
pub struct AppData {
    pub theme: Theme,
    pub task_queue: TaskQueue,
    pub active_task: Option<ActiveTask>,

    pub playlist_selected: bool,

    pub current_location: Option<PathBuf>,
    pub yt_dlp_path: PathBuf,
    pub ffmpeg_path: PathBuf,
    pub ffmpeg_install_state: FfmpegInstallState, // TODO - Display this

    pub show_about: bool,

    pub show_settings: bool,

    pub show_licenses: bool,
    pub licenses: Option<Vec<License>>,

    pub update_settings: UpdateSettings,

    #[cfg(windows)]
    pub maximized: bool,
}

impl AppData {
    fn update_theme(&mut self, cx: &mut EventContext, update: &UpdateTheme) {
        match update {
            UpdateTheme::Primary(color) => self.theme.primary = *color,
            UpdateTheme::Border(color) => self.theme.border = *color,
            UpdateTheme::TextPrimary(color) => self.theme.text_primary = *color,
            UpdateTheme::TextSecondary(color) => self.theme.text_secondary = *color,
            UpdateTheme::TextLight(color) => self.theme.text_light = *color,
            UpdateTheme::Background(color) => self.theme.background = *color,
            UpdateTheme::BackgroundDark(color) => self.theme.background_dark = *color,
            UpdateTheme::BackgroundLight(color) => self.theme.background_light = *color,

            UpdateTheme::DarkDefault => self.theme = Theme::dark(),
            UpdateTheme::LightDefault => self.theme = Theme::light(),
        }

        cx.emit(ConfigEvent::SetTheme(self.theme.to_owned()));
    }

    fn update_update_settings(&mut self, cx: &mut EventContext, update: &UpdateUpdateSettings) {
        match update {
            UpdateUpdateSettings::ToggleAutoUpdates => {
                self.update_settings.auto_update = !self.update_settings.auto_update
            }
        }

        cx.emit(ConfigEvent::SetUpdateSettings(
            self.update_settings.to_owned(),
        ));
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event: &AppEvent, _meta| match event {
            #[cfg(windows)]
            AppEvent::Maximized(is_max) => self.maximized = *is_max,
            AppEvent::RequestLocationChange => {
                let current_location = self.current_location.clone();
                cx.spawn(|cxp| {
                    let mut dialog = FileDialog::new().set_title("Choose Export Location");
                    if let Some(path) = current_location {
                        dialog = dialog.set_directory(path);
                    }
                    if let Some(path) = dialog.pick_folder() {
                        cxp.emit_print_err(AppEvent::SetNewLocation(path));
                    }
                });
            }

            AppEvent::SetNewLocation(new_path) => {
                self.current_location = Some(new_path.to_owned());
                cx.emit(ConfigEvent::SetExportPath(new_path.to_owned()));
            }

            AppEvent::SetFfmpegInstallState(state) => {
                self.ffmpeg_install_state = state.clone();
            }

            AppEvent::SubmitTask(task) => {
                if self.active_task.is_some() {
                    self.task_queue.push(task.clone());
                } else {
                    cx.emit(TaskEvent::SetActive(task.clone()))
                }
            }

            AppEvent::SetShowAbout(state) => {
                self.show_about = *state;
            }

            AppEvent::SetShowSettings(state) => {
                self.show_settings = *state;
            }

            AppEvent::SetShowLicense(state) => {
                self.show_licenses = *state;
            }

            AppEvent::ToggleVideo => {
                self.playlist_selected = false;
            }

            AppEvent::TogglePlaylist => {
                self.playlist_selected = true;
            }

            #[allow(unreachable_patterns)]
            _ => (),
        });

        event.take(|event, _meta| match event {
            TaskEvent::Remove(index) => self.task_queue.remove(index),
            TaskEvent::SetActive(task) => {
                if self.ffmpeg_install_state == FfmpegInstallState::Installed {
                    self.active_task = Some(ActiveTask::new(&task));
                    let yt_dlp_path = self.yt_dlp_path.clone();
                    let ffmpeg_path = self.ffmpeg_path.clone();

                    let (yt_dlp_exec, ffmpeg_exec) = if cfg!(windows) {
                        ("yt-dlp.exe", "ffmpeg.exe")
                    } else {
                        ("yt-dlp", "ffmpeg")
                    };

                    cx.spawn(move |cx| {
                        handle_task(
                            cx,
                            task,
                            yt_dlp_path.join(yt_dlp_exec),
                            ffmpeg_path.join(ffmpeg_exec),
                        )
                    });
                } else if self.ffmpeg_install_state == FfmpegInstallState::Installing
                    || self.ffmpeg_install_state == FfmpegInstallState::Updating
                {
                    cx.emit(
                        Notification::new()
                            .message("Please wait for ffmpeg to install")
                            .level(NotificationLevel::Warning)
                            .build(),
                    );
                    cx.emit(
                        Error::new()
                            .title("Installing ffmpeg Warning")
                            .code(6)
                            .severity(ErrorSeverity::Warning)
                            .description("ffmpeg is still installing, please wait.")
                            .build(),
                    );
                } else {
                    cx.emit(
                        Notification::new()
                            .message("ffmpeg could not be found")
                            .level(NotificationLevel::Error)
                            .build(),
                    );
                    cx.emit(
                        Error::new()
                            .title("No Installation of ffmpeg")
                            .code(7)
                            .severity(ErrorSeverity::Error)
                            .description("Previously failed to install ffmpeg, cannot be used.")
                            .build(),
                    );
                }
            }
            TaskEvent::UpdateActive(active_task) => {
                if let Some(ref mut task) = self.active_task {
                    *task = active_task;
                }
            }
            TaskEvent::FinishActive => {
                cx.schedule_emit(
                    TaskEvent::MoveToNext,
                    Instant::now() + Duration::from_secs(1),
                );
            }
            TaskEvent::MoveToNext => {
                if let Some(task) = self.task_queue.pull() {
                    cx.emit(TaskEvent::SetActive(task));
                } else {
                    self.active_task = None;
                }
            }
        });

        event.map(|event, _meta| match event {
            ToolbarEvent::ShowAbout => cx.emit(AppEvent::SetShowAbout(true)),
            ToolbarEvent::ShowSettings => cx.emit(AppEvent::SetShowSettings(true)),
            ToolbarEvent::ShowLicense => cx.emit(AppEvent::SetShowLicense(true)),

            ToolbarEvent::OpenDocumentationPage => {
                if let Err(_) = open::that("https://github.com/GlitchlessCode/ytconvertv2/wiki") {
                    eprintln!("Error opening url");
                }
            }
            ToolbarEvent::OpenIssuesPage => {
                if let Err(_) = open::that("https://github.com/GlitchlessCode/ytconvertv2/issues") {
                    eprintln!("Error opening url");
                }
            }
            _ => (),
        });

        event.map(|event, _meta| match event {
            Update::Theme(update) => self.update_theme(cx, update),
            Update::UpdateSettings(update) => self.update_update_settings(cx, update),

            #[allow(unreachable_patterns)]
            _ => (),
        });

        event.map(|event, _meta| match event {
            ConfigEvent::ConfigSetup {
                location,
                theme,
                update_settings,
            } => {
                self.current_location = location.to_owned();
                self.theme = theme.to_owned();
                self.update_settings = update_settings.to_owned();

                if self.update_settings.auto_update {
                    cx.emit(GlobalEvent::CheckForUpdates);
                }
            }
            _ => (),
        });
    }
}

fn handle_task(cx: &mut ContextProxy, task: Task, yt_dlp_path: PathBuf, ffmpeg_path: PathBuf) {
    let export_location = task.location();
    match task.data() {
        TaskData::Video(data, settings) => {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut active_task = ActiveTask::new(&task);
            active_task.set_downloading(false);

            cx.spawn(move |cx| {
                while let Ok(progress) = rx.recv() {
                    match progress {
                        VideoProgress::Progress(progress) => {
                            active_task.set_video_progress(progress);
                            cx.emit_print_err(TaskEvent::UpdateActive(active_task.clone()));
                        }
                        VideoProgress::Done => {
                            break;
                        }
                    }
                }

                active_task.set_video_progress(1.0);
                cx.emit_print_err(TaskEvent::UpdateActive(active_task));
                cx.emit_print_err(TaskEvent::FinishActive);
            });

            let exporter = VideoExporter::builder()
                .export_location(export_location.to_owned())
                .data(data.to_owned())
                .settings(settings.to_owned())
                .yt_dlp_path(yt_dlp_path)
                .ffmpeg_path(ffmpeg_path)
                .ffmpeg_progress(move |progress| {
                    if let Err(err) = tx.send(progress) {
                        eprintln!("Failed to submit event to mpsc channel due to error: {err}");
                    }
                })
                .build();

            handle_video_task(cx, exporter, task.clone());
        }
        TaskData::Playlist(data, settings) => {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut active_task = ActiveTask::new(&task);

            cx.spawn(move |cx| {
                while let Ok(progress) = rx.recv() {
                    match progress {
                        PlaylistProgress::StartVideo(settings) => {
                            active_task.set_downloading(true);
                            active_task.set_video_progress(0.0);
                            active_task.set_active_video(settings);
                            cx.emit_print_err(TaskEvent::UpdateActive(active_task.clone()));
                        }
                        PlaylistProgress::VideoProgress(progress) => {
                            active_task.set_downloading(false);
                            active_task.set_video_progress(progress);
                            cx.emit_print_err(TaskEvent::UpdateActive(active_task.clone()));
                        }
                        PlaylistProgress::VideoComplete(index) => {
                            active_task.set_video_progress(1.0);
                            active_task.set_finished_count(index + 1);
                            cx.emit_print_err(TaskEvent::UpdateActive(active_task.clone()));
                        }
                        PlaylistProgress::Done => {
                            break;
                        }
                    }
                }

                cx.emit_print_err(TaskEvent::FinishActive);
            });

            let exporter = PlaylistExporter::builder()
                .export_location(export_location.to_owned())
                .data(data.to_owned())
                .settings(settings.to_owned())
                .yt_dlp_path(yt_dlp_path)
                .ffmpeg_path(ffmpeg_path)
                .ffmpeg_progress(move |progress| {
                    if let Err(err) = tx.send(progress) {
                        eprintln!("Failed to submit event to mpsc channel due to error: {err}");
                    }
                })
                .build();

            handle_playlist_task(cx, exporter);
        }
    }
}

fn handle_video_task(
    cx: &mut ContextProxy,
    exporter: VideoExporter<crate::export::video::Unused>,
    task: Task,
) {
    let result = exporter
        .fetch_video()
        .inspect(|_| {
            let mut active_task = ActiveTask::new(&task);
            active_task.set_downloading(false);
            cx.emit_print_err(TaskEvent::UpdateActive(active_task));
        })
        .and_then(|exporter| exporter.grab_filepath())
        .and_then(|exporter| exporter.final_export());

    if let Err(err) = result {
        cx.emit_print_err(Error::from(err));
    }
}

fn handle_playlist_task(
    cx: &mut ContextProxy,
    exporter: PlaylistExporter<crate::export::playlist::Unused>,
) {
    let result = exporter.process_videos();

    if let Err(err) = result {
        cx.emit_print_err(Error::from(err));
    }
}
