use super::*;

#[derive(Debug, Data, Clone)]
pub enum ActiveTask {
    Video {
        data: VideoData,
        settings: VideoExportSettings,
        downloading: bool,
        progress: f32,
    },
    Playlist {
        data: PlaylistData,
        settings: PlaylistExportSettings,
        active_video: Option<PlaylistVideoSettings>,
        downloading: bool,
        video_progress: f32,
        finished_count: usize,
        total_videos: usize,
    },
}

impl ActiveTask {
    pub fn new(base: &Task) -> Self {
        match base.data() {
            TaskData::Video(data, settings) => Self::Video {
                data: data.clone(),
                settings: settings.clone(),
                downloading: true,
                progress: 0.0,
            },
            TaskData::Playlist(data, settings) => Self::Playlist {
                data: data.clone(),
                settings: settings.clone(),
                active_video: None,
                downloading: true,
                video_progress: 0.0,
                finished_count: 0,
                total_videos: data.active_video_count(),
            },
        }
    }

    pub fn set_video_progress(&mut self, new_progress: f32) {
        match self {
            ActiveTask::Video {
                ref mut progress, ..
            } => {
                *progress = new_progress;
            }
            ActiveTask::Playlist {
                ref mut video_progress,
                ..
            } => {
                *video_progress = new_progress;
            }
        }
    }

    pub fn get_video_progress(&self) -> f32 {
        *match self {
            ActiveTask::Video { progress, .. } => progress,
            ActiveTask::Playlist { video_progress, .. } => video_progress,
        }
    }

    pub fn set_downloading(&mut self, new_downloading: bool) {
        match self {
            ActiveTask::Video {
                ref mut downloading,
                ..
            } => {
                *downloading = new_downloading;
            }
            ActiveTask::Playlist {
                ref mut downloading,
                ..
            } => {
                *downloading = new_downloading;
            }
        }
    }

    pub fn get_downloading(&self) -> bool {
        *match self {
            ActiveTask::Video { downloading, .. } => downloading,
            ActiveTask::Playlist { downloading, .. } => downloading,
        }
    }

    pub fn set_finished_count(&mut self, new_finished_count: usize) {
        match self {
            ActiveTask::Playlist {
                ref mut finished_count,
                ..
            } => {
                *finished_count = new_finished_count;
            }
            _ => (),
        }
    }

    pub fn get_count_ratio(&self) -> Option<(usize, usize)> {
        match self {
            ActiveTask::Playlist {
                finished_count,
                total_videos,
                ..
            } => Some((*finished_count, *total_videos)),
            _ => None,
        }
    }

    pub fn set_active_video(&mut self, new_active_video: PlaylistVideoSettings) {
        match self {
            ActiveTask::Playlist {
                ref mut active_video,
                ..
            } => {
                *active_video = Some(new_active_video);
            }
            _ => (),
        }
    }

    pub fn get_active_video(&self) -> Option<PlaylistVideoSettings> {
        match self {
            ActiveTask::Playlist { active_video, .. } => active_video.clone(),
            _ => None,
        }
    }
}
