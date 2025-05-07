use super::*;
use ytconvertv2::{
    async_logic::AsyncAppEvent,
    error::{Error, ErrorSeverity},
};

pub fn update_yt_dlp(
    cx: &mut Context,
    submission_tx: tokio::sync::mpsc::UnboundedSender<AsyncAppEvent>,
) {
    let yt_dlp = AppData::yt_dlp_path.get(cx);
    cx.emit(
        Notification::new()
            .message("Updating yt-dlp...")
            .level(NotificationLevel::Info)
            .build(),
    );
    cx.spawn(move |cx| {
        let (tx, rx) = tokio::sync::oneshot::channel();
        if let Err(_) = submission_tx.send(AsyncAppEvent::UpdateYtdlp(yt_dlp, tx)) {
            if let Err(_) = cx.emit(
                Error::new()
                    .code(2)
                    .description("Failed to send yt-dlp download signal to tokio task")
                    .title("MPSC Channel Send Failure")
                    .severity(ErrorSeverity::Error)
                    .build(),
            ) {
                eprintln!("Could not submit error event, failed to send event");
            }
        }
        match rx.blocking_recv() {
            Err(_) => {
                if let Err(_) = cx.emit(
                    Error::new()
                        .code(1)
                        .description("Failed to recieve from yt-dlp downloader oneshot channel")
                        .title("Oneshot Channel Reciever Failure")
                        .severity(ErrorSeverity::Error)
                        .build(),
                ) {
                    eprintln!("Could not submit error event, failed to send event");
                }
            }
            Ok(result) => {
                if let Err(error) = result {
                    let msg = format!("Failed to download yt-dlp due to error: {error}");
                    if let Err(_) = cx.emit(
                        Error::new()
                            .code(300)
                            .description(msg)
                            .title("yt-dlp Download Failure")
                            .severity(ErrorSeverity::Error)
                            .build(),
                    ) {
                        eprintln!("Could not submit error event, failed to send event");
                    }
                } else {
                    if let Err(_) = cx.emit(
                        Notification::new()
                            .message("Updated yt-dlp successfully")
                            .level(NotificationLevel::Success)
                            .build(),
                    ) {
                        eprintln!("Could not submit error event, failed to send event");
                    };
                }
            }
        }
    });
}
