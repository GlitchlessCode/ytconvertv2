use super::*;

use crate::{events::PlaylistVideoSettingsEvent, helpers::make_filename_valid};

#[derive(Lens, Data, Clone, PartialEq, Debug)]
pub struct PlaylistVideoSettings {
    pub title: String,
    pub editing: bool,

    pub include: bool,
}

impl PlaylistVideoSettings {
    pub(crate) fn new(title: String) -> Self {
        Self {
            title: make_filename_valid(title),
            editing: false,
            include: true,
        }
    }

    pub(crate) fn event(&mut self, event: PlaylistVideoSettingsEvent) {
        match event {
            PlaylistVideoSettingsEvent::SetInclusion(inclusion) => {
                self.include = inclusion;
            }
            PlaylistVideoSettingsEvent::StartEdit => {
                self.editing = true;
            }
            PlaylistVideoSettingsEvent::EndEdit(text) => {
                if self.editing {
                    self.editing = false;
                    self.title = make_filename_valid(text);
                }
            }
        }
    }

    pub(crate) fn to_video_setting(&self, export_type: ExportType) -> VideoExportSettings {
        VideoExportSettings {
            title: self.title.clone(),
            export_type,
        }
    }
}
