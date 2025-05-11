use std::path::PathBuf;

use vizia::icons::ICON_TRASH;

use crate::{
    data::{
        task::{PlaylistData, TaskData, VideoData},
        ActiveTask, Task, TaskQueue, VideoExportSettings,
    },
    modifiers::ViewModifiers,
};

use super::*;

pub struct TaskQueueView {}

impl TaskQueueView {
    pub fn new<T, Q>(cx: &mut Context, theme: T, task_queue: Q) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
        Q: Lens<Target = TaskQueue>,
    {
        Self {}.build(cx, |cx| {
            Binding::new(cx, task_queue.then(TaskQueue::tasks), move |cx, tasks| {
                if tasks.get(cx).is_empty() {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Task Queue Is Empty...")
                            .color(theme.map(|theme| theme.text_light))
                            .font_size("small");
                    })
                    .alignment(Alignment::Center);
                } else {
                    List::new(
                        cx,
                        task_queue.then(TaskQueue::tasks),
                        move |cx, idx, task| {
                            TaskView::new(cx, theme, task, idx);
                        },
                    )
                    .padding_top(Pixels(6.0));
                }
            });
        })
    }
}

impl View for TaskQueueView {
    fn element(&self) -> Option<&'static str> {
        Some("taskqueue")
    }
}

pub struct TaskView {}

impl TaskView {
    pub fn new<T, L>(cx: &mut Context, theme: T, task: L, index: usize) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
        L: Lens<Target = Task>,
    {
        Self {}
            .build(cx, |cx| {
                let task = task.get(cx);
                match task.data() {
                    TaskData::Video(data, settings) => {
                        show_video_details(cx, theme, data, settings, task.location(), index)
                    }
                    TaskData::Playlist(data) => {
                        show_playlist_details(cx, theme, data, task.location(), index)
                    }
                }
            })
            .padding(Pixels(3.0))
            .padding_top(Pixels(0.0))
            .height(Pixels(75.0))
    }
}

fn show_video_details<T: Lens<Target = Theme>>(
    cx: &mut Context,
    theme: T,
    data: &VideoData,
    settings: &VideoExportSettings,
    location: &PathBuf,
    index: usize,
) {
    HStack::new(cx, |cx| {
        Label::new(cx, settings.export_type.to_string())
            .color(theme.map(|theme| theme.text_primary))
            .font_size("x-small")
            .padding(Pixels(10.0));

        VStack::new(cx, |cx| {
            Label::new(cx, &settings.title)
                .color(theme.map(|theme| theme.primary))
                .font_size("small")
                .width(Stretch(1.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis);
            Label::new(cx, data.title())
                .color(theme.map(|theme| theme.text_primary))
                .font_size("x-small")
                .width(Stretch(1.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis);
            Label::new(cx, data.author())
                .color(theme.map(|theme| theme.text_secondary))
                .font_size("x-small")
                .width(Stretch(1.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis);

            Spacer::new(cx);

            Label::new(cx, location.display().to_string())
                .color(theme.map(|theme| theme.text_light))
                .font_size("x-small")
                .width(Stretch(1.0))
                .text_wrap(false)
                .text_overflow(TextOverflow::Ellipsis);
        });

        Button::new(cx, |cx| Svg::new(cx, ICON_TRASH).fill("#ff0000"))
            .round_box(theme)
            .background_color("##ff64644d")
            .on_press(move |ex| ex.emit(TaskEvent::Remove(index)));
    })
    .gap(Pixels(6.0))
    .alignment(Alignment::Center)
    .padding(Pixels(3.0))
    .overflow(Overflow::Hidden)
    .background_color(theme.map(|theme| theme.background_dark))
    .round_box(theme);
}

fn show_playlist_details<T: Lens<Target = Theme>>(
    cx: &mut Context,
    theme: T,
    data: &PlaylistData,
    location: &PathBuf,
    index: usize,
) {
}

impl View for TaskView {
    fn element(&self) -> Option<&'static str> {
        Some("task")
    }
}

pub enum TaskEvent {
    Remove(usize),
    SetActive(Task),
    UpdateActive(ActiveTask),
    FinishActive,
    MoveToNext,
}
