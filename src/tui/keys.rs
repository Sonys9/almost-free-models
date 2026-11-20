use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rat_widget::event::{ChoiceOutcome, HandleEvent, Outcome, Popup, Regular};

use crate::{locales::Language, tui::App};

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
        match self.state.locale_state.handle(&event, Popup) {
            ChoiceOutcome::Changed => return Ok(()),
            ChoiceOutcome::Value => {
                let new_language = self.state.locale_state.value();
                self.save_language(new_language)?;
                return Ok(());
            },
            _ => {},
        };
        if self.state.locale_state.is_popup_active() {
            return Ok(());
        };
        if self.state.list_state.handle(&event, Regular) == Outcome::Changed
            && let Some(index) = self.state.list_state.selected()
        {
            self.state.current_page = index.into();
        };
        Ok(())
    }

    fn save_language(&mut self, new_language: Language) -> eyre::Result<()> {
        Language::set(new_language);
        self.config.language = Some(new_language);
        self.config.save()?;
        Ok(())
    }
}