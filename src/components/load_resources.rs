use super::*;
use ytconvertv2::{include_bytes_safe, models::LARGE_PLACEHOLDER};

pub fn load_resources(cx: &mut Context) {
    // Add font
    let bytes = include_bytes_safe!("..", "font", "Quicksand-Medium.ttf");
    cx.add_font_mem(bytes);

    // Add stylesheet
    cx.add_stylesheet(include_style!("src/style/style.css"))
        .expect("Failed to load stylesheet");

    // Add default thumbnails
    cx.load_image(
        "video_thumb",
        LARGE_PLACEHOLDER,
        ImageRetentionPolicy::Forever,
    );

    cx.load_image(
        "playlist_thumb",
        LARGE_PLACEHOLDER,
        ImageRetentionPolicy::Forever,
    );
}
