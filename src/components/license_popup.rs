use super::*;

use ytconvertv2::{events::AppEvent, views::licensepopup::LicensePopup};

pub fn license_popup(cx: &mut Context) {
    Binding::new(cx, AppData::show_licenses, |cx, show| {
        if show.get(cx) {
            Window::new(cx, |cx| {
                LicensePopup::new(cx, AppData::theme, AppData::licenses);
            })
            .min_inner_size(Some((600, 400)))
            .title("Licenses")
            .on_close(|ex| ex.emit(AppEvent::SetShowLicense(false)));
        }
    });
}
