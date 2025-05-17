use super::*;

use std::path::PathBuf;

use crate::{
    data::{PlaylistData, PlaylistExportSettings},
    events::AppPlaylistEvent,
    helpers::{fetch_thumbnail, ContextProxyExt},
};
use reqwest::blocking::{Client, Response};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[derive(Lens)]
pub struct AppPlaylistData {
    #[lens(ignore)]
    client: Client,

    pub searching: bool,
    pub link: String,

    pub playlist: Option<PlaylistData>,

    pub thumbnail_generation: usize,

    pub export_settings: PlaylistExportSettings,

    #[lens(ignore)]
    yt_dlp: PathBuf,
}

impl AppPlaylistData {
    pub fn new(yt_dlp: PathBuf, client: Client) -> Self {
        Self {
            client,
            searching: false,
            link: String::new(),
            playlist: None,
            thumbnail_generation: 0,
            export_settings: PlaylistExportSettings::new(String::new()),
            yt_dlp,
        }
    }

    fn finalize_thumbnail_fetch(&mut self, cx: &mut EventContext, link: String, res: Response) {
        if self
            .playlist
            .as_ref()
            .is_some_and(|playlist| *playlist.thumbnail() == link)
        {
            let image_data = match res.bytes() {
                Err(e) => {
                    let error = Error::new()
                    .title("Reqwest Response Bytes Conversion Failed")
                    .code(202)
                    .description(format!(
                        "Failed convert a response to bytes due to the following error from reqwest: {e}"
                    ))
                    .severity(ErrorSeverity::Warning)
                    .build();

                    cx.emit(error);

                    return;
                }
                Ok(bytes) => bytes,
            };

            if let Err(_) = cx.get_proxy().load_image(
                "playlist_thumb".to_string(),
                &image_data,
                ImageRetentionPolicy::Forever,
            ) {
                cx.emit(
                    Error::new()
                        .title("Image Loading Failed")
                        .code(0)
                        .severity(ErrorSeverity::Warning)
                        .description("Failed to load a new thumbnail image for a playlist"),
                );
            }

            self.thumbnail_generation += 1;
        }
    }

    fn reset(&mut self, cx: &mut EventContext) {
        self.link = String::new();
        self.playlist = None;
        self.export_settings = PlaylistExportSettings::new(String::new());

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
                        cx.emit(AppPlaylistEvent::TryPlaylistUrl(
                            self.link.trim().to_owned(),
                        ))
                    } else {
                        self.reset(cx);
                    }
                }
            }
            AppPlaylistEvent::Reset => self.reset(cx),

            AppPlaylistEvent::TryPlaylistUrl(url) => {
                self.searching = true;
                let yt_dlp = self.yt_dlp.clone();
                cx.spawn(|cx| fetch_playlist_data(cx, url, yt_dlp))
            }
            AppPlaylistEvent::UrlFailed => {
                self.searching = false;
                cx.emit(
                    Notification::new()
                        .message("Invalid URL, please try again")
                        .level(NotificationLevel::Error)
                        .build(),
                );

                if !self.playlist.same(&None) {
                    self.reset(cx);
                }
            }
            AppPlaylistEvent::UrlSucceeded(data, url) => {
                self.searching = false;
                if self.link == url {
                    let thumbnail = data.thumbnail().to_owned();
                    let wrapped = Some(data.clone());
                    if !self.playlist.same(&wrapped) {
                        cx.emit(AppPlaylistEvent::FetchThumbnail(thumbnail));
                        self.playlist = wrapped;
                        self.export_settings = PlaylistExportSettings::new(data.title().clone());
                    }
                }
            }

            AppPlaylistEvent::FetchThumbnail(url) => {
                let client = self.client.clone();
                cx.spawn(move |cx| {
                    fetch_thumbnail(cx, client, url, |cx, url, res| {
                        cx.emit(AppPlaylistEvent::FinishedThumbnailFetch(url, res))
                    })
                });
            }
            AppPlaylistEvent::FinishedThumbnailFetch(link, res) => {
                self.finalize_thumbnail_fetch(cx, link, res)
            }

            AppPlaylistEvent::ExportSettingsEvent(event) => self.export_settings.event(cx, event),
            AppPlaylistEvent::PlaylistVideoSettingsEvent(index, event) => {
                if let Some(ref mut data) = self.playlist {
                    data.video_event(index, event);
                }
            }
        });
    }
}

fn fetch_playlist_data(cx: &mut ContextProxy, url: String, yt_dlp: PathBuf) {
    let event = YoutubeDl::new(&url)
        .youtube_dl_path(yt_dlp)
        .flat_playlist(true)
        .extra_arg("--skip-download")
        .extra_arg("--yes-playlist")
        .run()
        .ok()
        .and_then(|output| match output {
            // If the response is a playlist
            YoutubeDlOutput::Playlist(playlist) => Some(playlist),
            // If the response is a video
            YoutubeDlOutput::SingleVideo(_) => None,
        })
        .and_then(|playlist| PlaylistData::try_from(*playlist).ok());

    let event = match event {
        Some(data) => AppPlaylistEvent::UrlSucceeded(data, url),
        None => AppPlaylistEvent::UrlFailed,
    };

    cx.emit_print_err(event)
}
