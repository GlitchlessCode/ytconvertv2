use std::any::Any;

use vizia::prelude::*;

use crate::{modifiers::ThemeModifiers, theme::Theme};

#[cfg(not(windows))]
#[inline(always)]
pub(crate) fn python_path() -> impl AsRef<std::path::Path> {
    #[cfg(not(feature = "mac-release"))]
    return "/usr/local/bin/python3";

    #[cfg(feature = "mac-release")]
    return {
        match std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(std::path::PathBuf::from))
        {
            Some(path) => path.join("python").join("bin").join("python3"),
            None => std::path::PathBuf::from("/usr/local/bin/python3"), // Fallback
        }
    };
}

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
            .with_text_secondary(theme)
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

pub fn format_color_hex(color: Color) -> String {
    format!("#{:0>2x}{:0>2x}{:0>2x}", color.r(), color.g(), color.b())
}

pub fn parse_color_hex(hex: &String) -> Option<Color> {
    let mut chars = hex.chars().peekable();
    if chars.peek().map(|ch| *ch) == Some('#') {
        let _ = chars.next(); // Discard starting #
    }

    // Get char sets
    let r: String = chars.by_ref().take(2).collect();
    let g: String = chars.by_ref().take(2).collect();
    let b: String = chars.by_ref().take(2).collect();

    let r = u8::from_str_radix(&r, 16).ok()?;
    let g = u8::from_str_radix(&g, 16).ok()?;
    let b = u8::from_str_radix(&b, 16).ok()?;

    Some(Color::rgb(r, g, b))
}

static INVALID_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

pub fn make_filename_valid(filename: String) -> String {
    filename
        .chars()
        .map(|ch| if INVALID_CHARS.contains(&ch) { '_' } else { ch })
        .collect()
}

pub fn fetch_thumbnail<F>(
    cx: &mut ContextProxy,
    client: reqwest::blocking::Client,
    url: String,
    callback: F,
) where
    F: Fn(&mut ContextProxy, String, reqwest::blocking::Response) -> Result<(), ProxyEmitError>,
{
    let request = match client.get(&url).build() {
        Err(e) => {
            let error = crate::error::Error::new()
                .title("Reqwest Request Builder Failed")
                .code(200)
                .description(format!(
                    "Failed to create a build a Request due to the following error from reqwest: {e}"
                ))
                .severity(crate::error::ErrorSeverity::Warning)
                .build();

            cx.emit_print_err(error);
            return;
        }
        Ok(request) => request,
    };

    let res = match client.execute(request) {
        Err(e) => {
            let error = crate::error::Error::new()
                .title("Reqwest Request Execution Failed")
                .code(201)
                .description(format!(
                    "Failed execute a built Request due to the following error from reqwest: {e}"
                ))
                .severity(crate::error::ErrorSeverity::Warning)
                .build();

            cx.emit_print_err(error);
            return;
        }
        Ok(response) => response,
    };

    if let Err(_) = callback(cx, url, res) {
        let error = crate::error::Error::new()
            .title("Event Emission Failure")
            .code(3)
            .description("Failed to emit an error to the context proxy while submitting a finished thumbnail")
            .severity(crate::error::ErrorSeverity::Warning)
            .build();

        cx.emit_print_err(error);
    }
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
