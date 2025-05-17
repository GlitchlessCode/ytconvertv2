use super::*;

use crate::modifiers::ThemeModifiers;

pub struct AboutPopup {}

impl AboutPopup {
    pub fn new<T>(cx: &mut Context, theme: T) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
    {
        Self {}
            .build(cx, |cx| {
                Label::new(cx, "ytconvertv2")
                    .color(theme.map(|theme| theme.primary))
                    .font_size("xx-large");
                Label::new(cx, format!("v{}", env!("CARGO_PKG_VERSION")))
                    .with_text_secondary(theme)
                    .font_size("small");

                VStack::new(cx, |cx| {
                    Label::rich(cx, "", |cx| {
                        TextSpan::new(cx, "Made by ", |_| {});
                        TextSpan::new(cx, "GlitchlessCode", |_| {})
                            .text_decoration_line(TextDecorationLine::Underline)
                            .cursor(CursorIcon::Hand)
                            .pointer_events(true)
                            .on_press(|_| {
                                if let Err(_) = open::that("https://github.com/GlitchlessCode/") {
                                    eprintln!("Error opening url");
                                }
                            });
                    })
                    .with_text_primary(theme);

                    Label::new(cx, "A convenient GUI wrapper around yt-dlp")
                        .with_text_primary(theme)
                        .text_wrap(false);
                })
                .padding(Pixels(12.0))
                .alignment(Alignment::Center)
                .height(Auto);

                Label::rich(cx, "", |cx| {
                    TextSpan::new(cx, "Built with ", |_| {});
                    TextSpan::new(cx, "vizia", |_| {})
                        .text_decoration_line(TextDecorationLine::Underline)
                        .cursor(CursorIcon::Hand)
                        .pointer_events(true)
                        .on_press(|_| {
                            if let Err(_) = open::that("https://github.com/vizia/vizia") {
                                eprintln!("Error opening url");
                            }
                        });

                    TextSpan::new(cx, ", ", |_| {});
                    TextSpan::new(cx, "yt-dlp", |_| {})
                        .text_decoration_line(TextDecorationLine::Underline)
                        .cursor(CursorIcon::Hand)
                        .pointer_events(true)
                        .on_press(|_| {
                            if let Err(_) = open::that("https://github.com/yt-dlp/yt-dlp") {
                                eprintln!("Error opening url");
                            }
                        });

                    TextSpan::new(cx, ", and ", |_| {});
                    TextSpan::new(cx, "ffmpeg", |_| {})
                        .text_decoration_line(TextDecorationLine::Underline)
                        .cursor(CursorIcon::Hand)
                        .pointer_events(true)
                        .on_press(|_| {
                            if let Err(_) = open::that("https://ffmpeg.org/") {
                                eprintln!("Error opening url");
                            }
                        });
                })
                .with_text_secondary(theme);
            })
            .gap(Pixels(3.0))
            .size(Stretch(1.0))
            .padding(Pixels(6.0))
            .layout_type(LayoutType::Column)
            .alignment(Alignment::Center)
            .on_background(theme)
    }
}

impl View for AboutPopup {
    fn element(&self) -> Option<&'static str> {
        Some("aboutpopup")
    }
}
