use reqwest::blocking::Response;

pub enum AppPlaylistEvent {
    LinkSubmit(String),
    Reset,

    // TryVideoUrl(String),
    // UrlFailed,
    // UrlSucceeded(VideoData, String),
    FetchThumbnail(String),
    FinishedThumbnailFetch(String, Response),
    // ExportSettingsEvent(VideoExportSettingsEvent),
}
