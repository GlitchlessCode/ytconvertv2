use super::*;

#[derive(Data, Clone, PartialEq)]
pub enum FfmpegInstallState {
    Installed,
    Installing,
    Updating,

    Failed,
    Missing,
    Unknown,
}

impl std::fmt::Display for FfmpegInstallState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Installed => "Installed",
                Self::Installing => "Installing",
                Self::Updating => "Updating",

                Self::Failed => "Failed",
                Self::Missing => "Missing",
                Self::Unknown => "Unknown",
            }
        )
    }
}
