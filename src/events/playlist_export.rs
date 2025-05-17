use crate::data::ExportType;

pub enum PlaylistExportSettingsEvent {
    ChangeTitle(String),
    SetToVideo(bool),
    SetExtension(ExportType),
}
