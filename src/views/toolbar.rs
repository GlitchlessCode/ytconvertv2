use vizia::icons::{
    ICON_ALERT_HEXAGON_FILLED, ICON_BUG_FILLED, ICON_FILE, ICON_LICENSE, ICON_QUESTION_MARK, ICON_X,
};

use crate::{
    error::{ErrorManager, ErrorManagerEvent},
    modifiers::menu::MenuStyleModifier,
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
    // TODO - Finish the toolbar
    pub fn new<T>(cx: &mut Context, theme: T) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
    {
        Self { on_exit: None }
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    MenuBar::new(cx, |cx| {
                        Submenu::new(cx, |cx| Label::new(cx, "File"), |cx| {})
                            .color(theme.map(|theme| theme.text_primary))
                            .class("menubutton");
                        Submenu::new(cx, |cx| Label::new(cx, "View"), |cx| {})
                            .color(theme.map(|theme| theme.text_primary))
                            .class("menubutton");
                        Submenu::new(
                            cx,
                            |cx| Label::new(cx, "Help"),
                            move |cx| {
                                MenuButton::new(
                                    cx,
                                    |ex| ex.emit(ToolbarEvent::ShowAbout),
                                    |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_QUESTION_MARK);
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
                                    |ex| ex.emit(ToolbarEvent::OpenIssuesPage),
                                    |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_BUG_FILLED);
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
                                    |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_FILE);
                                            Label::new(cx, "Documentation");
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
                                    |cx| {
                                        HStack::new(cx, |cx| {
                                            Svg::new(cx, ICON_LICENSE);
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
                        .color(theme.map(|theme| theme.text_primary))
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
            .background_color(theme.map(|theme| theme.background_dark))
            .border_width(Pixels(1.0))
            .border_color(theme.map(|theme| theme.border))
            .corner_radius(Pixels(4.0))
            .on_press_down(|ex| ex.emit(WindowEvent::DragWindow))
    }
}

pub enum ToolbarEvent {
    // Help
    ShowAbout,
    OpenIssuesPage,
    OpenDocumentationPage,
    ShowLicense,

    // Exit
    ExitApp,
}

pub trait ToolbarModifers {
    fn on_exit<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self;
}

impl<'a> ToolbarModifers for Handle<'a, Toolbar> {
    fn on_exit<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self {
        self.modify(|toolbar| toolbar.on_exit = Some(Box::new(callback)))
    }
}
