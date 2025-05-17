use vizia::icons::ICON_CHEVRON_RIGHT;

use crate::{data::ActiveTask, modifiers::ThemeModifiers};

use super::{all::*, *};

pub struct ActiveTaskView {}

impl ActiveTaskView {
    pub fn new<A, T>(cx: &mut Context, theme: T, active_task: A) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
        A: Lens<Target = Option<ActiveTask>>,
    {
        Self {}
            .build(cx, |cx| {
                Binding::new(cx, active_task, move |cx, active_task| {
                    VStack::new(cx, |cx| match active_task.get(cx) {
                        Some(ActiveTask::Video { settings, .. }) => {
                            VStack::new(cx, |cx| {
                                Label::new(cx, settings.title)
                                    .color(theme.map(|theme| theme.primary))
                                    .font_size("small")
                                    .text_overflow(TextOverflow::Ellipsis)
                                    .text_wrap(false)
                                    .width(Stretch(1.0));
                                video_components(cx, theme, active_task);
                            })
                            .height(Auto)
                            .gap(Pixels(3.0))
                            .alignment(Alignment::Center);
                        }
                        Some(ActiveTask::Playlist { settings, .. }) => {
                            VStack::new(cx, |cx| {
                                Label::new(cx, settings.title)
                                    .color(theme.map(|theme| theme.primary))
                                    .font_size("small")
                                    .text_overflow(TextOverflow::Ellipsis)
                                    .text_wrap(false)
                                    .width(Stretch(1.0));

                                HStack::new(cx, |cx| {
                                    Label::new(
                                        cx,
                                        active_task.map(|task| {
                                            if let Some(data) = task
                                                .as_ref()
                                                .and_then(|task| task.get_active_video())
                                            {
                                                format!("{}", data.title)
                                            } else {
                                                "No Video Processing...".to_string()
                                            }
                                        }),
                                    )
                                    .with_text_primary(theme)
                                    .font_size("small")
                                    .text_overflow(TextOverflow::Ellipsis)
                                    .text_wrap(false);

                                    Spacer::new(cx);

                                    Label::new(
                                        cx,
                                        active_task.map(|task| {
                                            if let Some((finished, total)) = task
                                                .as_ref()
                                                .and_then(|task| task.get_count_ratio())
                                            {
                                                format!(
                                                    "{}/{total}",
                                                    std::cmp::min(finished, total)
                                                )
                                            } else {
                                                "No Video Processing...".to_string()
                                            }
                                        }),
                                    )
                                    .with_text_secondary(theme)
                                    .font_size("small")
                                    .text_overflow(TextOverflow::Ellipsis)
                                    .text_wrap(false);
                                })
                                .height(Auto);

                                video_components(cx, theme, active_task);
                            })
                            .height(Auto)
                            .gap(Pixels(3.0))
                            .alignment(Alignment::Center);
                        }
                        None => {
                            Label::new(cx, "No Active Task...")
                                .with_text_light(theme)
                                .font_size("small");
                        }
                    })
                    .padding(Pixels(6.0))
                    .alignment(Alignment::Center);
                });
            })
            .size(Stretch(1.0))
    }
}

impl View for ActiveTaskView {
    fn element(&self) -> Option<&'static str> {
        Some("activetask")
    }
}

fn video_components<T, A>(cx: &mut Context, theme: T, active_task: A)
where
    T: Lens<Target = Theme>,
    A: Lens<Target = Option<ActiveTask>>,
{
    Binding::new(cx, active_task, move |cx, task| {
        let downloading = task.get(cx).is_some_and(|task| task.get_downloading());
        HStack::new(cx, |cx| {
            Svg::new(cx, ICON_CHEVRON_RIGHT).fill(theme.map(move |theme| {
                if downloading {
                    theme.primary
                } else {
                    theme.background_dark
                }
            }));

            Label::new(cx, "Downloading...")
                .font_size("small")
                .color(theme.map(move |theme| {
                    if downloading {
                        theme.text_primary
                    } else {
                        theme.text_secondary
                    }
                }));
        })
        .height(Auto)
        .alignment(Alignment::Left)
        .gap(Pixels(3.0));
    });

    Binding::new(cx, active_task, move |cx, task| {
        let not_downloading = task.get(cx).is_some_and(|task| !task.get_downloading());
        HStack::new(cx, |cx| {
            Svg::new(cx, ICON_CHEVRON_RIGHT).fill(theme.map(move |theme| {
                if not_downloading {
                    theme.primary
                } else {
                    theme.background_dark
                }
            }));
            VStack::new(cx, |cx| {
                Label::new(cx, "Converting...")
                    .font_size("small")
                    .color(theme.map(move |theme| {
                        if not_downloading {
                            theme.text_primary
                        } else {
                            theme.text_secondary
                        }
                    }));
                CustomProgressBar::new(
                    cx,
                    theme,
                    active_task.map(|active| {
                        active
                            .as_ref()
                            .map(|active| active.get_video_progress())
                            .unwrap_or(0.0)
                    }),
                );
            });
        })
        .height(Auto)
        .alignment(Alignment::Left)
        .gap(Pixels(3.0));
    });
}
