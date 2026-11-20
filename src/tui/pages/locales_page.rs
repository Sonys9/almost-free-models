use rat_widget::{choice::Choice, toolbar::ToolState::BasicCheckbox};
use ratatui::{Frame, layout::{Constraint, Layout, Rect}, text::Line};

use crate::{
    locales::{Language, locale}, tui::{App, Page},
};

impl App {
    pub(crate) fn locales_page(&mut self, frame: &mut Frame, area: Rect) {
        let base = Self::block().title_top(Line::from(Page::Locales.as_str()));
        let base_inner = base.inner(area);
        frame.render_widget(base, area);

        let hint = Self::text(locale().config_saves_hint);
        let [top_row, hint_area, _, preview_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(hint.line_count(base_inner.width) as u16),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(base_inner);

        let [choice_area, _] = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Min(0),
        ])
        .areas(top_row);

        frame.render_widget(hint, hint_area);

        let preview = Self::block()
            .title_top(locale().preview_title_with_spaces);
        let preview_inner = preview.inner(preview_area);
        frame.render_widget(preview, preview_area);
        frame.render_widget(Self::text(locale().preview_text), preview_inner);

        let (choice, popup) = Self::dropbox(self.state.locales);
        frame.render_stateful_widget(choice, choice_area, &mut self.state.locale_state);
        frame.render_stateful_widget(popup, base_inner, &mut self.state.locale_state);
    }
}
