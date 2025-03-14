use crate::modifiers::ViewModifiers;

use super::*;

pub struct ToggleButtonPanel {
    on_choose: Option<Box<dyn Fn(&mut EventContext, ToggleButtonChoice)>>,
}

impl ToggleButtonPanel {
    pub fn new<T: Lens<Target = Theme>, B: Lens<Target = bool>, U: View, V: View>(
        cx: &mut Context,
        theme: T,
        toggled_right: B,
        first: impl Fn(&mut Context) -> Handle<U> + 'static,
        second: impl Fn(&mut Context) -> Handle<V> + 'static,
    ) -> Handle<Self> {
        Self { on_choose: None }
            .build(cx, |cx| {
                ZStack::new(cx, |cx| {
                    Element::new(cx)
                        .round_box(theme)
                        .background_color(theme.map(|theme| theme.background))
                        .width(Percentage(50.0))
                        .class("bg")
                        .toggle_class("right", toggled_right);
                    HStack::new(cx, |cx| {
                        Button::new(cx, |cx| (first)(cx))
                            .width(Percentage(50.0))
                            .on_press(|ex| {
                                ex.emit(ToggleButtonEvent::Press(ToggleButtonChoice::Left))
                            })
                            .border_width(Pixels(0.0))
                            .background_color("transparent");
                        Button::new(cx, |cx| (second)(cx))
                            .width(Percentage(50.0))
                            .on_press(|ex| {
                                ex.emit(ToggleButtonEvent::Press(ToggleButtonChoice::Right))
                            })
                            .border_width(Pixels(0.0))
                            .background_color("transparent");
                    })
                    .alignment(Alignment::Center);
                });
            })
            .width(Stretch(1.0))
            .height(Pixels(32.0))
    }
}

impl View for ToggleButtonPanel {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|event, _meta| match event {
            ToggleButtonEvent::Press(choice) => {
                if let Some(callback) = &self.on_choose {
                    (callback)(cx, *choice)
                }
            }
            #[allow(unreachable_patterns)]
            _ => (),
        })
    }

    fn element(&self) -> Option<&'static str> {
        Some("togglebuttonpanel")
    }
}

pub trait ToggleButtonPanelModifers {
    fn on_choose<F: Fn(&mut EventContext, ToggleButtonChoice) + 'static>(self, callback: F)
        -> Self;
}

impl ToggleButtonPanelModifers for Handle<'_, ToggleButtonPanel> {
    fn on_choose<F: Fn(&mut EventContext, ToggleButtonChoice) + 'static>(
        self,
        callback: F,
    ) -> Self {
        self.modify(|panel| panel.on_choose = Some(Box::new(callback)))
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ToggleButtonEvent {
    Press(ToggleButtonChoice),
}

#[derive(Debug, Clone, Copy)]
pub enum ToggleButtonChoice {
    Left,
    Right,
}
