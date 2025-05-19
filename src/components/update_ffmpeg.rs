use super::*;

use std::path::PathBuf;
use ytconvertv2::{
    data::FfmpegInstallState,
    error::{Error, ErrorSeverity},
    events::AppEvent,
    helpers::ContextProxyExt,
};

pub fn update_ffmpeg(cx: &mut Context) {
    let ffmpeg = AppData::ffmpeg_path.get(cx);

    cx.emit(
        Notification::new()
            .message("Checking ffmpeg for updates")
            .level(NotificationLevel::Info)
            .build(),
    );

    cx.spawn(|cx| {
        match update_ffmpeg_inner(cx, ffmpeg.clone()) {
            Ok(updated) => {
                if !updated {
                    cx.emit_print_err(
                        Notification::new()
                            .message("ffmpeg is already up to date")
                            .level(NotificationLevel::Success)
                            .build(),
                    );
                    cx.emit_print_err(AppEvent::SetFfmpegInstallState(
                        FfmpegInstallState::Installed,
                    ));
                    return;
                }
            }
            Err(err) => {
                cx.emit_print_err(
                    Error::new()
                        .code(100)
                        .title("Dependency ffmpeg Installation Failed")
                        .severity(ErrorSeverity::Error)
                        .description(format!("Failed to install ffmpeg due an error: {err}"))
                        .build(),
                );
                cx.emit_print_err(AppEvent::SetFfmpegInstallState(FfmpegInstallState::Failed));
                return;
            }
        };

        // Error, ffmpeg should now be installed
        if !ffmpeg_is_installed(ffmpeg) {
            let msg =
                "Cannot detect an ffmpeg installation after completing the installation process.";

            cx.emit_print_err(
                Error::new()
                    .code(101)
                    .title("Dependency ffmpeg Unexpectedly Missing")
                    .severity(ErrorSeverity::Error)
                    .description(msg)
                    .footer(|cx| install_ffmpeg_footer(cx))
                    .build(),
            );
            cx.emit_print_err(
                Notification::new()
                    .message("Please try installing ffmpeg manually")
                    .level(NotificationLevel::Error)
                    .build(),
            );
            cx.emit_print_err(AppEvent::SetFfmpegInstallState(FfmpegInstallState::Missing));
        } else {
            cx.emit_print_err(
                Notification::new()
                    .message("Updated ffmpeg successfully")
                    .level(NotificationLevel::Success)
                    .build(),
            );
            cx.emit_print_err(AppEvent::SetFfmpegInstallState(
                FfmpegInstallState::Installed,
            ));
        }
    });
}

fn install_ffmpeg_footer(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Download ffmpeg..."))
            .on_press(|_| {
                if let Err(_) = open::that("https://www.ffmpeg.org/download.html") {
                    eprintln!("Error opening url");
                }
            })
            .width(Stretch(1.0));
    })
    .width(Stretch(1.0));
}

fn update_ffmpeg_inner(
    cx: &mut ContextProxy,
    ffmpeg: PathBuf,
) -> Result<bool, Box<dyn std::error::Error>> {
    use ffmpeg_sidecar::download::{
        check_latest_version, download_ffmpeg_package, ffmpeg_download_url, unpack_ffmpeg,
    };
    use ffmpeg_sidecar::version::ffmpeg_version_with_path;

    if ffmpeg_is_installed(ffmpeg.clone()) {
        let current_version = ffmpeg_version_with_path(ffmpeg.join(ffmpeg_exec()))?;
        let latest_version = check_latest_version()?;

        if !current_version.starts_with(&latest_version) {
            cx.emit_print_err(
                Notification::new()
                    .message("Updating ffmpeg...")
                    .level(NotificationLevel::Info)
                    .build(),
            );
            cx.emit_print_err(AppEvent::SetFfmpegInstallState(
                FfmpegInstallState::Updating,
            ));
            let download_url = ffmpeg_download_url()?;
            let destination = ffmpeg.clone();
            std::fs::create_dir_all(&ffmpeg)?;
            let archive_path = download_ffmpeg_package(download_url, &destination)?;
            unpack_ffmpeg(&archive_path, &destination)?;
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        cx.emit_print_err(
            Notification::new()
                .message("Installing ffmpeg...")
                .level(NotificationLevel::Info)
                .build(),
        );
        cx.emit_print_err(AppEvent::SetFfmpegInstallState(
            FfmpegInstallState::Installing,
        ));
        let download_url = ffmpeg_download_url()?;
        let destination = ffmpeg.clone();
        std::fs::create_dir_all(&ffmpeg)?;
        let archive_path = download_ffmpeg_package(download_url, &destination)?;
        unpack_ffmpeg(&archive_path, &destination)?;

        Ok(true)
    }
}

fn ffmpeg_is_installed(ffmpeg: PathBuf) -> bool {
    std::process::Command::new(ffmpeg.join(ffmpeg_exec()))
        .arg("-version")
        .create_no_window()
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or_else(|_| false)
}

trait BackgroundCommand {
    fn create_no_window(&mut self) -> &mut Self;
}

impl BackgroundCommand for std::process::Command {
    /// Disable creating a new console window for the spawned process on Windows.
    /// Has no effect on other platforms. This can be useful when spawning a command
    /// from a GUI program.
    fn create_no_window(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        std::os::windows::process::CommandExt::creation_flags(self, 0x08000000);
        self
    }
}

#[inline]
fn ffmpeg_exec() -> &'static str {
    if cfg!(windows) {
        "ffmpeg.exe"
    } else {
        "ffmpeg"
    }
}
