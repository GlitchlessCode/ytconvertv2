use youtube_dl::SingleVideo;

use super::*;

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
