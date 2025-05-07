use super::*;
use playlist_settings::playlist_settings;
use video_settings::video_settings;
use vizia::icons::{ICON_LIST, ICON_PLAYER_PLAY};

mod playlist_settings;
mod video_settings;

pub fn draw_export_settings(cx: &mut Context) {
    VStack::new(cx, |cx| {
        ToggleButtonPanel::new(
            cx,
            AppData::theme,
            AppData::playlist_selected,
            |cx| {
                HStack::new(cx, |cx| {
                    Svg::new(cx, ICON_PLAYER_PLAY);
                    Label::new(cx, "Video")
                        .font_size("large")
                        .color(AppData::theme.map(|theme| theme.text_primary));
                })
                .alignment(Alignment::Center)
            },
            |cx| {
                HStack::new(cx, |cx| {
                    Svg::new(cx, ICON_LIST);
                    Label::new(cx, "Playlist")
                        .font_size("large")
                        .color(AppData::theme.map(|theme| theme.text_primary));
                })
                .alignment(Alignment::Center)
            },
        )
        .on_choose(|ex, side| match side {
            ToggleButtonChoice::Left => ex.emit(AppEvent::ToggleVideo),
            ToggleButtonChoice::Right => ex.emit(AppEvent::TogglePlaylist),
        })
        .round_box(AppData::theme)
        .padding(Pixels(3.0))
        .background_color(AppData::theme.map(|theme| theme.background_dark));

        let anim_out = cx.add_animation(
            AnimationBuilder::new()
                .keyframe(0.0, |kf| kf.opacity(1.0).scale((1.0, 1.0)))
                .keyframe(1.0, |kf| kf.opacity(0.0).scale((0.983, 0.983))),
        );
        let anim_in = cx.add_animation(
            AnimationBuilder::new()
                .keyframe(0.0, |kf| kf.opacity(0.0).scale((0.983, 0.983)))
                .keyframe(1.0, |kf| kf.opacity(1.0).scale((1.0, 1.0))),
        );

        let executable = if cfg!(windows) {
            "yt-dlp.exe"
        } else {
            "yt-dlp"
        };

        AppVideoData::new(AppData::yt_dlp_path.get(cx).join(executable)).build(cx);

        AnimatedBinding::new(cx, AppData::playlist_selected, |cx, pl_selected| {
            VStack::new(cx, move |cx| {
                if pl_selected.get(cx) {
                    playlist_settings(cx);
                } else {
                    video_settings(cx);
                }
            })
            .gap(Pixels(6.0))
            .padding_top(Pixels(10.0));
        })
        .set_anim_out(AnimationDef::new(
            anim_out,
            Duration::from_millis(150),
            Duration::ZERO,
        ))
        .set_anim_in(AnimationDef::new(
            anim_in,
            Duration::from_millis(150),
            Duration::ZERO,
        ));
    })
    .round_box(AppData::theme)
    .background_color(AppData::theme.map(|theme| theme.background_light))
    .padding(Pixels(6.0));
}
