use std::path::PathBuf;

use tokio::sync::{
    mpsc,
    oneshot::{Receiver, Sender},
};
use youtube_dl::Error;

pub async fn run_event_loop(mut rx: mpsc::UnboundedReceiver<AsyncAppEvent>) {
    while let Some(event) = rx.recv().await {
        match event {
            AsyncAppEvent::Shutdown => return,
            AsyncAppEvent::UpdateYtdlp(data_path, sender) => {
                tokio::spawn(update_yt_dlp(data_path, sender));
            }
        }
    }
}

async fn update_yt_dlp(path: PathBuf, sender: Sender<Result<(), Error>>) {
    let result = youtube_dl::downloader::download_yt_dlp(path)
        .await
        .map(|_| ());

    if let Err(_) = sender.send(result) {
        eprintln!("Failed to submit yt-dlp downloader result because sender failed");
    }
}

pub enum AsyncAppEvent {
    Shutdown,
    UpdateYtdlp(PathBuf, Sender<Result<(), Error>>),
}
