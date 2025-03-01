use super::*;

#[derive(Clone, Data, Builder)]
#[builder(on(Color, into))]
pub struct Theme {
    pub background: Color,
    pub background_light: Color,
    pub background_dark: Color,

    pub primary: Color,

    pub border: Color,

    pub text_primary: Color,
}
