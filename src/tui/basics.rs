use rat_widget::list::List;
use ratatui::{
    style::Style, text::Text, widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::tui::{App, BG_COLOR, FG_COLOR, LIST_CHOOSEN_COLOR, PADDING};

impl App {
    pub fn text<'a, T: Into<Text<'a>>>(text: T) -> Paragraph<'a> {
        Paragraph::new(text).wrap(Wrap { trim: true })
    }

    pub fn base() -> Block<'static> {
        Block::default()
            .padding(PADDING)
            .style(Style::default().bg(BG_COLOR).fg(FG_COLOR))
            .title_style(Style::default().bold())
    }

    pub fn block() -> Block<'static> {
        Self::base()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(FG_COLOR))
    }

    pub fn list<'a, T>(items: T) -> List<'a>
    where
        T: IntoIterator,
        T::Item: Into<Text<'a>>,
    {
        List::new(items)
            .style(Style::new().bg(BG_COLOR).fg(FG_COLOR))
            .select_style(Style::new().bg(BG_COLOR).fg(LIST_CHOOSEN_COLOR).bold())
            .focus_style(Style::new().bg(BG_COLOR).fg(LIST_CHOOSEN_COLOR).bold())
            .highlight_symbol("•")
    }
}
