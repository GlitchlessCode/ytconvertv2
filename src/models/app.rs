use super::*;
use crate::{
    config::ConfigEvent,
    data::{Task, TaskQueue},
    views::task_queue::TaskEvent,
};
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Lens)]
pub struct AppData {
    pub theme: Theme,
    pub task_queue: TaskQueue,

    pub playlist_selected: bool,

    pub current_location: Option<PathBuf>,
    pub yt_dlp_path: PathBuf,
    pub ffmpeg_install_state: FfmpegInstallState,

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
                self.task_queue.push(task.clone());
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

        event.map(|event, _meta| match event {
            TaskEvent::Remove(index) => self.task_queue.remove(*index),
        });

        event.map(|event, _meta| match event {
            ConfigEvent::ConfigSetup { location } => {
                self.current_location = location.to_owned();
            }
            _ => (),
        });
    }
}

#[non_exhaustive]
pub enum AppEvent {
    // Windows only
    #[cfg(windows)]
    Maximized(bool),

    // Export location
    RequestLocationChange,
    SetNewLocation(PathBuf),

    // Ffmpeg install
    SetFfmpegInstallState(FfmpegInstallState),

    // Submit Task
    SubmitTask(Task),
    RemoveTask(usize),

    // Toggle button
    ToggleVideo,
    TogglePlaylist,
}

#[derive(Clone)]
pub enum FfmpegInstallState {
    Installed,
    Installing,
    Updating,

    Failed,
    Missing,
    Unknown,
}
