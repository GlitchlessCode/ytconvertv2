use super::*;

use std::path::PathBuf;

#[derive(Debug, Data, Clone)]
pub struct Task {
    export_location: PathBuf,

    data: TaskData,
}

impl Task {
    /// Create a new Video task
    ///
    /// `location` must be valid
    pub fn video(location: PathBuf, data: VideoData, settings: VideoExportSettings) -> Self {
        Self {
            export_location: location,
            data: TaskData::Video(data, settings),
        }
    }

    /// Create a new Playlist task
    ///
    /// `location` must be valid
    pub fn playlist(
        location: PathBuf,
        data: PlaylistData,
        settings: PlaylistExportSettings,
    ) -> Self {
        Self {
            export_location: location,
            data: TaskData::Playlist(data, settings),
        }
    }

    /// Get the export location
    pub fn location(&self) -> &PathBuf {
        &self.export_location
    }

    /// Get the task data
    pub fn data(&self) -> &TaskData {
        &self.data
    }

    pub fn is_video(&self) -> bool {
        if let TaskData::Video { .. } = self.data {
            return true;
        }

        return false;
    }

    pub fn is_playlist(&self) -> bool {
        if let TaskData::Playlist { .. } = self.data {
            return true;
        }

        return false;
    }
}

#[derive(Debug, Data, Clone)]
pub enum TaskData {
    Video(VideoData, VideoExportSettings),
    Playlist(PlaylistData, PlaylistExportSettings),
}

impl TaskData {
    /// Get the task type as a string
    pub fn get_type(&self) -> &str {
        match self {
            Self::Video { .. } => "VIDEO",
            Self::Playlist { .. } => "PLAYLIST",
        }
    }
}
