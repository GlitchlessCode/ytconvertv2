use crate::theme::Theme;

use super::*;

pub trait ViewModifiers {
    fn round_box<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;
}

impl<V> ViewModifiers for Handle<'_, V>
where
    V: View,
{
    fn round_box<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.corner_radius(Pixels(4.0))
            .border_color(theme.map(|theme| theme.border))
            .border_width(Pixels(1.0))
    }
}
