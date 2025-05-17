use super::*;

use ytconvertv2::events::AppEvent;

pub fn about_popup(cx: &mut Context) {
    Binding::new(cx, AppData::show_about, |cx, show| {
        if show.get(cx) {
            Window::new(cx, |cx| {
                AboutPopup::new(cx, AppData::theme);
            })
            .min_inner_size(Some((600, 400)))
            .max_inner_size(Some((600, 400)))
            .inner_size((600, 400))
            .resizable(false)
            .title("About")
            .on_close(|ex| ex.emit(AppEvent::SetShowAbout(false)));
        }
    });
}
