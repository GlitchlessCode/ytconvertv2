use reqwest::blocking::Client;
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        // Add the stylesheet to the app
        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("Failed to load stylesheet");

        AppData {}.build(cx); // Build the data into the app
    })
    .inner_size((400, 400))
    .run()
}
