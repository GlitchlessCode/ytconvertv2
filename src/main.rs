use reqwest::blocking::Client;
use vizia::prelude::*;
use ytconvertv2::{include_bytes_safe, theme::Theme, views::all::*};

#[derive(Lens)]
pub struct AppData {
    theme: Theme,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}
}

static CORNER_SIZE: Units = Pixels(6.0);

fn main() -> Result<(), ApplicationError> {
    #[allow(unused_mut)]
    let mut app = Application::new(|cx| {
        // Add font
        let bytes = include_bytes_safe!("font", "Inter-VariableFont_opsz,wght.ttf");
        cx.add_font_mem(bytes);

        // Add stylesheet
        cx.add_stylesheet(include_style!("src/style/style.css"))
            .expect("Failed to load stylesheet");

        // Build in root data
        AppData {
            // Build app theme
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

        // Main element stack
        let stack = VStack::new(cx, |cx| {
            // Main Toolbar
            Toolbar::new(cx, AppData::theme).on_exit(|ex| ex.emit(WindowEvent::WindowClose));

            // Sub toolbar stack
            HStack::new(cx, |cx| {
                Divider::new(cx).background_color(AppData::theme.map(|theme| theme.border));
            });
        })
        .background_color(AppData::theme.map(|theme| theme.background))
        .border_color(AppData::theme.map(|theme| theme.border))
        .border_width(Pixels(1.0))
        .padding(CORNER_SIZE)
        .size(Percentage(100.0))
        .gap(Pixels(3.0));

        #[cfg(not(windows))]
        {
            stack.corner_radius(CORNER_SIZE);
        }
    })
    .min_inner_size(Some((600, 400)))
    .ignore_default_theme()
    .transparent(true);

    #[cfg(not(windows))]
    {
        app = app.decorations(false);
    }

    app.run()
}
