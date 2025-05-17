use super::{PlaylistExportSettingsEvent, PlaylistVideoSettingsEvent};

use crate::data::PlaylistData;
use reqwest::blocking::Response;

pub enum AppPlaylistEvent {
    LinkSubmit(String),
    Reset,

    TryPlaylistUrl(String),
    UrlFailed,
    UrlSucceeded(PlaylistData, String), // Data, Original url

    FetchThumbnail(String),
    FinishedThumbnailFetch(String, Response),

    ExportSettingsEvent(PlaylistExportSettingsEvent),
    PlaylistVideoSettingsEvent(usize, PlaylistVideoSettingsEvent),
}
