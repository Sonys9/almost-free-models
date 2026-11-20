use rat_widget::choice::Choice;
use ratatui::{Frame, layout::{Constraint, Layout, Rect}, text::Line};

use crate::{
    locales::{Language, locale}, tui::{App, Page},
};

impl App {
    pub(crate) fn locales_page(&mut self, frame: &mut Frame, area: Rect) {
        let base = Self::block().title_top(Line::from(Page::Locales.as_str()));
        let base_inner = base.inner(area);
        frame.render_widget(base, area);

        let positions = Layout::vertical([
            Constraint::Length(1); 2
        ]).split(base_inner);

        let (choice, popup) = Choice::new()
            .items([
                (Language::En, "First language"),
                (Language::En, "Second language"),
            ])
            .into_widgets();

        frame.render_stateful_widget(choice, positions[0], &mut self.state.locale_state);
        frame.render_stateful_widget(popup, positions[1], &mut self.state.locale_state);
    }
}
