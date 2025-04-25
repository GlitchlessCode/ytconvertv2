use super::*;
use bon::Builder;
use std::collections::VecDeque;
use vizia::icons::{ICON_ALERT_HEXAGON, ICON_HELP_HEXAGON, ICON_HEXAGON_PLUS, ICON_INFO_HEXAGON};

#[derive(Lens)]
pub struct NotificationService {
    #[lens(ignore)]
    display_queue: VecDeque<Notification>,
    displaying: Vec<Notification>,

    anim_state: AnimationState,
    enqueued_departures: usize,

    slide_in: Animation,
    slide_out: Animation,
}

impl NotificationService {
    pub fn new(cx: &mut Context) {
        let slide_in = cx.add_animation(
            AnimationBuilder::new()
                .keyframe(0.0, |kf| {
                    kf.translate(Translate::new(Percentage(-100.0), Percentage(0.0)))
                })
                .keyframe(1.0, |kf| {
                    kf.translate(Translate::new(Percentage(0.0), Percentage(0.0)))
                }),
        );
        let slide_out = cx.add_animation(
            AnimationBuilder::new()
                .keyframe(0.0, |kf| {
                    kf.translate(Translate::new(Percentage(0.0), Percentage(0.0)))
                        .height(Pixels(40.0))
                })
                .keyframe(1.0, |kf| {
                    kf.translate(Translate::new(Percentage(-100.0), Percentage(0.0)))
                        .height(Pixels(0.0))
                }),
        );

        Self {
            display_queue: VecDeque::new(),
            displaying: Vec::new(),

            anim_state: AnimationState::None,
            enqueued_departures: 0,

            slide_in,
            slide_out,
        }
        .build(cx);
    }
}

#[derive(PartialEq, Data, Clone)]
pub enum AnimationState {
    None,
    AnimatingIn,
    AnimatingOut,
}

impl Model for NotificationService {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event: Notification, _meta| {
            if self.anim_state == AnimationState::None && self.displaying.len() < 4 {
                self.anim_state = AnimationState::AnimatingIn;
                self.displaying.push(event);
                cx.schedule_emit(
                    NotificationServiceEvent::Timeout,
                    Instant::now() + Duration::from_secs(5),
                );
            } else {
                self.display_queue.push_back(event);
            }
        });

        event.map(|event, _meta| match event {
            NotificationServiceEvent::FinishAnimating => {
                if self.anim_state == AnimationState::AnimatingOut {
                    self.displaying.remove(0);
                }

                if self.displaying.len() < 4 && !self.display_queue.is_empty() {
                    cx.emit(NotificationServiceEvent::StartAnimatingIn)
                } else if self.enqueued_departures > 0 {
                    cx.emit(NotificationServiceEvent::StartAnimatingOut)
                } else {
                    self.anim_state = AnimationState::None;
                }
            }
            NotificationServiceEvent::StartAnimatingIn => {
                self.anim_state = AnimationState::AnimatingIn;
                self.displaying.push(
                    self.display_queue
                        .pop_front()
                        .expect("Should be able to pop, queue is not empty"),
                );
                cx.schedule_emit(
                    NotificationServiceEvent::Timeout,
                    Instant::now() + Duration::from_secs(5),
                );
            }
            NotificationServiceEvent::StartAnimatingOut => {
                self.enqueued_departures -= 1;
                self.anim_state = AnimationState::AnimatingOut;
                let id = if self.displaying.len() > 1 {
                    "slide-out-notif"
                } else {
                    "slide-in-notif"
                };
                cx.play_animation_for(
                    self.slide_out,
                    id,
                    Duration::from_millis(120),
                    Duration::ZERO,
                );
                cx.schedule_emit(
                    NotificationServiceEvent::FinishAnimating,
                    Instant::now() + Duration::from_millis(120),
                );
            }
            NotificationServiceEvent::Timeout => {
                if self.anim_state == AnimationState::None {
                    if !self.displaying.is_empty() {
                        self.anim_state = AnimationState::AnimatingOut;
                        let id = if self.displaying.len() > 1 {
                            "slide-out-notif"
                        } else {
                            "slide-in-notif"
                        };
                        cx.play_animation_for(
                            self.slide_out,
                            id,
                            Duration::from_millis(240),
                            Duration::ZERO,
                        );
                        cx.schedule_emit(
                            NotificationServiceEvent::FinishAnimating,
                            Instant::now() + Duration::from_millis(240),
                        );
                    }
                } else {
                    self.enqueued_departures += 1;
                }
            }
        });
    }
}

pub struct NotificationPopups;

impl NotificationPopups {
    pub fn new<T: Lens<Target = Theme>>(cx: &mut Context, theme: T) -> Handle<Self> {
        Self.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx).height(Stretch(1.0)).width(Stretch(1.0));

                Binding::new(
                    cx,
                    NotificationService::displaying,
                    move |cx, displaying| {
                        let displaying = displaying.get(cx);
                        let len = displaying.len();

                        for (idx, notif) in displaying.into_iter().rev().enumerate() {
                            let anim_state = NotificationService::anim_state.get(cx);

                            let notif = NotificationView::new(cx, theme, notif);
                            if idx == 0 {
                                notif.id("slide-in-notif");

                                if anim_state == AnimationState::AnimatingIn {
                                    let anim_id = NotificationService::slide_in.get(cx);

                                    EventContext::new(cx).play_animation_for(
                                        anim_id,
                                        "slide-in-notif",
                                        Duration::from_millis(120),
                                        Duration::ZERO,
                                    );

                                    cx.schedule_emit(
                                        NotificationServiceEvent::FinishAnimating,
                                        Instant::now() + Duration::from_millis(120),
                                    );
                                }
                            } else if idx == len - 1 {
                                notif.id("slide-out-notif");
                            }
                        }
                    },
                );
            })
            .width(Pixels(300.0))
            .height(Pixels(160.0));
        })
        .alignment(Alignment::BottomLeft)
        .position_type(PositionType::Absolute)
        .width(Percentage(100.0))
        .height(Percentage(100.0))
        .z_index(5)
        .pointer_events(PointerEvents::None)
    }
}

impl View for NotificationPopups {}

pub enum NotificationServiceEvent {
    FinishAnimating,
    StartAnimatingIn,
    StartAnimatingOut,
    Timeout,
}

#[derive(Builder, Data, Clone)]
#[builder(start_fn = new)]
pub struct Notification {
    #[builder(into)]
    message: String,

    level: NotificationLevel,
}

#[derive(Clone, Data, PartialEq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

pub struct NotificationView {}

impl NotificationView {
    fn new<T: Lens<Target = Theme>>(
        cx: &mut Context,
        theme: T,
        notification: Notification,
    ) -> Handle<Self> {
        Self {}
            .build(cx, |cx| {
                let (icon, color, class) = match notification.level {
                    NotificationLevel::Info => (ICON_INFO_HEXAGON, "#5590ff", "info"),
                    NotificationLevel::Success => (ICON_HEXAGON_PLUS, "#55ff58", "success"),
                    NotificationLevel::Warning => (ICON_HELP_HEXAGON, "#fffb55", "warning"),
                    NotificationLevel::Error => (ICON_ALERT_HEXAGON, "#ff0055", "error"),
                };

                HStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Svg::new(cx, icon).class(&format!("{class}-svg")).z_index(2);
                        Label::new(cx, notification.message)
                            .color(theme.map(|theme| theme.text_primary))
                            .width(Stretch(1.0))
                            .text_overflow(TextOverflow::Ellipsis)
                            .z_index(2);
                    })
                    .alignment(Alignment::Left)
                    .gap(Pixels(6.0))
                    .padding(Pixels(4.0));
                })
                .overflow(Overflow::Hidden)
                .position_type(PositionType::Relative)
                .background_color(theme.map(|theme| theme.background))
                .corner_radius(Pixels(18.0))
                .border_color(color)
                .border_width(Pixels(2.0));
            })
            .background_color("transparent")
            .width(Stretch(1.0))
            .height(Pixels(40.0))
            .padding(Pixels(2.0))
            .padding_left(Pixels(10.0))
    }
}

impl View for NotificationView {
    fn element(&self) -> Option<&'static str> {
        Some("notification")
    }
}
