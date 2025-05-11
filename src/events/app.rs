use std::path::PathBuf;

use crate::data::{FfmpegInstallState, Task};

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
