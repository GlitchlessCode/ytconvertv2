use super::*;
use crate::{
    data::task::VideoData,
    error::{Error, ErrorSeverity},
    include_bytes_safe,
    views::all::{Notification, NotificationLevel},
};
use reqwest::blocking::{Client, Response};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

pub static LARGE_PLACEHOLDER: &[u8] = include_bytes_safe!("..", "img", "large_placeholder.png");

#[derive(Lens)]
pub struct AppVideoData {
    #[lens(ignore)]
    client: Client,

    pub link: String,

    pub video: Option<VideoData>,

    pub thumbnail_generation: usize,
}

impl AppVideoData {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            link: String::new(),
            video: None,
            thumbnail_generation: 0,
        }
    }

    fn finalize_thumbnail_fetch(&mut self, cx: &mut EventContext, link: String, res: Response) {
        if self
            .video
            .as_ref()
            .is_some_and(|video| *video.thumbnail() == link)
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
                "video_thumb".to_string(),
                &image_data,
                ImageRetentionPolicy::Forever,
            ) {
                cx.emit(
                    Error::new()
                        .title("Image Loading Failed")
                        .code(0)
                        .severity(ErrorSeverity::Warning)
                        .description("Failed to load a new thumbnail image for a video"),
                );
            }

            self.thumbnail_generation += 1;
        }
    }
}

pub enum AppVideoEvent {
    LinkSubmit(String),
    Reset,

    TryVideoUrl(String),
    UrlFailed,
    UrlSucceeded(VideoData),

    FetchThumbnail(String),
    FinishedThumbnailFetch(String, Response),
}

impl Model for AppVideoData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _meta| match event {
            AppVideoEvent::LinkSubmit(text) => {
                self.link = text;

                if !self.link.trim().is_empty() {
                    cx.emit(AppVideoEvent::TryVideoUrl(self.link.trim().to_owned()))
                }
            }
            AppVideoEvent::Reset => self.link = String::new(),
            // Try to get video info
            AppVideoEvent::TryVideoUrl(url) => cx.spawn(|cx| fetch_video_data(cx, url)),
            AppVideoEvent::UrlFailed => {
                cx.emit(
                    Notification::new()
                        .message("Invalid URL, please try again")
                        .level(NotificationLevel::Error)
                        .build(),
                );

                if !self.video.same(&None) {
                    self.video = None;

                    if let Err(_) = cx.get_proxy().load_image(
                        "video_thumb".to_string(),
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
            AppVideoEvent::UrlSucceeded(data) => {
                let thumbnail = data.thumbnail().to_owned();
                let wrapped = Some(data);
                if !self.video.same(&wrapped) {
                    cx.emit(AppVideoEvent::FetchThumbnail(thumbnail));
                    self.video = wrapped;
                }
            }

            // Link should be https://i.ytimg.com/vi/<video_id>/mqdefault.jpg
            AppVideoEvent::FetchThumbnail(url) => {
                let client = self.client.clone();
                cx.spawn(move |cx| fetch_thumbnail(cx, client, url));
            }

            AppVideoEvent::FinishedThumbnailFetch(link, res) => {
                self.finalize_thumbnail_fetch(cx, link, res)
            }
        });
    }
}

fn fetch_video_data(cx: &mut ContextProxy, url: String) {
    let event = YoutubeDl::new(url)
        .flat_playlist(true)
        .extra_arg("--skip-download")
        .extra_arg("--no-playlist") // Restrict to videos only
        .run()
        .ok()
        .and_then(|output| match output {
            // If the response is a playlist (should not be possible)
            YoutubeDlOutput::Playlist(_) => None,
            // If the response is a video
            YoutubeDlOutput::SingleVideo(video) => Some(video),
        })
        .and_then(|video| VideoData::try_from(*video).ok());

    let event = match event {
        Some(data) => AppVideoEvent::UrlSucceeded(data),
        None => AppVideoEvent::UrlFailed,
    };

    if let Err(_) = cx.emit(event) {
        eprintln!("Could not submit error event, failed to send event");
    }
}

fn fetch_thumbnail(cx: &mut ContextProxy, client: Client, url: String) {
    let request = match client.get(&url).build() {
        Err(e) => {
            let error = Error::new()
                .title("Reqwest Request Builder Failed")
                .code(200)
                .description(format!(
                    "Failed to create a build a Request due to the following error from reqwest: {e}"
                ))
                .severity(ErrorSeverity::Warning)
                .build();

            if let Err(_) = cx.emit(error) {
                eprintln!("Could not submit error event, failed to send event");
            }
            return;
        }
        Ok(request) => request,
    };

    let res = match client.execute(request) {
        Err(e) => {
            let error = Error::new()
                .title("Reqwest Request Execution Failed")
                .code(201)
                .description(format!(
                    "Failed execute a built Request due to the following error from reqwest: {e}"
                ))
                .severity(ErrorSeverity::Warning)
                .build();

            if let Err(_) = cx.emit(error) {
                eprintln!("Could not submit error event, failed to send event");
            }
            return;
        }
        Ok(response) => response,
    };

    if let Err(_) = cx.emit(AppVideoEvent::FinishedThumbnailFetch(url, res)) {
        let error = Error::new()
            .title("Event Emission Failure")
            .code(100)
            .description("Failed to emit an error to the context proxy while submitting an AppVideoEvent::FinishedThumbnailFetch event")
            .severity(ErrorSeverity::Warning)
            .build();

        if let Err(_) = cx.emit(error) {
            eprintln!("Could not submit error event, failed to send event");
        }
    }
}
