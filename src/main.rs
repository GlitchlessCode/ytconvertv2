use std::path::PathBuf;

use platform_dirs::AppDirs;
use reqwest::blocking::Client;
use rfd::FileDialog;
use vizia::{icons::ICON_FOLDER, prelude::*};
use ytconvertv2::{
    config::{ConfigEvent, ConfigModel},
    data::TaskQueue,
    helpers::labelled,
    include_bytes_safe,
    modifiers::ViewModifiers,
    theme::Theme,
    views::all::*,
};

#[derive(Lens)]
pub struct AppData {
    theme: Theme,
    task_queue: TaskQueue,

    current_location: Option<PathBuf>,

    #[cfg(windows)]
    maximized: bool,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event: &AppEvent, _meta| match event {
            #[cfg(windows)]
            AppEvent::Maximized(is_max) => self.maximized = *is_max,
            AppEvent::RequestLocationChange => {
                let current_location = self.current_location.clone();
                cx.spawn(|cxp| {
                    let mut dialog = FileDialog::new().set_title("Choose Export Location");
                    if let Some(path) = current_location {
                        dialog = dialog.set_directory(path);
                    }
                    if let Some(path) = dialog.pick_folder() {
                        if let Err(error) = cxp.emit(AppEvent::SetNewLocation(path)) {
                            eprintln!("Could not emit new location request, context proxy event emission failed due to error: {error}")
                        }
                    }
                });
            }

            AppEvent::SetNewLocation(new_path) => {
                self.current_location = Some(new_path.to_owned());
                cx.emit(ConfigEvent::SetExportPath(new_path.to_owned()));
            }

            #[allow(unreachable_patterns)]
            _ => (),
        });

        event.map(|event, _meta| match event {
            ConfigEvent::ConfigSetup { location } => {
                self.current_location = location.to_owned();
            }
            _ => (),
        })
    }
}

#[non_exhaustive]
enum AppEvent {
    #[cfg(windows)]
    Maximized(bool),
    RequestLocationChange,
    SetNewLocation(PathBuf),
}

static CORNER_SIZE: Units = Pixels(6.0);

fn main() -> Result<(), ApplicationError> {
    let dirs = AppDirs::new(Some("ytconvertv2"), false);

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
                .background("#252525")
                .background_dark("#151515")
                .background_light("#2f2f2f")
                .border("#444444")
                .primary("gold")
                .text_primary("#eeeeee")
                .text_secondary("#aaaaaa")
                .text_light("#575757")
                .build(),

            task_queue: TaskQueue::default(),

            current_location: None,

            #[cfg(windows)]
            maximized: false,
        }
        .build(cx); // Build the data into the app

        ConfigModel::open_config_path(dirs).build(cx);

        cx.emit(ConfigEvent::RequestSetup);

        // Layer Stack
        ZStack::new(cx, |cx| {
            // Main element stack
            #[allow(unused)]
            let mut window = VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    // Main Toolbar
                    Toolbar::new(cx, AppData::theme)
                        .on_exit(|ex| ex.emit(WindowEvent::WindowClose));

                    // Sub tool stack
                    HStack::new(cx, |cx| {
                        VStack::new(cx, |cx| {
                            labelled(cx, AppData::theme, "Export Path", |cx| {
                                HStack::new(cx, |cx| {
                                    ScrollView::new(cx, |cx| {
                                        Label::new(
                                            cx,
                                            AppData::current_location.map(|loc| {
                                                if let Some(path) = loc {
                                                    path.display().to_string()
                                                } else {
                                                    "No Path Chosen".to_string()
                                                }
                                            }),
                                        )
                                        .padding(Pixels(6.0))
                                        .color(AppData::theme.map(|theme| theme.text_primary));
                                    })
                                    .width(Stretch(1.0))
                                    .show_horizontal_scrollbar(true);

                                    Button::new(cx, |cx| Svg::new(cx, ICON_FOLDER))
                                        .alignment(Alignment::Center)
                                        .background_color(AppData::theme.map(|theme| theme.primary))
                                        .on_press(|ex| ex.emit(AppEvent::RequestLocationChange));
                                })
                                .alignment(Alignment::Left)
                                .height(Units::Auto)
                                .round_box(AppData::theme)
                                .background_color(
                                    AppData::theme.map(|theme| theme.background_dark),
                                );
                            })
                            .height(Auto);

                            labelled(cx, AppData::theme, "Export Settings", |cx| {
                                VStack::new(cx, |cx| {})
                                    .round_box(AppData::theme)
                                    .background_color(
                                        AppData::theme.map(|theme| theme.background_light),
                                    )
                                    .padding(Pixels(6.0));
                            })
                            .height(Stretch(1.0));
                        })
                        .gap(Pixels(3.0))
                        .width(Stretch(1.0));

                        VStack::new(cx, |cx| {
                            labelled(cx, AppData::theme, "Tasks", |cx| {
                                TaskQueueView::new(cx, AppData::theme, AppData::task_queue)
                                    .round_box(AppData::theme)
                                    .background_color(
                                        AppData::theme.map(|theme| theme.background_light),
                                    );
                            })
                            .height(Stretch(1.0));

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
            .position_type(PositionType::Relative);

            #[cfg(windows)]
            {
                window = window.padding(Pixels(4.0));
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
