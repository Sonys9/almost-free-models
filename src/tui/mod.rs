use crossterm::event::EventStream;
use rat_widget::{choice::ChoiceState, list::{ListState, selection::RowSelection}};
use ratatui::{DefaultTerminal, style::Color, text::Text, widgets::Padding};

use crate::{config::Config, locales::{Language, locale}};

mod basics;
mod pages;
mod render;
mod sidebar;
mod keys;

const PADDING: Padding = Padding::new(1, 1, 0, 0);
const BG_COLOR: Color = Color::Rgb(22, 29, 36);
const FG_COLOR: Color = Color::Rgb(106, 124, 154);
const LIST_CHOOSEN_COLOR: Color = Color::Rgb(207, 216, 227);

pub struct App {
    pub reader: EventStream,
    state: AppState,
    should_break: bool,
    config: Config,
}

pub struct AppState {
    list_state: ListState<RowSelection>,
    list_items: &'static [Page],
    current_page: Page,
    locale_state: ChoiceState<Language>,
    locales: &'static [(Language, &'static str)]
}

impl App {
    pub fn new(config: Config) -> Self {
        let mut state = AppState {
            list_state: ListState::default(),
            list_items: &[Page::Main, Page::TestPage, Page::Locales],
            current_page: Page::Main,
            locale_state: ChoiceState::default(),
            locales: &[
                (Language::En, "English"),
                (Language::Ru, "Русский"),
            ]
        };
        state.list_state.focus.set(true);
        state.list_state.select(Some(0));
        state.locale_state.set_value(Language::current());
        state.locale_state.focus.set(true);
        Self {
            reader: EventStream::new(),
            state,
            should_break: false,
            config,
        }
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.frame().await?;
            if self.should_break {
                break Ok(());
            };
        }
    }
}

pub enum Page {
    Main,
    TestPage,
    Locales
}

impl Page {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Page::Main => locale().main_page_with_spaces,
            Page::TestPage => locale().test_page_with_spaces,
            Page::Locales => locale().locales_page_with_spaces,
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
            1 => Page::TestPage,
            2 => Page::Locales,
            _ => unreachable!(),
        }
    }
}
