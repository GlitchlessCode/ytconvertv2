use vizia::icons::ICON_X;

use super::*;

pub struct Toolbar {}

impl View for Toolbar {}

impl Toolbar {
    pub fn new<T>(cx: &mut Context, theme: T) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
    {
        Self {}
            .build(cx, |cx| {
                HStack::new(cx, |cx| {
                    Button::new(cx, |cx| {
                        Label::new(cx, "Button").color(theme.map(|theme| theme.text_primary))
                    })
                    .pointer_events(true);
                    Button::new(cx, |cx| {
                        Label::new(cx, "Button").color(theme.map(|theme| theme.text_primary))
                    })
                    .pointer_events(true);

                    Element::new(cx).width(Stretch(1.0));

                    Button::new(cx, |cx| Svg::new(cx, ICON_X))
                        .class("exit")
                        .pointer_events(true);
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
