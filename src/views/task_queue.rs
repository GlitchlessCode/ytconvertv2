use crate::{data::TaskQueue, modifiers::ViewModifiers};

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
                        move |cx, _task_queue, _task| {
                            TaskView::new(cx, theme);
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
    pub fn new<T>(cx: &mut Context, theme: T) -> Handle<Self>
    where
        T: Lens<Target = Theme>,
    {
        Self {}
            .build(cx, |cx| {
                Element::new(cx).round_box(theme);
            })
            .padding(Pixels(6.0))
            .padding_top(Pixels(0.0))
    }
}

impl View for TaskView {
    fn element(&self) -> Option<&'static str> {
        Some("task")
    }
}
