use velopack::{sources::HttpSource, UpdateCheck, UpdateManager};
use vizia::context::{EmitContext, EventContext};
use ytconvertv2::{
    error::{Error, ErrorSeverity},
    events::GlobalEvent,
    views::all::{Notification, NotificationLevel},
};

pub fn check_for_updates(cx: &mut EventContext) {
    cx.emit(
        Notification::new()
            .message("Checking ytconvertv2 for updates...")
            .level(NotificationLevel::Info)
            .build(),
    );

    let source =
        HttpSource::new("https://github.com/GlitchlessCode/ytconvertv2/releases/latest/download/");
    let manager = match UpdateManager::new(source, None, None) {
        Ok(manager) => manager,
        Err(err) => return update_err(cx, Some(err)),
    };

    let update_check = match manager.check_for_updates() {
        Ok(updates) => updates,
        Err(err) => return update_err(cx, Some(err)),
    };

    let update = match update_check {
        UpdateCheck::UpdateAvailable(update) => update,
        _ => {
            return cx.emit(
                Notification::new()
                    .message("No update found for ytconvertv2")
                    .level(NotificationLevel::Info)
                    .build(),
            )
        }
    };

    if let Err(err) = manager.download_updates(&update, None) {
        return update_err(cx, Some(err));
    }

    cx.emit(GlobalEvent::RestartForUpdate(manager, update));
}

fn update_err(cx: &mut EventContext, err: Option<impl std::error::Error>) {
    let error = Error::new()
        .title("Velopack Update Failed")
        .code(999)
        .severity(ErrorSeverity::Error);

    if let Some(err) = err {
        cx.emit(error.description(err.to_string()).build())
    } else {
        cx.emit(error.build());
    }

    cx.emit(
        Notification::new()
            .message("Update Failed")
            .level(NotificationLevel::Error)
            .build(),
    )
}
