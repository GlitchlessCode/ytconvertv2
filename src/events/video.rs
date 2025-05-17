use super::VideoExportSettingsEvent;

use crate::data::VideoData;
use reqwest::blocking::Response;

pub enum AppVideoEvent {
    LinkSubmit(String),
    Reset,

    TryVideoUrl(String),
    UrlFailed,
    UrlSucceeded(VideoData, String), // Data, Original url

    FetchThumbnail(String),
    FinishedThumbnailFetch(String, Response),

    ExportSettingsEvent(VideoExportSettingsEvent),
}
