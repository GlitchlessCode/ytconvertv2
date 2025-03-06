use reqwest::blocking::Client;
use vizia::prelude::*;
use ytconvertv2::{
    data::TaskQueue, include_bytes_safe, modifiers::ViewModifiers, theme::Theme, views::all::*,
};

#[derive(Lens)]
pub struct AppData {
    theme: Theme,
    task_queue: TaskQueue,

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
        let bytes = include_bytes_safe!("font", "Quicksand-Medium.ttf");
        cx.add_font_mem(bytes);

        // Add stylesheet
        cx.add_stylesheet(include_style!("src/style/style.css"))
            .expect("Failed to load stylesheet");

        // Build in root data
        AppData {
            // Build app theme
            theme: Theme::builder()
                .background("#020e31")
                .background_dark("#010829")
                .background_light("#031339")
                .border("#344855")
                .primary("gold")
                .text_primary("#eeeeee")
                .text_light("#57577f")
                .build(),

            task_queue: TaskQueue::default(),

            #[cfg(windows)]
            maximized: false,
        }
        .build(cx); // Build the data into the app

        // Layer Stack
        ZStack::new(cx, |cx| {
            // Main element stack
            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    // Main Toolbar
                    Toolbar::new(cx, AppData::theme)
                        .on_exit(|ex| ex.emit(WindowEvent::WindowClose));

                    // Sub tool stack
                    HStack::new(cx, |cx| {
                        VStack::new(cx, |cx| {})
                            .round_box(AppData::theme)
                            .width(Stretch(1.0))
                            .background_color(AppData::theme.map(|theme| theme.background_light));
                        VStack::new(cx, |cx| {
                            TaskQueueView::new(cx, AppData::theme, AppData::task_queue)
                                .round_box(AppData::theme)
                                .height(Stretch(1.0))
                                .background_color(
                                    AppData::theme.map(|theme| theme.background_light),
                                );

                            VStack::new(cx, |cx| {
                                //ProgressBar goes here
                            })
                            .round_box(AppData::theme)
                            .height(Pixels(120.0))
                            .background_color(AppData::theme.map(|theme| theme.background_light));
                        })
                        .gap(Pixels(6.0))
                        .width(Pixels(290.0));
                    })
                    .gap(Pixels(6.0));
                })
                .background_color(AppData::theme.map(|theme| theme.background))
                .border_color(AppData::theme.map(|theme| theme.border))
                .border_width(Pixels(1.0))
                .padding(CORNER_SIZE)
                .size(Percentage(100.0))
                .gap(Pixels(6.0))
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
    // .ignore_default_theme()
    .transparent(true)
    .decorations(false)
    .run()
}
