use vizia::icons::ICON_X;

use super::*;

pub struct Toolbar {
    on_exit: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl View for Toolbar {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _meta| match event {
            ToolbarEvent::Exit => {
                if let Some(callback) = &self.on_exit {
                    (callback)(cx);
                }
            }
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
                    // TODO: Add logo svg icon here
                    Button::new(cx, |cx| {
                        Label::new(cx, "Button").color(theme.map(|theme| theme.text_primary))
                    })
                    .corner_radius(Pixels(2.0))
                    .pointer_events(true);
                    Button::new(cx, |cx| {
                        Label::new(cx, "Button").color(theme.map(|theme| theme.text_primary))
                    })
                    .corner_radius(Pixels(2.0))
                    .pointer_events(true);

                    Element::new(cx).width(Stretch(1.0));

                    Button::new(cx, |cx| Svg::new(cx, ICON_X))
                        .class("exit")
                        .pointer_events(true)
                        .on_press(|ex| ex.emit(ToolbarEvent::Exit));
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
    Exit,
}

pub trait ToolbarModifers {
    fn on_exit<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self;
}

impl<'a> ToolbarModifers for Handle<'a, Toolbar> {
    fn on_exit<F: Fn(&mut EventContext) + 'static>(self, callback: F) -> Self {
        self.modify(|toolbar| toolbar.on_exit = Some(Box::new(callback)))
    }
}
