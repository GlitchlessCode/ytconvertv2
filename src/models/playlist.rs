use std::path::PathBuf;

use crate::events::AppPlaylistEvent;

use super::*;
use reqwest::blocking::{Client, Response};

#[derive(Lens)]
pub struct AppPlaylistData {
    #[lens(ignore)]
    client: Client,

    pub searching: bool,
    pub link: String,

    pub thumbnail_generation: usize,

    #[lens(ignore)]
    yt_dlp: PathBuf,
}

impl AppPlaylistData {
    pub fn new(yt_dlp: PathBuf, client: Client) -> Self {
        Self {
            client,
            searching: false,
            link: String::new(),
            thumbnail_generation: 0,
            yt_dlp,
        }
    }

    fn reset(&mut self, cx: &mut EventContext) {
        self.link = String::new();
        // self.video = None;
        // self.export_settings = VideoExportSettings::new(String::new());

        if let Err(_) = cx.get_proxy().load_image(
            "playlist_thumb".to_string(),
            LARGE_PLACEHOLDER,
            ImageRetentionPolicy::Forever,
        ) {
            cx.emit(
                Error::new()
                    .title("Image Loading Failed")
                    .code(0)
                    .severity(ErrorSeverity::Warning)
                    .description("Failed to load default thumbnail image")
                    .build(),
            );
        }

        self.thumbnail_generation += 1;
    }
}

impl Model for AppPlaylistData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _meta| match event {
            AppPlaylistEvent::LinkSubmit(text) => {
                if self.link != text {
                    self.link = text;

                    if !self.link.trim().is_empty() {
                        // cx.emit(AppPlaylistEvent::TryPlaylistUrl(self.link.trim().to_owned()))
                    } else {
                        self.reset(cx);
                    }
                }
            }
            AppPlaylistEvent::Reset => self.reset(cx),

            // AppPlaylistEvent::TryPlaylistUrl(String)
            // AppPlaylistEvent::UrlFailed
            // AppPlaylistEvent::UrlSucceeded(PlaylistData, String)
            AppPlaylistEvent::FetchThumbnail(_) => (),
            AppPlaylistEvent::FinishedThumbnailFetch(_, _) => (),
            // AppPlaylistEvent::ExportSettingsEvent(PlaylistExportSettingsEvent)
        });
    }
}
