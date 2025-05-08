use crate::data::ActiveTask;

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
                            Label::new(cx, settings.title)
                                .color(theme.map(|theme| theme.primary))
                                .font_size("small");
                            // CustomProgressBar::new(
                            //     cx,
                            //     theme,
                            //     active_task.map(|active| {
                            //         active
                            //             .as_ref()
                            //             .map(|active| active.get_video_progress())
                            //             .unwrap_or(0.0)
                            //     }),
                            // );
                        }
                        Some(ActiveTask::Playlist { .. }) => {}
                        None => {
                            Label::new(cx, "No Active Task...")
                                .color(theme.map(|theme| theme.text_light))
                                .font_size("small");
                        }
                    })
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
