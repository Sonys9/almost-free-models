use crate::{locales::Language, tui::App};

mod tui;
mod locales;

#[tokio::main]
async fn main() {
    Language::update();
    let mut app = App::new();
    let result = app.run(ratatui::init()).await;
    ratatui::restore();
    result.expect("The app exited unsuccessfuly");
}