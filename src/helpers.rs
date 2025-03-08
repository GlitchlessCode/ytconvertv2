use vizia::prelude::*;

use crate::theme::Theme;

pub fn labelled<T, F, Th>(
    cx: &mut Context,
    theme: Th,
    name: impl Res<T> + Clone,
    content: F,
) -> Handle<VStack>
where
    Th: Lens<Target = Theme>,
    T: ToStringLocalized,
    F: FnOnce(&mut Context),
{
    VStack::new(cx, |cx| {
        Label::new(cx, name)
            .color(theme.map(|theme| theme.text_secondary))
            .font_size("small");

        (content)(cx);
    })
    .gap(Pixels(1.5))
}
