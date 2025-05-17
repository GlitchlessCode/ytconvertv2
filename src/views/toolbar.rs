use vizia::icons::{
    ICON_ALERT_HEXAGON_FILLED, ICON_BUG_FILLED, ICON_FILE, ICON_LICENSE, ICON_QUESTION_MARK,
    ICON_SETTINGS, ICON_X,
};

use crate::{
    error::{ErrorManager, ErrorManagerEvent},
    events::ToolbarEvent,
    modifiers::{menu::MenuStyleModifier, ThemeModifiers, ViewModifiers},
};

use super::*;

pub struct Toolbar {
    on_exit: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl View for Toolbar {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _meta| match event {
            ToolbarEvent::ExitApp => {
                if let Some(callback) = &self.on_exit {
                    (callback)(cx);
                }
            }
            _ => (),
        });
    }

    fn element(&self) -> Option<&'static str> {
        Some("toolbar")
    }
}

impl Toolbar {
    pub fn new<T>(cx: &mut Context, theme: T) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
    {
        Self { on_exit: None }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    MenuBar::new(cx, |cx| {
                        Submenu::new(
                            cx,
                            move |cx| Label::new(cx, "ytconvertv2").with_text_primary(theme),
                            move |cx| {
                                MenuButton::new(
                                    cx,
                                    |ex| ex.emit(ToolbarEvent::ShowAbout),
                                    move |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_QUESTION_MARK)
                                                .fill(theme.map(|theme| theme.primary));
                                            Label::new(cx, "About");
                                        })
                                        .class("inner")
                                        .corner_radius(Pixels(3.0))
                                    },
                                )
                                .padding(Pixels(0.0))
                                .default_menu_style(theme)
                                .round_top(Pixels(3.0));

                                MenuButton::new(
                                    cx,
                                    |ex| ex.emit(ToolbarEvent::ShowSettings),
                                    move |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_SETTINGS)
                                                .fill(theme.map(|theme| theme.primary));
                                            Label::new(cx, "Settings");
                                        })
                                        .class("inner")
                                        .corner_radius(Pixels(3.0))
                                    },
                                )
                                .padding(Pixels(0.0))
                                .default_menu_style(theme);

                                MenuButton::new(
                                    cx,
                                    |ex| ex.emit(ToolbarEvent::ShowLicense),
                                    move |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_LICENSE)
                                                .fill(theme.map(|theme| theme.primary));
                                            Label::new(cx, "Licenses");
                                        })
                                        .class("inner")
                                        .corner_radius(Pixels(3.0))
                                    },
                                )
                                .padding(Pixels(0.0))
                                .default_menu_style(theme)
                                .round_bottom(Pixels(3.0));
                            },
                        )
                        .with_text_primary(theme)
                        .class("menubutton");
                        // Submenu::new(cx, move |cx| Label::new(cx, "View").with_text_primary(theme), |cx| {})
                        //     .with_text_primary(theme)
                        //     .class("menubutton");
                        Submenu::new(
                            cx,
                            move |cx| Label::new(cx, "Help").with_text_primary(theme),
                            move |cx| {
                                MenuButton::new(
                                    cx,
                                    |ex| ex.emit(ToolbarEvent::OpenIssuesPage),
                                    move |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_BUG_FILLED)
                                                .fill(theme.map(|theme| theme.primary));
                                            Label::new(cx, "Report Bugs");
                                        })
                                        .class("inner")
                                        .corner_radius(Pixels(3.0))
                                    },
                                )
                                .padding(Pixels(0.0))
                                .default_menu_style(theme);

                                MenuButton::new(
                                    cx,
                                    |ex| ex.emit(ToolbarEvent::OpenDocumentationPage),
                                    move |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_FILE)
                                                .fill(theme.map(|theme| theme.primary));
                                            Label::new(cx, "Documentation");
                                        })
                                        .class("inner")
                                        .corner_radius(Pixels(3.0))
                                    },
                                )
                                .padding(Pixels(0.0))
                                .default_menu_style(theme);
                            },
                        )
                        .with_text_primary(theme)
                        .class("menubutton");
                    })
                    .alignment(Alignment::Left)
                    .pointer_events(true);

                    Button::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Svg::new(cx, ICON_ALERT_HEXAGON_FILLED);
                            Label::new(
                                cx,
                                ErrorManager::errors.map(|errors| format!("{}", errors.len())),
                            );
                        })
                    })
                    .class("error-tag")
                    .toggle_class(
                        "error-tag-warn",
                        ErrorManager::warning_count.map(|warns| *warns > 0),
                    )
                    .toggle_class(
                        "error-tag-error",
                        ErrorManager::error_count.map(|errs| *errs > 0),
                    )
                    .pointer_events(true)
                    .on_press(|ex| ex.emit(ErrorManagerEvent::SetPopup(true)));

                    Element::new(cx).width(Stretch(1.0));

                    Button::new(cx, |cx| Svg::new(cx, ICON_X))
                        .class("exit")
                        .pointer_events(true)
                        .on_press(|ex| ex.emit(ToolbarEvent::ExitApp));
                })
                .padding_right(Pixels(4.0))
                .padding_left(Pixels(4.0))
                .pointer_events(false)
                .gap(Pixels(4.0))
                .class("toolbar")
                .alignment(Alignment::Center);
            })
            .width(Percentage(100.0))
            .height(Pixels(32.0))
            .on_background_dark(theme)
            .round_box(theme)
            .on_press_down(|ex| ex.emit(WindowEvent::DragWindow))
    }
}

pub trait ToolbarModifers {
    fn on_exit<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self;
}

impl<'a> ToolbarModifers for Handle<'a, Toolbar> {
    fn on_exit<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self {
        self.modify(|toolbar| toolbar.on_exit = Some(Box::new(callback)))
    }
}
