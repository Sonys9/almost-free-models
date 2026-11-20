use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rat_widget::event::{ChoiceOutcome, HandleEvent, Outcome, Popup, Regular};

use crate::{
    locales::Language,
    tui::{App, Page},
};

impl App {
    pub(crate) fn key_enter(&mut self, event: Event) -> eyre::Result<()> {
        if let Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_break = true;
            return Ok(());
        };
        if self.locale_key(&event)? {
            return Ok(());
        }
        if self.state.list_state.handle(&event, Regular) == Outcome::Changed
            && let Some(index) = self.state.list_state.selected()
        {
            self.state.current_page = index.into();
        };
        Ok(())
    }

    /// Routes an event to the language dropdown and reports whether it was consumed.
    ///
    /// An open popup owns every event. A closed one only receives the events that
    /// can open it (Space/Enter or a mouse click), so that the arrow keys keep
    /// navigating the sidebar instead of opening the popup and trapping input here.
    fn locale_key(&mut self, event: &Event) -> eyre::Result<bool> {
        if !matches!(self.state.current_page, Page::Locales) {
            return Ok(false);
        }

        let was_active = self.state.locale_state.is_popup_active();
        let open_key = matches!(
            event,
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char(' ') | KeyCode::Enter,
                ..
            })
        );
        if !was_active && !open_key && !matches!(event, Event::Mouse(_)) {
            return Ok(false);
        }

        let outcome = self.state.locale_state.handle(event, Popup);
        if outcome == ChoiceOutcome::Value {
            let new_language = self.state.locale_state.value();
            self.save_language(new_language)?;
        }

        // A click on the sidebar leaves the popup closed, so let the list handle it.
        Ok(was_active || self.state.locale_state.is_popup_active())
    }

    fn save_language(&mut self, new_language: Language) -> eyre::Result<()> {
        Language::set(new_language);
        self.config.language = Some(new_language);
        self.config.save()?;
        Ok(())
    }
}
