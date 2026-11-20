use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rat_widget::event::{ChoiceOutcome, HandleEvent, Outcome, Popup, Regular};

use crate::{locales::Language, tui::App};

impl App {
    pub(crate) fn key_enter(&mut self, event: Event) {
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
        match self.state.locale_state.handle(&event, Popup) {
            ChoiceOutcome::Changed => return,
            ChoiceOutcome::Value => {
                let new_language = self.state.locale_state.value();
                Language::set(new_language);
                return;
            },
            _ => {},
        };
        if self.state.locale_state.is_popup_active() {
            return;
        };
        if self.state.list_state.handle(&event, Regular) == Outcome::Changed
            && let Some(index) = self.state.list_state.selected()
        {
            self.state.current_page = index.into();
        };
    }
}