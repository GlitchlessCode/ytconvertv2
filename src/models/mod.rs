use crate::{
    error::{Error, ErrorSeverity},
    include_bytes_safe,
    theme::Theme,
    views::all::*,
};
use vizia::prelude::*;

pub mod animation;
pub mod app;
pub mod playlist;
pub mod video;

pub static LARGE_PLACEHOLDER: &[u8] = include_bytes_safe!("..", "img", "large_placeholder.png");

pub mod all {
    pub use super::app::AppData;

    pub use super::animation::AppAnimationData;

    pub use super::video::AppVideoData;

    pub use super::playlist::AppPlaylistData;
}
