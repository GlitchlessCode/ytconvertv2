use velopack::{UpdateInfo, UpdateManager};

pub enum GlobalEvent {
    CheckForUpdates,
    RestartForUpdate(UpdateManager, UpdateInfo),
}
