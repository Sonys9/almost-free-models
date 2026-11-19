use crossterm::event::EventStream;
use rat_widget::list::{ListState, selection::RowSelection};
use ratatui::{DefaultTerminal, style::Color, text::Text, widgets::Padding};

use crate::locales::locale;

mod basics;
mod render;
mod sidebar;
mod pages;

const PADDING: Padding = Padding::new(1, 1, 0, 0);
const BG_COLOR: Color = Color::Rgb(22, 29, 36);
const FG_COLOR: Color = Color::Rgb(106, 124, 154);
const LIST_CHOOSEN_COLOR: Color = Color::Rgb(207, 216, 227);

pub struct App {
    pub reader: EventStream,
    state: AppState,
    should_break: bool,
}

pub struct AppState {
    list_state: ListState<RowSelection>,
    list_items: &'static [Page],
    current_page: Page,
}

impl App {
    pub fn new() -> Self {
        let mut state = AppState {
            list_state: ListState::default(),
            list_items: &[Page::Main],
            current_page: Page::Main,
        };
        state.list_state.focus.set(true);
        state.list_state.select(Some(0));
        Self { reader: EventStream::new(), state, should_break: false }
    }
    
    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.frame().await?;
            if self.should_break { break Ok(()) };
        }
    }
}

pub enum Page {
    Main,
}

impl Page {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Page::Main => locale().main_page,
        }
    }
}

impl<'a> From<&Page> for Text<'a> {
    fn from(page: &Page) -> Self {
        Text::from(page.as_str())
    }
}

impl<'a> From<Page> for Text<'a> {
    fn from(page: Page) -> Self {
        Text::from(&page)
    }
}

impl From<usize> for Page {
    fn from(value: usize) -> Self {
        match value {
            0 => Page::Main,
            _ => unreachable!()
        }
    }
}