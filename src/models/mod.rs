use crate::theme::Theme;
use vizia::prelude::*;

pub mod animation;
pub mod app;
pub mod playlist;
pub mod video;

pub mod all {
    pub use super::app::AppData;
    pub use super::app::AppEvent;

    pub use super::animation::AppAnimationData;

    pub use super::video::AppVideoData;
    pub use super::video::AppVideoEvent;
}
