use std::panic;

use crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, terminal::EnterAlternateScreen};

use crate::{config::Config, locales::Language, tui::App};

mod config;
mod locales;
mod tui;

#[tokio::main]
async fn main() {
    prepare();
    let config = Config::load().expect("Failed to load the configuration file");
    println!("{:?}", config);
    Language::update(&config);

    let mut app = App::new(config);
    let result = app.run(ratatui::init()).await;

    ratatui::restore();
    result.expect("The app exited unsuccessfuly");
    clean();
}

fn prepare() {
    crossterm::terminal::enable_raw_mode()
        .expect("Failed to enable crossterm raw mode");
    crossterm::execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)
        .expect("Failed to crossterm::execute! to get mouse capture");

    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        clean();
        default_hook(info);
    }));
}

fn clean() {
    let _ = crossterm::execute!(std::io::stdout(), DisableMouseCapture);
    ratatui::restore();
}