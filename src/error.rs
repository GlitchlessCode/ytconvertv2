use super::*;

#[derive(Builder)]
pub struct Error {
    pub code: u16,
    #[builder(into)]
    pub title: String,

    pub description: Option<String>,

    #[builder(setters(vis="pub(crate)", name=footer_internal))]
    pub footer: Option<Box<dyn Fn(&mut Context)>>,
}

use error_builder::{IsUnset, SetFooter, State};

impl<S: State> ErrorBuilder<S> {
    /// ***Optional** (Some / Option setters).*
    pub fn footer<F: Fn(&mut Context) + 'static>(self, value: F) -> ErrorBuilder<SetFooter<S>>
    where
        S::Footer: IsUnset,
    {
        self.footer_internal(Box::new(value))
    }
}

#[derive(Lens)]
pub struct ErrorManager {}

impl Model for ErrorManager {}
