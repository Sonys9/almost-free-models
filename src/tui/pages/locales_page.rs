use rat_widget::choice::Choice;
use ratatui::{Frame, layout::{Constraint, Layout, Rect}, text::Line};

use crate::{
    locales::Language, tui::{App, Page},
};

impl App {
    pub(crate) fn locales_page(&mut self, frame: &mut Frame, area: Rect) {
        let base = Self::block().title_top(Line::from(Page::Locales.as_str()));
        let base_inner = base.inner(area);
        frame.render_widget(base, area);

        let [choice_col, _] = Layout::horizontal([
            Constraint::Length(15),
            Constraint::Min(0),
        ])
        .areas(base_inner);

        let [choice_area, _] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(choice_col);

        let (choice, popup) = Self::dropbox(self.state.locales);
        frame.render_stateful_widget(choice, choice_area, &mut self.state.locale_state);
        frame.render_stateful_widget(popup, base_inner, &mut self.state.locale_state);
    }
}
