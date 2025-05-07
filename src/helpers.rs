use std::any::Any;

use vizia::prelude::*;

use crate::theme::Theme;

pub fn labelled<T, F, Th>(
    cx: &mut Context,
    theme: Th,
    name: impl Res<T> + Clone,
    content: F,
) -> Handle<VStack>
where
    Th: Lens<Target = Theme>,
    T: ToStringLocalized,
    F: FnOnce(&mut Context),
{
    VStack::new(cx, |cx| {
        Label::new(cx, name)
            .color(theme.map(|theme| theme.text_secondary))
            .font_size("small");

        (content)(cx);
    })
    .gap(Pixels(1.5))
}

pub fn format_seconds(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = seconds / 60 % 60;
    let seconds = seconds % 60;

    if hours > 0 {
        format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
    } else {
        format!("{minutes:0>2}:{seconds:0>2}")
    }
}

pub fn parse_formatted_seconds(seconds: String) -> Option<u64> {
    let mut time = seconds.split([':', '.']);

    let hours: u64 = time.next()?.parse().ok()?;
    let minutes: u64 = time.next()?.parse().ok()?;
    let seconds: u64 = time.next()?.parse().ok()?;

    hours
        .checked_mul(3600)?
        .checked_add(minutes.checked_mul(60)?)?
        .checked_add(seconds)
}

static INVALID_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

pub fn make_filename_valid(filename: String) -> String {
    filename
        .chars()
        .map(|ch| if INVALID_CHARS.contains(&ch) { '_' } else { ch })
        .collect()
}

pub trait ContextProxyExt {
    fn emit_print_err<M: Any + Send>(&mut self, msg: M);
}

impl ContextProxyExt for ContextProxy {
    fn emit_print_err<M: Any + Send>(&mut self, msg: M) {
        if let Err(err) = self.emit(msg) {
            eprintln!("Failed to submit event to context proxy due to error: {err}")
        }
    }
}
