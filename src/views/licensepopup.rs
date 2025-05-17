use vizia::icons::ICON_SCALE;

use super::*;

use crate::{
    data::license::License,
    modifiers::{ThemeModifiers, ViewModifiers},
};

pub struct LicensePopup {}

impl LicensePopup {
    pub fn new<T, L>(cx: &mut Context, theme: T, licenses: L) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
        L: Lens<Target = Option<Vec<License>>>,
    {
        Self {}
            .build(cx, move |cx| {
                Label::new(cx, "Licenses")
                    .with_text_primary(theme)
                    .font_weight("bold")
                    .font_size("large");
                Label::new(
                    cx,
                    "Here is a list of all dependencies that ytconvertv2 \
                    relies on, and their licenses. The license text for each \
                    license could not be included here, but can be found online.",
                )
                .with_text_secondary(theme)
                .font_size("small")
                .width(Stretch(1.0))
                .text_wrap(true);

                Binding::new(cx, licenses, move |cx, licenses| {
                    if licenses.get(cx).is_some() {
                        VirtualList::new(
                            cx,
                            licenses.map(|l| l.clone().unwrap_or_default()),
                            120.0,
                            move |cx, _idx, license| LicenseView::new(cx, theme, license.get(cx)),
                        )
                        .round_box(theme)
                        .padding(Pixels(6.0))
                        .on_background_dark(theme)
                        .height(Stretch(1.0));
                    } else {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Failed to Load Licenses")
                                .with_text_light(theme)
                                .font_size("small");
                        })
                        .size(Stretch(1.0))
                        .alignment(Alignment::Center);
                    }
                });
            })
            .gap(Pixels(3.0))
            .size(Stretch(1.0))
            .padding(Pixels(6.0))
            .layout_type(LayoutType::Column)
            .on_background(theme)
    }
}

impl View for LicensePopup {
    fn element(&self) -> Option<&'static str> {
        Some("licensepopup")
    }
}

struct LicenseView {}

impl LicenseView {
    fn new<T>(cx: &mut Context, theme: T, license: License) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
    {
        Self {}
            .build(cx, |cx| {
                let License {
                    name,
                    version,
                    repository,
                    license,
                    description,
                    ..
                } = license;

                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, name).with_text_primary(theme).width(Auto);

                            if let Some(version) = version {
                                Label::new(cx, format!("v{version}"))
                                    .with_text_secondary(theme)
                                    .font_size("small")
                                    .width(Auto);
                            }

                            Spacer::new(cx);
                        })
                        .gap(Pixels(6.0))
                        .alignment(Alignment::Center);

                        Spacer::new(cx);

                        Svg::new(cx, ICON_SCALE).fill(theme.map(|theme| theme.text_secondary));

                        Label::new(cx, license.unwrap_or("No License".to_string()))
                            .with_text_secondary(theme);
                    })
                    .height(Auto)
                    .gap(Pixels(6.0))
                    .alignment(Alignment::Center);

                    if let Some(repository) = repository {
                        HStack::new(cx, |cx| {
                            Label::new(cx, repository.clone())
                                .with_text_secondary(theme)
                                .font_size("small")
                                .cursor(CursorIcon::Hand)
                                .on_press(move |_| {
                                    if let Err(_) = open::that(repository.clone()) {
                                        eprintln!("Error opening url");
                                    }
                                })
                                .width(Auto);

                            Spacer::new(cx);
                        })
                        .height(Auto);
                    }

                    if let Some(description) = description {
                        Markdown::new(cx, &description)
                            .with_text_secondary(theme)
                            .font_size("small")
                            .size(Stretch(1.0))
                            .text_overflow(TextOverflow::Ellipsis)
                            .text_wrap(true)
                            .padding(Pixels(0.0))
                            .alignment(Alignment::Left);
                    } else {
                        VStack::new(cx, |cx| {
                            Label::new(cx, "No Description Provided")
                                .with_text_light(theme)
                                .font_size("small");
                        })
                        .size(Stretch(1.0))
                        .alignment(Alignment::Center);
                    }
                })
                .gap(Pixels(6.0))
                .padding(Pixels(6.0))
                .on_background(theme)
                .round_box(theme);
            })
            .padding_bottom(Pixels(6.0))
            .height(Pixels(120.0))
    }
}

impl View for LicenseView {
    fn element(&self) -> Option<&'static str> {
        Some("license")
    }
}
