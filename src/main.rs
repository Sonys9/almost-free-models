use crate::{config::Config, locales::Language, tui::App};

mod config;
mod locales;
mod tui;

#[tokio::main]
async fn main() {
    let config = Config::load().expect("Failed to load the configuration file");
    Language::update(&config);
    let mut app = App::new(config);
    let result = app.run(ratatui::init()).await;
    ratatui::restore();
    result.expect("The app exited unsuccessfuly");
}
