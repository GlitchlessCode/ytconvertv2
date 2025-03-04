use reqwest::blocking::Client;
use vizia::prelude::*;
use ytconvertv2::{include_bytes_safe, theme::Theme, views::all::*};

#[derive(Lens)]
pub struct AppData {
    theme: Theme,
    #[cfg(windows)]
    maximized: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        #[cfg(windows)]
        {
            event.map(|event, _meta| match event {
                AppEvent::Maximized(is_max) => self.maximized = *is_max,
            });
        }
    }
}

enum AppEvent {
    #[cfg(windows)]
    Maximized(bool),
}

static CORNER_SIZE: Units = Pixels(6.0);

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        #[cfg(windows)]
        {
            // Check for maximization
            let maximized_timer = cx.add_timer(Duration::from_millis(200), None, |ex, _action| {
                let mut proxy = ex.get_proxy();
                ex.modify_window(|window| {
                    if let Err(error) = proxy.emit(AppEvent::Maximized(window.is_maximized())) {
                        eprintln!("{error}")
                    }
                });
            });

            cx.start_timer(maximized_timer);
        }

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
            #[cfg(windows)]
            maximized: false,
        }
        .build(cx); // Build the data into the app

        ZStack::new(cx, |cx| {
            // Main element stack
            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    // Main Toolbar
                    Toolbar::new(cx, AppData::theme)
                        .on_exit(|ex| ex.emit(WindowEvent::WindowClose));

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
                .gap(Pixels(3.0))
                .corner_radius(CORNER_SIZE);
            })
            .padding(Pixels(4.0))
            .position_type(PositionType::Relative);

            #[cfg(windows)]
            {
                ResizerGroup::new(cx).display(AppData::maximized.map(|max| !max));
            }
        });
    })
    .min_inner_size(Some((400, 300)))
    .ignore_default_theme()
    .transparent(true)
    .decorations(false)
    .run()
}
