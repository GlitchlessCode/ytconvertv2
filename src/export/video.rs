use crate::{
    data::{VideoData, VideoExportSettings},
    error::ErrorSeverity,
    helpers::parse_formatted_seconds,
};
use bon::bon;
use ffmpeg_sidecar::event::{FfmpegEvent, LogLevel};
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
/// Error emitted while exporting a video
pub enum VideoExporterError {
    // Transparent
    #[error(transparent)]
    /// A standard library IO error
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    /// An error emitted by yt-dlp
    YoutubeDlError(#[from] youtube_dl::Error),

    #[error(transparent)]
    /// A generic anyhow error
    AnyhowError(#[from] anyhow::Error),

    // Custom
    #[error("temporary directory is either missing or is not a directory")]
    /// Emitted when the temporary directory provided by temp_dir is not found
    TempDirMissing,

    #[error("file exported from yt-dlp is missing")]
    /// Emitted when the file that should have been exported to the temporary directory is missing
    ExportMissing,

    #[error("there are multiple files that could be the target file from yt-dlp")]
    /// Emitted when the temporary directory has multiple files with the same file stem that is expected
    AmbiguousTempFile,

    #[error("converting a `PathBuf` to str unexepectedly returned `None`")]
    /// Emitted when a PathBuf cannot be converted to str
    InvalidPathBufStr,
}

// For ease of converting a VideoExporterError into an error that can be handled by the error manager
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

/// Video processing progress, either a float from 0.0 to 1.0, or Done
pub enum VideoProgress {
    Progress(f32),
    Done,
}

/// The state machine used for exporting a single video
pub struct VideoExporter<S> {
    state: S,
}

// Bon builder to make a VideoExporter
#[bon]
impl VideoExporter<Unused> {
    #[builder]
    pub fn new<F: Fn(VideoProgress) + Send + Sync + 'static>(
        export_location: PathBuf,
        data: VideoData,
        settings: VideoExportSettings,
        #[builder(into)] yt_dlp_path: PathBuf,
        #[builder(into)] ffmpeg_path: PathBuf,
        ffmpeg_progress: F,
    ) -> VideoExporter<Unused> {
        VideoExporter {
            state: Unused {
                export_location,
                data,
                settings,
                yt_dlp_path,
                ffmpeg_path,
                ffmpeg_progress: Box::new(ffmpeg_progress),
            },
        }
    }
}

/// First state, when the state machine is unused
pub struct Unused {
    export_location: PathBuf,
    data: VideoData,
    settings: VideoExportSettings,
    yt_dlp_path: PathBuf,
    ffmpeg_path: PathBuf,
    ffmpeg_progress: Box<dyn Fn(VideoProgress) + Send + Sync + 'static>,
}

// Fetch the video to a temporary directory
impl VideoExporter<Unused> {
    pub fn fetch_video(self) -> Result<VideoFetched> {
        // Unpack state
        let Unused {
            export_location,
            data,
            settings,
            yt_dlp_path,
            ffmpeg_path,
            ffmpeg_progress,
        } = self.state;

        // Prep yt-dlp
        let mut ytdl = YoutubeDl::new(data.url());

        let name = nanoid!();

        ytdl.youtube_dl_path(yt_dlp_path)
            .flat_playlist(true)
            .output_template(format!("{name}"))
            .extra_arg("--no-playlist")
            .extra_arg("--ffmpeg-location")
            .extra_arg(ffmpeg_sidecar::paths::ffmpeg_path().to_string_lossy());

        if settings.export_type.is_audio() {
            ytdl.format("ba");
        }

        // Create temporary directory
        let temporary_dir = TempDir::new()?;

        // Run yt-dlp
        ytdl.download_to(temporary_dir.path())?;

        // Return new state
        Ok(VideoExporter {
            state: VideoFetched {
                export_location,
                data,
                settings,
                ffmpeg_path,
                ffmpeg_progress,
                temporary_dir,
                id_name: name,
            },
        })
    }
}

/// Second state, once the video is fetched to the temporary directory
pub struct VideoFetched {
    export_location: PathBuf,
    data: VideoData,
    settings: VideoExportSettings,
    ffmpeg_path: PathBuf,
    ffmpeg_progress: Box<dyn Fn(VideoProgress) + Send + Sync + 'static>,
    temporary_dir: TempDir,
    id_name: String,
}

// Finds and grabs the exported video
impl VideoExporter<VideoFetched> {
    pub fn grab_filepath(self) -> Result<PathGrabbed> {
        // Unpack state
        let VideoFetched {
            export_location,
            data,
            settings,
            ffmpeg_path,
            ffmpeg_progress,
            temporary_dir,
            id_name,
        } = self.state;

        // Try to find the expected file
        let temp_dir_path = temporary_dir.path();

        let input_path = if temp_dir_path.is_dir() {
            match_file_stem(temp_dir_path, id_name)?
        } else {
            return Err(VideoExporterError::TempDirMissing);
        };

        // Return new state
        Ok(VideoExporter {
            state: PathGrabbed {
                export_location,
                data,
                settings,
                ffmpeg_path,
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
    // Start with no file found
    let mut found: Option<PathBuf> = None;

    // For each file in the directory
    for file in read_dir(dir_path)? {
        if let Ok(file) = file {
            if let Some(stem) = file.path().file_stem() {
                // If the file doesn't match the search string, keep lookig
                if &stem.to_string_lossy() != target.as_str() {
                    continue;
                }

                // Otherwise, if a different file has already been found, return an error
                if found.is_some() {
                    return Err(VideoExporterError::AmbiguousTempFile);
                }

                // Otherwise, store this filepath
                found = Some(file.path())
            }
        }
    }

    // If no file was found, return an error
    match found {
        None => Err(VideoExporterError::ExportMissing),
        Some(found) => Ok(found),
    }
}

/// Third state, after the file has been grabbed
pub struct PathGrabbed {
    export_location: PathBuf,
    data: VideoData,
    settings: VideoExportSettings,
    ffmpeg_path: PathBuf,
    ffmpeg_progress: Box<dyn Fn(VideoProgress) + Send + Sync + 'static>,
    temporary_dir: TempDir,
    input_path: PathBuf,
}

// Runs the fetched video through FFMPEG
impl VideoExporter<PathGrabbed> {
    pub fn final_export(self) -> std::result::Result<(), VideoExporterError> {
        // Unpack state
        let PathGrabbed {
            export_location,
            data,
            settings,
            ffmpeg_path,
            ffmpeg_progress,
            temporary_dir,
            input_path,
        } = self.state;

        // Get input and output paths as strings
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

        // Prep ffmpeg
        let mut ffmpeg = ffmpeg_sidecar::command::FfmpegCommand::new_with_path(ffmpeg_path)
            .input(input_path)
            .overwrite()
            .output(output_path)
            .spawn()?;

        // Convert to iterator
        let steps = match ffmpeg.iter() {
            Ok(steps) => steps,
            Err(err) => {
                ffmpeg.kill()?;
                return Err(err.into());
            }
        };

        let steps = steps
            .filter_map(|msg| match msg {
                FfmpegEvent::Log(LogLevel::Info, info) => Some(info),
                FfmpegEvent::Progress(progress) => Some(format!("time={}", progress.time)),
                _ => None,
            })
            .filter_map(|info| {
                info.split_whitespace()
                    .find_map(|s| s.strip_prefix("time=").map(|s| s.to_string()))
            })
            .filter_map(|seconds| parse_formatted_seconds(seconds))
            .filter_map(|seconds| seconds.to_f64());

        if let Some(total) = data.duration().to_f64() {
            for seconds in steps {
                let percentage = (seconds / total).clamp(0.0, 1.0);
                if let Some(percentage) = percentage.to_f32().take_if(|p| p.is_finite()) {
                    ffmpeg_progress(VideoProgress::Progress(percentage));
                }
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
