use vizia::icons::{ICON_ALERT_HEXAGON, ICON_HELP_HEXAGON};

use crate::{
    error::{Error, ErrorSeverity},
    modifiers::{ThemeModifiers, ViewModifiers},
};

use super::*;

pub struct ErrorView {}

impl ErrorView {
    pub fn new<T: Lens<Target = Theme>>(cx: &mut Context, theme: T, error: Error) -> Handle<Self> {
        Self {}
            .build(cx, |cx| {
                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        match error.severity {
                            ErrorSeverity::Warning => {
                                Svg::new(cx, ICON_HELP_HEXAGON).class("warning-svg");
                            }
                            ErrorSeverity::Error => {
                                Svg::new(cx, ICON_ALERT_HEXAGON).class("error-svg");
                            }
                        }

                        Label::new(cx, format!("E{:0>5}", error.code))
                            .with_text_primary(theme)
                            .width(Auto);

                        Label::new(cx, error.title)
                            .width(Stretch(1.0))
                            .text_overflow(TextOverflow::Ellipsis)
                            .text_wrap(false)
                            .with_text_primary(theme);
                    })
                    .alignment(Alignment::Left)
                    .height(Auto)
                    .gap(Pixels(4.0));

                    ScrollView::new(cx, move |cx| {
                        if let Some(description) = error.description {
                            Label::new(cx, description)
                                .text_wrap(true)
                                .font_size("small")
                                .with_text_secondary(theme)
                                .width(Stretch(1.0));
                        }

                        if let Some(footer) = error.footer {
                            (footer)(cx);
                        }
                    })
                    .height(Stretch(1.0));
                })
                .padding(Pixels(6.0))
                .gap(Pixels(3.0))
                .round_box(theme)
                .on_background_light(theme);
            })
            .padding_bottom(Pixels(6.0))
            .layout_type(LayoutType::Column)
            .width(Stretch(1.0))
            .height(Pixels(150.0))
    }
}

impl View for ErrorView {
    fn element(&self) -> Option<&'static str> {
        Some("error")
    }
}
