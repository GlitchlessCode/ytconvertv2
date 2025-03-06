use super::*;
use winit::window::ResizeDirection;

pub struct Resizer;

impl View for Resizer {
    fn element(&self) -> Option<&'static str> {
        Some("resizer")
    }
}

impl Resizer {
    pub fn new(cx: &mut Context, direction: ResizerDirection) -> Handle<Self> {
        Self.build(cx, |_| {})
            .cursor(CursorIcon::from(direction))
            .size(Stretch(1.0))
            .pointer_events(true)
            .on_mouse_down(move |ex, mb| {
                if mb == MouseButton::Left {
                    ex.modify_window(|window| window.drag_resize_window(direction.into()));
                }
            })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResizerDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl From<ResizerDirection> for CursorIcon {
    fn from(value: ResizerDirection) -> Self {
        match value {
            ResizerDirection::Up => CursorIcon::NsResize,
            ResizerDirection::UpRight => CursorIcon::NeswResize,
            ResizerDirection::Right => CursorIcon::EwResize,
            ResizerDirection::DownRight => CursorIcon::NwseResize,
            ResizerDirection::Down => CursorIcon::NsResize,
            ResizerDirection::DownLeft => CursorIcon::NeswResize,
            ResizerDirection::Left => CursorIcon::EwResize,
            ResizerDirection::UpLeft => CursorIcon::NwseResize,
        }
    }
}

impl From<ResizerDirection> for ResizeDirection {
    fn from(value: ResizerDirection) -> Self {
        match value {
            ResizerDirection::Up => ResizeDirection::North,
            ResizerDirection::UpRight => ResizeDirection::NorthEast,
            ResizerDirection::Right => ResizeDirection::East,
            ResizerDirection::DownRight => ResizeDirection::SouthEast,
            ResizerDirection::Down => ResizeDirection::South,
            ResizerDirection::DownLeft => ResizeDirection::SouthWest,
            ResizerDirection::Left => ResizeDirection::West,
            ResizerDirection::UpLeft => ResizeDirection::NorthWest,
        }
    }
}

pub struct ResizerGroup;

impl ResizerGroup {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Resizer::new(cx, ResizerDirection::UpLeft).width(Pixels(8.0));
                Resizer::new(cx, ResizerDirection::Up).width(Stretch(1.0));
                Resizer::new(cx, ResizerDirection::UpRight).width(Pixels(8.0));
            })
            .height(Pixels(8.0));

            HStack::new(cx, |cx| {
                Resizer::new(cx, ResizerDirection::Left).width(Pixels(8.0));

                Element::new(cx).width(Stretch(1.0));

                Resizer::new(cx, ResizerDirection::Right).width(Pixels(8.0));
            })
            .height(Stretch(1.0));

            HStack::new(cx, |cx| {
                Resizer::new(cx, ResizerDirection::DownLeft).width(Pixels(8.0));
                Resizer::new(cx, ResizerDirection::Down).width(Stretch(1.0));
                Resizer::new(cx, ResizerDirection::DownRight).width(Pixels(8.0));
            })
            .height(Pixels(8.0));
        })
        .pointer_events(false)
        .layout_type(LayoutType::Column)
    }
}

impl View for ResizerGroup {
    fn element(&self) -> Option<&'static str> {
        Some("resizergroup")
    }
}
