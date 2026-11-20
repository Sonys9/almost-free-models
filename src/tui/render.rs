use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use rat_widget::event::{ChoiceOutcome, HandleEvent, Outcome, Popup, Regular};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::StatefulWidget,
};

use crate::{
    locales::{Language, locale}, tui::{App, Page},
};

impl App {
    pub(crate) async fn frame(&mut self) -> eyre::Result<()> {
        tokio::select! {
            Some(Ok(event)) = self.reader.next() => {
                return Ok(self.key_enter(event));
            }
            else => return Ok(()),
        }
    }

    pub(crate) fn render(&mut self, frame: &mut Frame) {
        let base = Self::base()
            .title_top(locale().app_name_with_spaces)
            .title_bottom(locale().quit_hint_with_spaces);
        let base_inner = base.inner(frame.area());
        frame.render_widget(base, frame.area());

        let [sidebar_area, rest] =
            Layout::horizontal([Constraint::Percentage(15), Constraint::Fill(1)])
                .spacing(1)
                .areas(base_inner);

        let sidebar = Self::list(self.state.list_items);
        sidebar.render(sidebar_area, frame.buffer_mut(), &mut self.state.list_state);

        match self.state.current_page {
            Page::Main => self.main_page(frame, rest),
            Page::TestPage => self.test_page(frame, rest),
            Page::Locales => self.locales_page(frame, rest),
        };
    }
}
