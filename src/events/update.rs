use vizia::style::Color;

#[non_exhaustive]
pub enum Update {
    Theme(UpdateTheme),
    UpdateSettings(UpdateUpdateSettings),
}

pub enum UpdateTheme {
    Primary(Color),
    Border(Color),
    TextPrimary(Color),
    TextSecondary(Color),
    TextLight(Color),
    Background(Color),
    BackgroundDark(Color),
    BackgroundLight(Color),

    DarkDefault,
    LightDefault,
}

pub enum UpdateUpdateSettings {
    ToggleAutoUpdates,
}
