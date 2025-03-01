use reqwest::blocking::Client;
use vizia::prelude::*;
use ytconvertv2::{theme::Theme, views::Toolbar};

#[derive(Lens)]
pub struct AppData {
    theme: Theme,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}
}

static CORNER_SIZE: Units = Pixels(8.0);

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style/style.css"))
            .expect("Failed to load stylesheet");

        AppData {
            theme: Theme::builder()
                .background("#020e25")
                .background_dark("#010819")
                .background_light("#03132f")
                .border("#344855")
                .primary("gold")
                .text_primary("#eeeeee")
                .build(),
        }
        .build(cx); // Build the data into the app

        VStack::new(cx, |cx| {
            Toolbar::new(cx, AppData::theme);

            HStack::new(cx, |cx| {
                Divider::new(cx).background_color(AppData::theme.map(|theme| theme.border));
            });
        })
        .background_color(AppData::theme.map(|theme| theme.background))
        .border_color(AppData::theme.map(|theme| theme.border))
        .border_width(Pixels(1.0))
        .padding(CORNER_SIZE)
        .size(Percentage(100.0))
        .corner_radius(CORNER_SIZE)
        .gap(Pixels(3.0));
    })
    .ignore_default_theme()
    .transparent(true)
    .decorations(false)
    .run()
}
