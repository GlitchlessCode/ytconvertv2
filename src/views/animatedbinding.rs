use std::sync::Arc;

use animated_binding_derived_lenses::internal;
use vizia::animation::AnimId;

use super::*;

#[derive(Lens)]
pub struct AnimatedBinding<D: 'static + Data, L: 'static + Lens<Source: 'static, Target = D>> {
    internal: D,

    #[lens(ignore)]
    state: AnimationState,

    #[lens(ignore)]
    external: L,

    #[lens(ignore)]
    anim_out: Option<AnimationDef>,
    #[lens(ignore)]
    anim_in: Option<AnimationDef>,
}

impl<D, L> AnimatedBinding<D, L>
where
    D: 'static + Data,
    L: 'static + Lens<Source: 'static, Target = D>,
{
    pub fn new<F>(cx: &mut Context, lens: L, builder: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context, vizia::binding::Wrapper<internal<D, L>>),
    {
        Self {
            internal: lens.get(cx),
            state: AnimationState::None,
            external: lens,
            anim_out: None,
            anim_in: None,
        }
        .build(cx, |cx| {
            Binding::new(cx, AnimatedBinding::internal, builder);
        })
        .bind(lens, |handle, _| {
            let mut should_emit = false;
            let mut handle = handle.modify(|this| {
                if this.state == AnimationState::None {
                    should_emit = true;
                }
            });

            if should_emit {
                handle
                    .context()
                    .emit(AnimatedBindingEvent::StartAnimatingOut);
            }
        })
    }
}

impl<D, L> View for AnimatedBinding<D, L>
where
    D: 'static + Data,
    L: 'static + Lens<Source: 'static, Target = D>,
{
    fn element(&self) -> Option<&'static str> {
        Some("animatedbinding")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _meta| match event {
            AnimatedBindingEvent::StartAnimatingOut => {
                // Guard clause
                let ext_data = self.external.get(cx);

                if self.internal.same(&ext_data) {
                    return;
                }

                // If they are not the same, continue with animation
                self.state = AnimationState::AnimatingOut;
                if let Some(anim) = &self.anim_out {
                    cx.play_animation(anim.clone(), anim.duration, anim.delay);

                    cx.schedule_emit(
                        AnimatedBindingEvent::FinishedAnimatingOut,
                        Instant::now() + (anim.duration + anim.delay),
                    );
                } else {
                    cx.emit(AnimatedBindingEvent::FinishedAnimatingOut);
                }
            }
            AnimatedBindingEvent::FinishedAnimatingOut => {
                self.internal = self.external.get(cx);
                self.state = AnimationState::AnimatingIn;
                if let Some(anim) = &self.anim_in {
                    cx.play_animation(anim.clone(), anim.duration, anim.delay);

                    cx.schedule_emit(
                        AnimatedBindingEvent::FinishedAnimatingIn,
                        Instant::now() + (anim.duration + anim.delay),
                    );
                } else {
                    cx.emit(AnimatedBindingEvent::FinishedAnimatingIn);
                }
            }
            AnimatedBindingEvent::FinishedAnimatingIn => {
                let ext_data = self.external.get(cx);

                // Finish animation
                if self.internal.same(&ext_data) {
                    self.state = AnimationState::None;
                    return;
                }

                cx.emit(AnimatedBindingEvent::StartAnimatingOut);
            }
        });
    }
}

pub trait AnimatedBindingModifiers {
    fn set_anim_out(self, animation: AnimationDef) -> Self;
    fn set_anim_in(self, animation: AnimationDef) -> Self;
}

impl<'a, D, L> AnimatedBindingModifiers for Handle<'a, AnimatedBinding<D, L>>
where
    D: 'static + Data,
    L: 'static + Lens<Source: 'static, Target = D>,
{
    fn set_anim_out(self, animation: AnimationDef) -> Self {
        self.modify(|binding| binding.anim_out = Some(animation))
    }

    fn set_anim_in(self, animation: AnimationDef) -> Self {
        self.modify(|binding| binding.anim_in = Some(animation))
    }
}

#[derive(Debug, PartialEq)]
enum AnimatedBindingEvent {
    StartAnimatingOut,
    FinishedAnimatingOut,
    FinishedAnimatingIn,
}

#[derive(Debug, PartialEq)]
pub enum AnimationState {
    None,
    AnimatingOut,
    AnimatingIn,
}

#[derive(Clone)]
pub struct AnimationDef {
    anim_id: Arc<dyn AnimId>,
    duration: Duration,
    delay: Duration,
}

impl AnimationDef {
    pub fn new(anim_id: impl AnimId + 'static, duration: Duration, delay: Duration) -> Self {
        Self {
            anim_id: Arc::new(anim_id),
            duration,
            delay,
        }
    }
}

impl AnimId for AnimationDef {
    fn get(&self, cx: &EventContext) -> Option<Animation> {
        self.anim_id.get(cx)
    }
}
