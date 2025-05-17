use super::video::{VideoExporter, VideoProgress};

use crate::{
    data::{PlaylistData, PlaylistExportSettings, PlaylistVideoSettings},
    error::{Error, ErrorSeverity},
};
use bon::bon;
use std::{fs::create_dir_all, path::PathBuf, sync::Arc};

#[derive(thiserror::Error, Debug)]
pub enum PlaylistExporterError {
    // Transparent
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    VideoError(#[from] super::video::VideoExporterError),
}

impl From<PlaylistExporterError> for Error {
    fn from(value: PlaylistExporterError) -> Self {
        match value {
            PlaylistExporterError::IOError(error) => Self::new()
                .title("Playlist Error: Standard IO Error")
                .code(13)
                .severity(ErrorSeverity::Error)
                .description(format!("{error}"))
                .build(),
            PlaylistExporterError::VideoError(error) => Error::from(error),
        }
    }
}

pub enum PlaylistProgress {
    StartVideo(PlaylistVideoSettings),
    VideoProgress(f32),
    VideoComplete(usize),
    Done,
}

pub struct PlaylistExporter<S> {
    state: S,
}

#[bon]
impl PlaylistExporter<Unused> {
    #[builder]
    pub fn new<F: Fn(PlaylistProgress) + Send + Sync + 'static>(
        export_location: PathBuf,
        data: PlaylistData,
        settings: PlaylistExportSettings,
        #[builder(into)] yt_dlp_path: PathBuf,
        ffmpeg_progress: F,
    ) -> PlaylistExporter<Unused> {
        PlaylistExporter {
            state: Unused {
                export_location,
                data,
                settings,
                yt_dlp_path,
                ffmpeg_progress: Arc::new(ffmpeg_progress),
            },
        }
    }
}

pub struct Unused {
    export_location: PathBuf,
    data: PlaylistData,
    settings: PlaylistExportSettings,
    yt_dlp_path: PathBuf,
    ffmpeg_progress: Arc<dyn Fn(PlaylistProgress) + Send + Sync + 'static>,
}

impl PlaylistExporter<Unused> {
    pub fn process_videos(self) -> Result<(), PlaylistExporterError> {
        let Unused {
            export_location,
            data,
            settings,
            yt_dlp_path,
            ffmpeg_progress,
        } = self.state;

        let export_location = export_location.join(settings.title);
        let export_type = settings.export_type;

        for (idx, (data, settings)) in data
            .iter()
            .filter(|(_, settings)| settings.include)
            .enumerate()
        {
            create_dir_all(export_location.clone())?;

            ffmpeg_progress(PlaylistProgress::StartVideo(settings.clone()));

            let ffmpeg_progress_clone = Arc::clone(&ffmpeg_progress);
            let exporter = VideoExporter::builder()
                .data(data.clone())
                .export_location(export_location.clone())
                .ffmpeg_progress(move |progress| {
                    if let VideoProgress::Progress(progress) = progress {
                        ffmpeg_progress_clone(PlaylistProgress::VideoProgress(progress));
                    }
                })
                .settings(settings.to_video_setting(export_type.clone()))
                .yt_dlp_path(yt_dlp_path.clone())
                .build();

            exporter.fetch_video()?.grab_filepath()?.final_export()?;

            ffmpeg_progress(PlaylistProgress::VideoComplete(idx))
        }

        ffmpeg_progress(PlaylistProgress::Done);

        Ok(())
    }
}
