use bon::Builder;
use vizia::prelude::*;

pub mod data;
pub mod helpers;
pub mod modifiers;
pub mod theme;
pub mod views;

#[cfg(not(windows))]
#[macro_export]
macro_rules! main_separator {
    () => {
        "/"
    };
}

#[cfg(windows)]
#[macro_export]
macro_rules! main_separator {
    () => {
        r#"\"#
    };
}

#[macro_export]
macro_rules! include_bytes_safe {
    // Empty path segments
    () => {
        compile_error!("This macro requires a series of path segments");
    };

    // Success
    ($first:literal $(, $extra:literal)*) => {
        include_bytes!(concat!(
            $first,
            $(
                $crate::main_separator!(),
                $extra,
            )*

        ));
    };
}
