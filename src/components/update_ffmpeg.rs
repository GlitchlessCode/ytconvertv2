use ytconvertv2::{
    data::FfmpegInstallState,
    error::{Error, ErrorSeverity},
    events::AppEvent,
    helpers::ContextProxyExt,
};

use super::*;

pub fn update_ffmpeg(cx: &mut Context) {
    use ffmpeg_sidecar::command::ffmpeg_is_installed;

    cx.emit(
        Notification::new()
            .message("Checking ffmpeg for updates")
            .level(NotificationLevel::Info)
            .build(),
    );

    cx.spawn(|cx| {
        match update_ffmpeg_inner(cx) {
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
            Err(_) => {
                cx.emit_print_err(
                    Error::new()
                        .code(100)
                        .title("Dependency ffmpeg Installation Failed")
                        .severity(ErrorSeverity::Error)
                        .description("Failed to install ffmpeg due to an unknown error.")
                        .build(),
                );
                cx.emit_print_err(AppEvent::SetFfmpegInstallState(FfmpegInstallState::Failed));
                return;
            }
        };

        // Error, ffmpeg should now be installed
        if !ffmpeg_is_installed() {
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

fn update_ffmpeg_inner(cx: &mut ContextProxy) -> Result<bool, ()> {
    use ffmpeg_sidecar::command::ffmpeg_is_installed;
    use ffmpeg_sidecar::download::{
        check_latest_version, download_ffmpeg_package, ffmpeg_download_url, unpack_ffmpeg,
    };
    use ffmpeg_sidecar::paths::sidecar_dir;
    use ffmpeg_sidecar::version::ffmpeg_version;

    if ffmpeg_is_installed() {
        let current_version = ffmpeg_version().map_err(|_| ())?;
        let latest_version = check_latest_version().map_err(|_| ())?;

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
            let download_url = ffmpeg_download_url().map_err(|_| ())?;
            let destination = sidecar_dir().map_err(|_| ())?;
            let archive_path =
                download_ffmpeg_package(download_url, &destination).map_err(|_| ())?;
            unpack_ffmpeg(&archive_path, &destination).map_err(|_| ())?;
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
        let download_url = ffmpeg_download_url().map_err(|_| ())?;
        let destination = sidecar_dir().map_err(|_| ())?;
        let archive_path = download_ffmpeg_package(download_url, &destination).map_err(|_| ())?;
        unpack_ffmpeg(&archive_path, &destination).map_err(|_| ())?;

        Ok(true)
    }
}
