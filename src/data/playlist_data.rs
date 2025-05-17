use youtube_dl::Playlist;

use crate::events::PlaylistVideoSettingsEvent;

use super::*;

#[derive(Debug, Data, Clone)]
pub struct PlaylistData {
    title: String,
    url: String,
    creator: String,
    thumbnail_id: String,

    videos: Vec<(VideoData, PlaylistVideoSettings)>,
}

impl PlaylistData {
    pub fn iter(&self) -> std::slice::Iter<'_, (VideoData, PlaylistVideoSettings)> {
        self.videos.iter()
    }

    pub fn video_count(&self) -> usize {
        self.videos.len()
    }

    pub fn active_video_count(&self) -> usize {
        self.videos
            .iter()
            .filter(|(_, settings)| settings.include)
            .count()
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn creator(&self) -> &String {
        &self.creator
    }

    pub fn thumbnail(&self) -> &String {
        &self.thumbnail_id
    }

    pub fn video(&self, index: usize) -> Option<&(VideoData, PlaylistVideoSettings)> {
        (&self.videos as &[_]).get(index)
    }

    pub fn video_event(&mut self, index: usize, event: PlaylistVideoSettingsEvent) {
        if let Some((_, settings)) = self.videos.get_mut(index) {
            settings.event(event);
        }
    }
}

impl TryFrom<Playlist> for PlaylistData {
    type Error = ();
    fn try_from(value: Playlist) -> Result<Self, Self::Error> {
        let title = value.title.ok_or(())?;
        let url = format!(
            "https://www.youtube.com/playlist?list={}",
            value.id.ok_or(())?
        );
        let creator = value.uploader.ok_or(())?;
        let thumbnail_id = value
            .thumbnails
            .and_then(|thmbs| thmbs.get(thmbs.len() - 1).map(|thmb| thmb.to_owned()))
            .and_then(|thmb| thmb.url)
            .ok_or(())?;

        let videos = value
            .entries
            .ok_or(())?
            .into_iter()
            .map(|vid| {
                let title = vid.title.clone();
                (
                    VideoData::try_from(vid).ok(),
                    title.map(|title| PlaylistVideoSettings::new(title)),
                )
            })
            .filter_map(|data| match data {
                (Some(vid), Some(settings)) => Some((vid, settings)),
                _ => None,
            })
            .collect();

        Ok(Self {
            title,
            url,
            creator,
            thumbnail_id,
            videos,
        })
    }
}
