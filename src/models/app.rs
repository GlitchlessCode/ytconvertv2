use super::*;
use crate::{
    config::ConfigEvent,
    data::{task::TaskData, ActiveTask, FfmpegInstallState, Task, TaskQueue},
    error::{Error, ErrorSeverity},
    events::AppEvent,
    export::video::{Unused, VideoExporter, VideoProgress},
    helpers::ContextProxyExt,
    views::task_queue::TaskEvent,
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
    pub ffmpeg_install_state: FfmpegInstallState, // TODO - Display this

    #[cfg(windows)]
    pub maximized: bool,
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
                        if let Err(error) = cxp.emit(AppEvent::SetNewLocation(path)) {
                            eprintln!("Could not emit new location request, context proxy event emission failed due to error: {error}")
                        }
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
                    self.active_task = Some(ActiveTask::new(task.clone()));
                    let yt_dlp_path = self.yt_dlp_path.clone();

                    let executable = if cfg!(windows) {
                        "yt-dlp.exe"
                    } else {
                        "yt-dlp"
                    };

                    cx.spawn(move |cx| handle_task(cx, task, yt_dlp_path.join(executable)));
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
            ConfigEvent::ConfigSetup { location } => {
                self.current_location = location.to_owned();
            }
            _ => (),
        });
    }
}

fn handle_task(cx: &mut ContextProxy, task: Task, yt_dlp_path: PathBuf) {
    let export_location = task.location();
    match task.data() {
        TaskData::Video(data, settings) => {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut active_task = ActiveTask::new(task.clone());
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
                .ffmpeg_progress(move |progress| {
                    if let Err(err) = tx.send(progress) {
                        eprintln!("Failed to submit event to context proxy due to error: {err}");
                    }
                })
                .build();

            handle_video_task(cx, exporter, task.clone());
        }
        TaskData::Playlist(data) => {
            handle_playlist_task(cx);
        }
    }
}

fn handle_video_task(cx: &mut ContextProxy, exporter: VideoExporter<Unused>, task: Task) {
    let result = exporter
        .fetch_video()
        .inspect(|_| {
            let mut active_task = ActiveTask::new(task);
            active_task.set_downloading(false);
            cx.emit_print_err(TaskEvent::UpdateActive(active_task));
        })
        .and_then(|exporter| exporter.grab_filepath())
        .and_then(|exporter| exporter.final_export());

    if let Err(err) = result {
        cx.emit_print_err(Error::from(err));
    }
}

fn handle_playlist_task(cx: &mut ContextProxy) {
    todo!();
}
