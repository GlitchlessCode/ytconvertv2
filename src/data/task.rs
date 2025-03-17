use std::path::PathBuf;
use vizia::prelude::*;

#[derive(Debug, Data, Clone)]
pub struct Task {
    export_location: PathBuf,

    data: TaskData,
}

impl Task {
    /// Create a new Video task
    ///
    /// `location` and `url` must both be valid
    pub fn video(
        location: PathBuf,
        url: String,
        author: String,
        duration: u64,
        thumbnail_id: Option<String>,
    ) -> Self {
        Self {
            export_location: location,
            data: TaskData::video(url, author, duration, thumbnail_id),
        }
    }

    /// Create a new Playlist task
    ///
    /// `location` must be valid
    pub fn playlist(location: PathBuf) -> Self {
        Self {
            export_location: location,
            data: TaskData::playlist(vec![]),
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
    Video(VideoData),
    Playlist(PlaylistData),
}

impl TaskData {
    fn video(url: String, author: String, duration: u64, thumbnail_id: Option<String>) -> Self {
        Self::Video(VideoData {
            url,
            author,
            duration,
            thumbnail_id,
        })
    }

    fn playlist(videos: Vec<VideoData>) -> Self {
        Self::Playlist(PlaylistData { videos })
    }

    /// Get the task type as a string
    pub fn get_type(&self) -> &str {
        match self {
            Self::Video { .. } => "VIDEO",
            Self::Playlist { .. } => "PLAYLIST",
        }
    }
}

#[derive(Debug, Data, Clone)]
pub struct VideoData {
    url: String,
    author: String,
    duration: u64,
    thumbnail_id: Option<String>,
}

impl VideoData {
    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }

    pub fn thumbnail(&self) -> Option<&String> {
        self.thumbnail_id.as_ref()
    }
}

#[derive(Debug, Data, Clone)]
pub struct PlaylistData {
    videos: Vec<VideoData>,
}

impl PlaylistData {
    pub fn iter(&self) -> std::slice::Iter<'_, VideoData> {
        self.videos.iter()
    }
}
