use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use rat_widget::event::{HandleEvent, Outcome, Regular};
use ratatui::{Frame, layout::{Constraint, Layout}, widgets::StatefulWidget};

use crate::{locales::locale, tui::{App, Page}};

impl App {
    pub(crate) async fn frame(&mut self) -> eyre::Result<()> {
        tokio::select! {
            Some(Ok(event)) = self.reader.next() => {
                return Ok(self.key_enter(event));
            }
            else => return Ok(()),
        }
    }

    fn key_enter(&mut self, event: Event)  {
        if let Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_break = true;
            return;
        };
        if self.state.list_state.handle(&event, Regular) == Outcome::Changed 
            && let Some(index) = self.state.list_state.selected() 
        {
            self.state.current_page = index.into();
        };
    }
    
    pub(crate) fn render(&mut self, frame: &mut Frame) {
        let base = Self::block()
            .title_top(locale().app_name)
            .title_bottom(locale().quit_hint);
        let base_inner = base.inner(frame.area());
        frame.render_widget(base, frame.area());

        let [sidebar_area, rest] = Layout::horizontal(
            [
                Constraint::Percentage(15),
                Constraint::Fill(1)
            ])
            .spacing(1)
            .areas(base_inner);

        let sidebar = Self::list(self.state.list_items);
        sidebar.render(sidebar_area, frame.buffer_mut(), &mut self.state.list_state);

        match self.state.current_page {
            Page::Main => self.main_page(frame, rest),
        };
    }
}