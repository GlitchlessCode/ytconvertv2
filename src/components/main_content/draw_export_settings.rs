use super::*;
use playlist_settings::playlist_settings;
use reqwest::blocking::Client;
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
                    Svg::new(cx, ICON_PLAYER_PLAY).fill(AppData::theme.map(|theme| theme.primary));
                    Label::new(cx, "Video")
                        .font_size("large")
                        .with_text_primary(AppData::theme);
                })
                .alignment(Alignment::Center)
            },
            |cx| {
                HStack::new(cx, |cx| {
                    Svg::new(cx, ICON_LIST).fill(AppData::theme.map(|theme| theme.primary));
                    Label::new(cx, "Playlist")
                        .font_size("large")
                        .with_text_primary(AppData::theme);
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
        .on_background_dark(AppData::theme);

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

        let yt_dlp = AppData::yt_dlp_path.get(cx).join(executable);
        let client = Client::new();

        AppVideoData::new(yt_dlp.clone(), client.clone()).build(cx);
        AppPlaylistData::new(yt_dlp, client).build(cx);

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
    .on_background_light(AppData::theme)
    .padding(Pixels(6.0));
}
