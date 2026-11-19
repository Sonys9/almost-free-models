use ratatui::{Frame, layout::{Constraint, Layout, Rect}, text::Line};

use crate::{
    locales::locale,
    tui::{App, Page},
};

impl App {
    pub(crate) fn test_page(&mut self, frame: &mut Frame, area: Rect) {
        let base = Self::block().title_top(Line::from(Page::TestPage.as_str()));
        let base_inner = base.inner(area);
        frame.render_widget(base, area);
    }
}
