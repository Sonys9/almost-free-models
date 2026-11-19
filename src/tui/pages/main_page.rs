use ratatui::{Frame, layout::Rect, text::Line};

use crate::tui::{App, Page};

impl App {
    pub(crate) fn main_page(&mut self, frame: &mut Frame, area: Rect) {
        let base = Self::block()
            .title_top(Line::from(Page::Main.as_str()));
        frame.render_widget(base, area);
    }
}