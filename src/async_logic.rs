use tokio::sync::mpsc;

pub async fn run_event_loop(mut rx: mpsc::UnboundedReceiver<AsyncAppEvent>) {
    while let Some(event) = rx.recv().await {
        match event {
            AsyncAppEvent::Shutdown => return,
        }
    }
}

pub enum AsyncAppEvent {
    Shutdown,
}
