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

pub trait ThemeModifiers {
    fn on_background<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;

    fn on_background_light<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;

    fn on_background_dark<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;

    fn with_border<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;

    fn with_text_primary<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;

    fn with_text_secondary<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;

    fn with_text_light<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;
}

impl<V> ThemeModifiers for Handle<'_, V>
where
    V: View,
{
    #[inline(always)]
    fn on_background<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.background_color(theme.map(|theme| theme.background))
    }

    #[inline(always)]
    fn on_background_dark<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.background_color(theme.map(|theme| theme.background_dark))
    }

    #[inline(always)]
    fn on_background_light<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.background_color(theme.map(|theme| theme.background_light))
    }

    #[inline(always)]
    fn with_border<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.border_color(theme.map(|theme| theme.border))
            .border_width(Pixels(1.0))
    }

    #[inline(always)]
    fn with_text_primary<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.color(theme.map(|theme| theme.text_primary))
    }

    #[inline(always)]
    fn with_text_secondary<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.color(theme.map(|theme| theme.text_secondary))
    }

    #[inline(always)]
    fn with_text_light<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.color(theme.map(|theme| theme.text_light))
    }
}
