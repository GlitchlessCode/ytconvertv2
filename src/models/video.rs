use std::path::PathBuf;

use super::*;
use crate::{
    data::task::VideoData,
    error::{Error, ErrorSeverity},
    helpers::make_filename_valid,
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

    pub searching: bool,
    pub link: String,

    pub video: Option<VideoData>,

    pub thumbnail_generation: usize,

    pub export_settings: VideoExportSettings,

    #[lens(ignore)]
    yt_dlp: PathBuf,
}

#[derive(Lens, Data, Clone, PartialEq, Debug)]
pub struct VideoExportSettings {
    pub title: String,

    pub export_type: ExportType,
}

impl VideoExportSettings {
    fn new(title: String) -> Self {
        Self {
            title: make_filename_valid(title),
            export_type: AudioExtension::Mp3.into(),
        }
    }

    fn event(&mut self, _cx: &mut EventContext, event: VideoExportSettingsEvent) {
        match event {
            VideoExportSettingsEvent::ChangeTitle(title) => {
                self.title = make_filename_valid(title);
            }
            VideoExportSettingsEvent::SetToVideo(set_to_video) => {
                if set_to_video {
                    if self.export_type.is_audio() {
                        self.export_type = ExportType::Video(VideoExtension::Mp4);
                    }
                } else {
                    if self.export_type.is_video() {
                        self.export_type = ExportType::Audio(AudioExtension::Mp3);
                    }
                }
            }
            VideoExportSettingsEvent::SetExtension(extension) => {
                if (extension.is_audio() && self.export_type.is_audio())
                    || (extension.is_video() && self.export_type.is_video())
                {
                    self.export_type = extension;
                }
            }
        }
    }
}

pub enum VideoExportSettingsEvent {
    ChangeTitle(String),
    SetToVideo(bool),
    SetExtension(ExportType),
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum ExportType {
    Audio(AudioExtension),
    Video(VideoExtension),
}

impl ExportType {
    pub fn is_audio(&self) -> bool {
        match self {
            Self::Audio(_) => true,
            Self::Video(_) => false,
        }
    }

    pub fn is_video(&self) -> bool {
        match self {
            Self::Audio(_) => false,
            Self::Video(_) => true,
        }
    }

    pub fn formats(&self) -> Vec<ExportType> {
        match self {
            Self::Audio(_) => vec![
                AudioExtension::Mp3.into(),
                AudioExtension::Wav.into(),
                AudioExtension::Ogg.into(),
            ],
            Self::Video(_) => vec![
                VideoExtension::Mp4.into(),
                VideoExtension::Mov.into(),
                VideoExtension::Mkv.into(),
            ],
        }
    }
}

impl std::fmt::Display for ExportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Audio(a) => a.to_string(),
                Self::Video(v) => v.to_string(),
            }
        )
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum AudioExtension {
    Mp3,
    Wav,
    Ogg,
}

impl std::fmt::Display for AudioExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".{}",
            match self {
                Self::Mp3 => "mp3",
                Self::Wav => "wav",
                Self::Ogg => "ogg",
            }
        )
    }
}

impl From<AudioExtension> for ExportType {
    fn from(value: AudioExtension) -> Self {
        ExportType::Audio(value)
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum VideoExtension {
    Mp4,
    Mov,
    Mkv,
}

impl std::fmt::Display for VideoExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".{}",
            match self {
                Self::Mp4 => "mp4",
                Self::Mov => "mov",
                Self::Mkv => "mkv",
            }
        )
    }
}

impl From<VideoExtension> for ExportType {
    fn from(value: VideoExtension) -> Self {
        ExportType::Video(value)
    }
}

impl AppVideoData {
    pub fn new(yt_dlp: PathBuf) -> Self {
        Self {
            client: Client::new(),
            searching: false,
            link: String::new(),
            video: None,
            thumbnail_generation: 0,
            export_settings: VideoExportSettings::new(String::new()),
            yt_dlp,
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

    fn reset(&mut self, cx: &mut EventContext) {
        self.link = String::new();
        self.video = None;
        self.export_settings = VideoExportSettings::new(String::new());

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

pub enum AppVideoEvent {
    LinkSubmit(String),
    Reset,

    TryVideoUrl(String),
    UrlFailed,
    UrlSucceeded(VideoData, String),

    FetchThumbnail(String),
    FinishedThumbnailFetch(String, Response),

    ExportSettingsEvent(VideoExportSettingsEvent),
}

impl Model for AppVideoData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _meta| match event {
            AppVideoEvent::LinkSubmit(text) => {
                if self.link != text {
                    self.link = text;

                    if !self.link.trim().is_empty() {
                        cx.emit(AppVideoEvent::TryVideoUrl(self.link.trim().to_owned()))
                    } else {
                        self.reset(cx);
                    }
                }
            }
            AppVideoEvent::Reset => self.reset(cx),
            // Try to get video info
            AppVideoEvent::TryVideoUrl(url) => {
                self.searching = true;
                let yt_dlp = self.yt_dlp.clone();
                cx.spawn(|cx| fetch_video_data(cx, url, yt_dlp))
            }
            AppVideoEvent::UrlFailed => {
                self.searching = false;
                cx.emit(
                    Notification::new()
                        .message("Invalid URL, please try again")
                        .level(NotificationLevel::Error)
                        .build(),
                );

                if !self.video.same(&None) {
                    self.reset(cx);
                }
            }
            AppVideoEvent::UrlSucceeded(data, url) => {
                self.searching = false;
                if self.link == url {
                    let thumbnail = data.thumbnail().to_owned();
                    let wrapped = Some(data.clone());
                    if !self.video.same(&wrapped) {
                        cx.emit(AppVideoEvent::FetchThumbnail(thumbnail));
                        self.video = wrapped;
                        self.export_settings = VideoExportSettings::new(data.title().clone());
                    }
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

            AppVideoEvent::ExportSettingsEvent(event) => self.export_settings.event(cx, event),
        });
    }
}

fn fetch_video_data(cx: &mut ContextProxy, url: String, yt_dlp: PathBuf) {
    let event = YoutubeDl::new(&url)
        .youtube_dl_path(yt_dlp)
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
        Some(data) => AppVideoEvent::UrlSucceeded(data, url),
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
            .code(3)
            .description("Failed to emit an error to the context proxy while submitting an AppVideoEvent::FinishedThumbnailFetch event")
            .severity(ErrorSeverity::Warning)
            .build();

        if let Err(_) = cx.emit(error) {
            eprintln!("Could not submit error event, failed to send event");
        }
    }
}
