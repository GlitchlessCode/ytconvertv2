use super::*;

use crate::{events::PlaylistExportSettingsEvent, helpers::make_filename_valid};

#[derive(Lens, Data, Clone, PartialEq, Debug)]
pub struct PlaylistExportSettings {
    pub title: String,

    pub export_type: ExportType,
}

impl PlaylistExportSettings {
    pub(crate) fn new(title: String) -> Self {
        Self {
            title: make_filename_valid(title),
            export_type: AudioExtension::Mp3.into(),
        }
    }

    pub(crate) fn event(&mut self, _cx: &mut EventContext, event: PlaylistExportSettingsEvent) {
        match event {
            PlaylistExportSettingsEvent::ChangeTitle(title) => {
                self.title = make_filename_valid(title);
            }
            PlaylistExportSettingsEvent::SetToVideo(set_to_video) => {
                if set_to_video {
                    if self.export_type.is_audio() {
                        self.export_type = VideoExtension::Mp4.into();
                    }
                } else {
                    if self.export_type.is_video() {
                        self.export_type = AudioExtension::Mp3.into();
                    }
                }
            }
            PlaylistExportSettingsEvent::SetExtension(extension) => {
                if (extension.is_audio() && self.export_type.is_audio())
                    || (extension.is_video() && self.export_type.is_video())
                {
                    self.export_type = extension;
                }
            }
        }
    }
}
