use super::*;
use nanoserde::{DeBin, SerBin};

#[derive(Debug, Clone, Data, SerBin, DeBin)]
#[nserde(proxy = "ThemeProxy")]
pub struct Theme {
    pub background: Color,
    pub background_light: Color,
    pub background_dark: Color,

    pub primary: Color,

    pub border: Color,

    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_light: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: "#252525".into(),
            background_dark: "#151515".into(),
            background_light: "#2f2f2f".into(),
            border: "#444444".into(),
            primary: "gold".into(),
            text_primary: "#eeeeee".into(),
            text_secondary: "#aaaaaa".into(),
            text_light: "#575757".into(),
        }
    }
    pub fn light() -> Self {
        Self {
            background: "#ececf0".into(),
            background_dark: "#d9d9eb".into(),
            background_light: "#fbfbfb".into(),
            border: "#a0a0a0".into(),
            primary: "gold".into(),
            text_primary: "#1a1a1a".into(),
            text_secondary: "#696969".into(),
            text_light: "#a9a9a9".into(),
        }
    }
}

#[derive(Debug, SerBin, DeBin)]
struct ThemeProxy {
    pub background: (u8, u8, u8),
    pub background_light: (u8, u8, u8),
    pub background_dark: (u8, u8, u8),

    pub primary: (u8, u8, u8),

    pub border: (u8, u8, u8),

    pub text_primary: (u8, u8, u8),
    pub text_secondary: (u8, u8, u8),
    pub text_light: (u8, u8, u8),
}

impl From<&Theme> for ThemeProxy {
    fn from(value: &Theme) -> Self {
        Self {
            background: color_to_tuple(value.background),
            background_light: color_to_tuple(value.background_light),
            background_dark: color_to_tuple(value.background_dark),
            primary: color_to_tuple(value.primary),
            border: color_to_tuple(value.border),
            text_primary: color_to_tuple(value.text_primary),
            text_secondary: color_to_tuple(value.text_secondary),
            text_light: color_to_tuple(value.text_light),
        }
    }
}

impl From<&ThemeProxy> for Theme {
    fn from(value: &ThemeProxy) -> Self {
        Self {
            background: tuple_to_color(value.background),
            background_light: tuple_to_color(value.background_light),
            background_dark: tuple_to_color(value.background_dark),
            primary: tuple_to_color(value.primary),
            border: tuple_to_color(value.border),
            text_primary: tuple_to_color(value.text_primary),
            text_secondary: tuple_to_color(value.text_secondary),
            text_light: tuple_to_color(value.text_light),
        }
    }
}

fn color_to_tuple(color: Color) -> (u8, u8, u8) {
    (color.r(), color.g(), color.b())
}

fn tuple_to_color((r, g, b): (u8, u8, u8)) -> Color {
    Color::rgb(r, g, b)
}
