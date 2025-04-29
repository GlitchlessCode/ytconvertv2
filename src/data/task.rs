use std::path::PathBuf;
use vizia::prelude::*;
use youtube_dl::SingleVideo;

use crate::models::video::VideoExportSettings;

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
    Video(VideoData, VideoExportSettings),
    Playlist(PlaylistData),
}

impl TaskData {
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
    title: String,
    url: String,
    author: String,
    duration: u64,
    thumbnail_id: String,
}

impl VideoData {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }

    pub fn thumbnail(&self) -> &String {
        &self.thumbnail_id
    }
}

impl TryFrom<SingleVideo> for VideoData {
    type Error = ();
    fn try_from(value: SingleVideo) -> Result<Self, Self::Error> {
        let title = value.title.ok_or(())?;
        let url = format!("https://www.youtube.com/watch?v={}", value.id);
        let author = value.channel.ok_or(())?;
        let duration = value.duration.ok_or(())?.as_u64().ok_or(())?;
        let thumbnail_id = format!("https://i.ytimg.com/vi/{}/mqdefault.jpg", value.id);

        Ok(Self {
            title,
            url,
            author,
            duration,
            thumbnail_id,
        })
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
