use super::video_export::VideoExportSettingsEvent;

use crate::data::task::VideoData;
use reqwest::blocking::Response;

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
