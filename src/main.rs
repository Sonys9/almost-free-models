use crate::tui::App;

mod tui;

#[tokio::main]
async fn main() {
    let mut app = App::new();
    let result = app.run(ratatui::init()).await;
    ratatui::restore();
    result.unwrap();
}

// let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
// tracing_subscriber::fmt().with_env_filter(filter).init();