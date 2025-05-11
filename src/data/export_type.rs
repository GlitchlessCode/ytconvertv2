use super::*;

#[derive(Clone, Data, PartialEq, Debug)]
pub enum ExportType {
    Audio(AudioExtension),
    Video(VideoExtension),
}

impl ExportType {
    pub fn is_audio(&self) -> bool {
        match self {
            Self::Audio(_) => true,
            Self::Video(_) => false,
        }
    }

    pub fn is_video(&self) -> bool {
        match self {
            Self::Audio(_) => false,
            Self::Video(_) => true,
        }
    }

    pub fn formats(&self) -> Vec<ExportType> {
        match self {
            Self::Audio(_) => vec![
                AudioExtension::Mp3.into(),
                AudioExtension::Wav.into(),
                AudioExtension::Ogg.into(),
            ],
            Self::Video(_) => vec![
                VideoExtension::Mp4.into(),
                VideoExtension::Mov.into(),
                VideoExtension::Mkv.into(),
            ],
        }
    }
}

impl std::fmt::Display for ExportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Audio(a) => a.to_string(),
                Self::Video(v) => v.to_string(),
            }
        )
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum AudioExtension {
    Mp3,
    Wav,
    Ogg,
}

impl std::fmt::Display for AudioExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".{}",
            match self {
                Self::Mp3 => "mp3",
                Self::Wav => "wav",
                Self::Ogg => "ogg",
            }
        )
    }
}

impl From<AudioExtension> for ExportType {
    fn from(value: AudioExtension) -> Self {
        ExportType::Audio(value)
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum VideoExtension {
    Mp4,
    Mov,
    Mkv,
}

impl std::fmt::Display for VideoExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            ".{}",
            match self {
                Self::Mp4 => "mp4",
                Self::Mov => "mov",
                Self::Mkv => "mkv",
            }
        )
    }
}

impl From<VideoExtension> for ExportType {
    fn from(value: VideoExtension) -> Self {
        ExportType::Video(value)
    }
}
