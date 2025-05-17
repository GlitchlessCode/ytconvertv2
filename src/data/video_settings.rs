use super::*;

use crate::{events::VideoExportSettingsEvent, helpers::make_filename_valid};

#[derive(Lens, Data, Clone, PartialEq, Debug)]
pub struct VideoExportSettings {
    pub title: String,

    pub export_type: ExportType,
}

impl VideoExportSettings {
    pub(crate) fn new(title: String) -> Self {
        Self {
            title: make_filename_valid(title),
            export_type: AudioExtension::Mp3.into(),
        }
    }

    pub(crate) fn event(&mut self, _cx: &mut EventContext, event: VideoExportSettingsEvent) {
        match event {
            VideoExportSettingsEvent::ChangeTitle(title) => {
                self.title = make_filename_valid(title);
            }
            VideoExportSettingsEvent::SetToVideo(set_to_video) => {
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
