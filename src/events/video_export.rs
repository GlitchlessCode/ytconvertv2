use crate::data::ExportType;

pub enum VideoExportSettingsEvent {
    ChangeTitle(String),
    SetToVideo(bool),
    SetExtension(ExportType),
}
