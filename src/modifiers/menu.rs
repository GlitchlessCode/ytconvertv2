use crate::theme::Theme;

use super::*;

pub trait MenuStyleModifier {
    fn default_menu_style<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>;

    fn round_top(self, radius: Units) -> Self;

    fn round_bottom(self, radius: Units) -> Self;
}

impl MenuStyleModifier for Handle<'_, MenuButton> {
    fn default_menu_style<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.background_color(theme.map(|theme| theme.background))
            .corner_radius(Pixels(0.0))
    }

    fn round_top(self, radius: Units) -> Self {
        self.corner_top_left_radius(radius)
            .corner_top_right_radius(radius)
    }

    fn round_bottom(self, radius: Units) -> Self {
        self.corner_bottom_left_radius(radius)
            .corner_bottom_right_radius(radius)
    }
}

impl MenuStyleModifier for Handle<'_, Submenu> {
    fn default_menu_style<T>(self, theme: T) -> Self
    where
        T: Lens<Target = Theme>,
    {
        self.background_color(theme.map(|theme| theme.background))
            .corner_radius(Pixels(0.0))
    }

    fn round_top(self, radius: Units) -> Self {
        self.corner_top_left_radius(radius)
            .corner_top_right_radius(radius)
    }

    fn round_bottom(self, radius: Units) -> Self {
        self.corner_bottom_left_radius(radius)
            .corner_bottom_right_radius(radius)
    }
}
