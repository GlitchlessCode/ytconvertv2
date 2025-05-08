use crate::{
    data::task::VideoData, error::ErrorSeverity, helpers::parse_formatted_seconds,
    models::video::VideoExportSettings,
};
use bon::bon;
use nanoid::nanoid;
use num_traits::ToPrimitive;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};
use temp_dir::TempDir;
use youtube_dl::YoutubeDl;

pub type Result<S> = std::result::Result<VideoExporter<S>, VideoExporterError>;

#[derive(thiserror::Error, Debug)]
pub enum VideoExporterError {
    // Transparent
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    YoutubeDlError(#[from] youtube_dl::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    // Custom
    #[error("temporary directory is either missing or is not a directory")]
    TempDirMissing,

    #[error("file exported from yt-dlp is missing")]
    ExportMissing,

    #[error("there are multiple files that could be the target file from yt-dlp")]
    AmbiguousTempFile,

    #[error("converting a `PathBuf` to str unexepectedly returned `None`")]
    InvalidPathBufStr,
}

pub enum VideoProgress {
    Progress(f32),
    Done,
}

impl From<VideoExporterError> for crate::error::Error {
    fn from(value: VideoExporterError) -> Self {
        let title = format!(
            "Video Export: {}",
            match value {
                VideoExporterError::YoutubeDlError(_) => "yt-dlp Error",
                VideoExporterError::IOError(_) => "Standard IO Error",
                VideoExporterError::AnyhowError(_) => "Generic Error",
                VideoExporterError::TempDirMissing => "Missing Temporary Directory",
                VideoExporterError::ExportMissing => "Missing Exported File",
                VideoExporterError::AmbiguousTempFile => "Ambiguous File Found",
                VideoExporterError::InvalidPathBufStr => "Invalid PathBuf Found",
            }
        );

        let description = format!("{value}");

        let code = match value {
            VideoExporterError::YoutubeDlError(_) => 301,
            VideoExporterError::IOError(_) => 8,
            VideoExporterError::AnyhowError(_) => 400,
            VideoExporterError::TempDirMissing => 9,
            VideoExporterError::ExportMissing => 10,
            VideoExporterError::AmbiguousTempFile => 11,
            VideoExporterError::InvalidPathBufStr => 12,
        };

        Self::new()
            .title(title)
            .code(code)
            .severity(ErrorSeverity::Error)
            .description(description)
            .build()
    }
}

pub struct VideoExporter<S> {
    state: S,
}

#[bon]
impl VideoExporter<Unused> {
    #[builder]
    pub fn new<F: Fn(VideoProgress) + Send + Sync + 'static>(
        export_location: PathBuf,
        data: VideoData,
        settings: VideoExportSettings,
        #[builder(into)] yt_dlp_path: PathBuf,
        ffmpeg_progress: F,
    ) -> VideoExporter<Unused> {
        VideoExporter {
            state: Unused {
                export_location,
                data,
                settings,
                yt_dlp_path,
                ffmpeg_progress: Box::new(ffmpeg_progress),
            },
        }
    }
}

pub struct Unused {
    export_location: PathBuf,
    data: VideoData,
    settings: VideoExportSettings,
    yt_dlp_path: PathBuf,
    ffmpeg_progress: Box<dyn Fn(VideoProgress) + Send + Sync + 'static>,
}

impl VideoExporter<Unused> {
    pub fn fetch_video(self) -> Result<VideoFetched> {
        let Unused {
            export_location,
            data,
            settings,
            yt_dlp_path,
            ffmpeg_progress,
        } = self.state;

        let mut ytdl = YoutubeDl::new(data.url());

        let name = nanoid!();

        ytdl.youtube_dl_path(yt_dlp_path)
            .flat_playlist(true)
            .output_template(format!("{name}"))
            .extra_arg("--no-playlist")
            .extra_arg(format!(
                "--ffmpeg-location {}",
                ffmpeg_sidecar::paths::ffmpeg_path().to_string_lossy()
            ));

        if settings.export_type.is_audio() {
            ytdl.format("ba");
        }

        let temporary_dir = TempDir::new()?;

        ytdl.download_to(temporary_dir.path())?;

        Ok(VideoExporter {
            state: VideoFetched {
                export_location,
                data,
                settings,
                ffmpeg_progress,
                temporary_dir,
                id_name: name,
            },
        })
    }
}

pub struct VideoFetched {
    export_location: PathBuf,
    data: VideoData,
    settings: VideoExportSettings,
    ffmpeg_progress: Box<dyn Fn(VideoProgress) + Send + Sync + 'static>,
    temporary_dir: TempDir,
    id_name: String,
}

impl VideoExporter<VideoFetched> {
    pub fn grab_filepath(self) -> Result<PathGrabbed> {
        let VideoFetched {
            export_location,
            data,
            settings,
            ffmpeg_progress,
            temporary_dir,
            id_name,
        } = self.state;

        let temp_dir_path = temporary_dir.path();

        let input_path = if temp_dir_path.is_dir() {
            match_file_stem(temp_dir_path, id_name)?
        } else {
            return Err(VideoExporterError::TempDirMissing);
        };

        Ok(VideoExporter {
            state: PathGrabbed {
                export_location,
                data,
                settings,
                ffmpeg_progress,
                temporary_dir,
                input_path,
            },
        })
    }
}

fn match_file_stem(
    dir_path: &Path,
    target: String,
) -> std::result::Result<PathBuf, VideoExporterError> {
    let mut found: Option<PathBuf> = None;

    for file in read_dir(dir_path)? {
        if let Ok(file) = file {
            if let Some(stem) = file.path().file_stem() {
                if &stem.to_string_lossy() != target.as_str() {
                    continue;
                }

                if found.is_some() {
                    return Err(VideoExporterError::AmbiguousTempFile);
                }

                found = Some(file.path())
            }
        }
    }

    match found {
        None => Err(VideoExporterError::ExportMissing),
        Some(found) => Ok(found),
    }
}

pub struct PathGrabbed {
    export_location: PathBuf,
    data: VideoData,
    settings: VideoExportSettings,
    ffmpeg_progress: Box<dyn Fn(VideoProgress) + Send + Sync + 'static>,
    temporary_dir: TempDir,
    input_path: PathBuf,
}

impl VideoExporter<PathGrabbed> {
    pub fn final_export(self) -> std::result::Result<(), VideoExporterError> {
        let PathGrabbed {
            export_location,
            data,
            settings,
            ffmpeg_progress,
            temporary_dir,
            input_path,
        } = self.state;

        let input_path = match input_path.to_str() {
            Some(path) => path,
            None => return Err(VideoExporterError::InvalidPathBufStr),
        };

        let output_path =
            export_location.join(format!("{}{}", settings.title, settings.export_type));

        let output_path = match output_path.to_str() {
            Some(path) => path,
            None => return Err(VideoExporterError::InvalidPathBufStr),
        };

        let mut ffmpeg = ffmpeg_sidecar::command::FfmpegCommand::new()
            .input(input_path)
            .overwrite()
            .output(output_path)
            .spawn()?;

        let steps = match ffmpeg.iter() {
            Ok(steps) => steps,
            Err(err) => {
                ffmpeg.kill()?;
                return Err(err.into());
            }
        };

        for progress in steps.filter_progress() {
            let seconds = parse_formatted_seconds(progress.time);
            match (seconds.and_then(|s| s.to_f64()), data.duration().to_f64()) {
                (Some(num), Some(div)) => {
                    let percentage = (num / div).clamp(0.0, 1.0);
                    if let Some(percentage) = percentage.to_f32().take_if(|p| p.is_finite()) {
                        ffmpeg_progress(VideoProgress::Progress(percentage));
                    }
                }
                (_, _) => (),
            }
        }

        ffmpeg_progress(VideoProgress::Done);

        let result = ffmpeg.wait();

        temporary_dir.cleanup()?;

        match result {
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }
}
