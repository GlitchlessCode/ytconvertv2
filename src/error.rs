use std::sync::Arc;

use super::*;
use crate::{theme::Theme, views::error::ErrorView};

#[derive(Clone, Copy, Data, PartialEq, Debug)]
pub enum ErrorSeverity {
    Warning,
    Error,
}

#[derive(Builder, Data, Clone)]
#[builder(start_fn = new)]
pub struct Error {
    pub code: u16,
    #[builder(into)]
    pub title: String,

    pub severity: ErrorSeverity,

    #[builder(into)]
    pub description: Option<String>,

    #[builder(setters(vis="pub(crate)", name=footer_internal))]
    pub footer: Option<Arc<dyn Fn(&mut Context) + Send + Sync>>,
}

impl Error {
    fn fuzzy_score<M: FuzzyMatcher>(&self, matcher: &M, search_string: &str) -> i64 {
        let mut high_score = 0_i64;

        fuzzy_match(matcher, &mut high_score, &self.title, search_string);
        fuzzy_match(
            matcher,
            &mut high_score,
            &format!("E{:0>5}", self.code),
            search_string,
        );
        if let Some(ref text) = self.description {
            fuzzy_match(matcher, &mut high_score, &text, search_string);
        }

        high_score
    }
}

fn fuzzy_match<M: FuzzyMatcher>(matcher: &M, high_score: &mut i64, choice: &str, pattern: &str) {
    if let Some(score) = matcher.fuzzy_match(choice, pattern) {
        if score > *high_score {
            *high_score = score;
        }
    }
}

use error_builder::{IsUnset, SetFooter, State};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

impl<S: State> ErrorBuilder<S> {
    /// ***Optional** (Some / Option setters).*
    pub fn footer<F: Fn(&mut Context) + Send + Sync + 'static>(
        self,
        value: F,
    ) -> ErrorBuilder<SetFooter<S>>
    where
        S::Footer: IsUnset,
    {
        self.footer_internal(Arc::new(value))
    }
}

#[derive(Lens, Default)]
pub struct ErrorManager {
    pub errors: Vec<Error>,

    pub warning_count: usize,
    pub error_count: usize,

    pub popup_open: bool,
    pub popup_search_str: String,
    pub popup_sorted_errors: Vec<Error>,

    generation: usize,
    last_recieved: usize,
}

impl ErrorManager {
    fn process_error_list(&mut self, cx: &mut EventContext, search_string: String) {
        self.popup_sorted_errors = Vec::new();

        self.generation += 1;

        let generation = self.generation;
        let search_list = self.errors.clone();

        cx.spawn(move |cx| {
            if search_string.trim().is_empty() {
                if let Err(_) = cx.emit(ErrorManagerEvent::FinishFuzzySearch(
                    search_list,
                    generation,
                )) {
                    eprintln!("Could not submit fuzzy search results, failed to send event");
                }
            } else {
                let matcher = SkimMatcherV2::default();

                let mut mapped_list = search_list
                    .into_iter()
                    .map(|error| (error.fuzzy_score(&matcher, &search_string), error))
                    .collect::<Vec<(i64, Error)>>();

                mapped_list.sort_by(|(sort_index_a, _), (sort_index_b, _)| {
                    // Flipped sorting order
                    sort_index_b.cmp(sort_index_a)
                });

                let sorted_list = mapped_list.into_iter().map(|(_, error)| error).collect();

                if let Err(_) = cx.emit(ErrorManagerEvent::FinishFuzzySearch(
                    sorted_list,
                    generation,
                )) {
                    eprintln!("Could not submit fuzzy search results, failed to send event");
                }
            }
        });
    }
}

impl Model for ErrorManager {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|event, _meta| {
            let Error { severity, .. } = event;

            match severity {
                ErrorSeverity::Error => self.error_count += 1,
                ErrorSeverity::Warning => self.warning_count += 1,
            }

            self.errors.insert(0, event);

            self.process_error_list(cx, self.popup_search_str.clone());
        });

        event.take(|event, _meta| match event {
            ErrorManagerEvent::SetPopup(state) => self.popup_open = state,
            ErrorManagerEvent::SetSearchString(state) => {
                self.popup_search_str = state.clone();
                self.process_error_list(cx, state);
            }
            ErrorManagerEvent::FinishFuzzySearch(list, generation) => {
                if generation > self.last_recieved {
                    self.popup_sorted_errors = list;
                }
            }
        })
    }
}

/// Add the error popup to the tree. Assumes the existance of an ErrorManager.
pub fn error_popup<T: Lens<Target = Theme>>(cx: &mut Context, theme: T) {
    Binding::new(cx, ErrorManager::popup_open, move |cx, open| {
        if open.get(cx) {
            Window::new(cx, move |cx| {
                VStack::new(cx, |cx| {
                    Textbox::new(cx, ErrorManager::popup_search_str)
                        .on_edit(|ex, state| ex.emit(ErrorManagerEvent::SetSearchString(state)))
                        .width(Stretch(1.0))
                        .background_color(theme.map(|theme| theme.background_dark))
                        .color(theme.map(|theme| theme.text_primary));

                    VirtualList::new(
                        cx,
                        ErrorManager::popup_sorted_errors,
                        160.0,
                        move |cx, _, error| {
                            ErrorView::new(cx, theme, error.get(cx)).top(Pixels(3.0))
                        },
                    );
                })
                .padding(Pixels(6.0))
                .padding_bottom(Pixels(0.0))
                .gap(Pixels(3.0))
                .background_color(theme.map(|theme| theme.background));
            })
            .min_inner_size(Some((600, 400)))
            .title("Errors & Warnings")
            .on_close(|ex| ex.emit(ErrorManagerEvent::SetPopup(false)));
        }
    });
}

pub enum ErrorManagerEvent {
    SetPopup(bool),
    SetSearchString(String),
    FinishFuzzySearch(Vec<Error>, usize),
}
