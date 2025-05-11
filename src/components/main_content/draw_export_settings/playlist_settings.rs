use super::*;

use vizia::icons::ICON_RELOAD;
use ytconvertv2::events::AppPlaylistEvent;

pub fn playlist_settings(cx: &mut Context) {
    labelled(cx, AppData::theme, "Youtube Playlist Link", |cx| {
        HStack::new(cx, |cx| {
            Textbox::new(cx, AppPlaylistData::link)
                .disabled(AppPlaylistData::searching)
                .text_overflow(TextOverflow::Ellipsis)
                .round_box(AppData::theme)
                .background_color(AppData::theme.map(|theme| theme.background_dark))
                .color(AppData::theme.map(|theme| theme.text_primary))
                .width(Stretch(1.0))
                .on_submit(move |ex, text, _| {
                    ex.emit(AppPlaylistEvent::LinkSubmit(text));
                    ex.focus_next();
                });

            Button::new(cx, |cx| Svg::new(cx, ICON_RELOAD))
                .disabled(AppPlaylistData::searching)
                .class("std-btn")
                .on_press(|ex| ex.emit(AppPlaylistEvent::Reset));
        })
        .height(Auto)
        .gap(Pixels(6.0))
        .alignment(Alignment::Center);
    })
    .height(Auto);

    labelled(cx, AppData::theme, "Preview", |cx| {
        HStack::new(cx, |cx| {
            ZStack::new(cx, |cx| {
                let fade_in = AppAnimationData::fade_in.get(cx);

                AnimatedBinding::new(cx, AppPlaylistData::thumbnail_generation, |cx, _| {
                    Image::new(cx, "playlist_thumb")
                        .class("thumbnail-img")
                        .corner_radius(Pixels(8.0))
                        .border_width(Pixels(1.0))
                        .border_color(AppData::theme.map(|theme| theme.border));
                })
                .set_anim_in(AnimationDef::new(
                    fade_in,
                    Duration::from_millis(400),
                    Duration::ZERO,
                ));

                VStack::new(cx, |cx| {
                    Label::new(
                        cx,
                        // AppPlaylistData::video.map(|video| {
                        //     if let Some(video) = video {
                        //         format_seconds(video.duration())
                        //     } else {
                        //         "??:??".to_string()
                        //     }
                        // }),
                        "????", // Video count "<Playlist Icon> N Videos"
                    )
                    .corner_radius(Pixels(2.0))
                    .font_size("small")
                    .padding(Pixels(2.0))
                    .color("white")
                    .background_color("black");
                })
                .width(Pixels(256.0))
                .height(Pixels(144.0))
                .padding(Pixels(8.0))
                .alignment(Alignment::BottomRight);
            })
            .width(Pixels(256.0))
            .height(Pixels(144.0));

            VStack::new(cx, |cx| {
                Label::new(
                    cx,
                    // AppPlaylistData::video.map(|video| {
                    //     if let Some(video) = video {
                    //         video.title().clone()
                    //     } else {
                    //         "No Video Selected".to_string()
                    //     }
                    // }),
                    "No Playlist Selected",
                )
                .width(Stretch(1.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis)
                .color(AppData::theme.map(|theme| theme.text_primary));
                Label::new(
                    cx,
                    // AppPlaylistData::video.map(|video| {
                    //     if let Some(video) = video {
                    //         video.author().clone()
                    //     } else {
                    //         "".to_string()
                    //     }
                    // }),
                    "",
                )
                .font_size("small")
                .color(AppData::theme.map(|theme| theme.text_primary));
            });
        })
        .overflowx(Overflow::Hidden)
        .gap(Pixels(6.0));
    })
    .height(Pixels(164.0))
    .top(Pixels(6.0));
}

// TODO - Add
