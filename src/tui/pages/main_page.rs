use ratatui::{Frame, layout::{Constraint, Layout, Rect}, text::Line};

use crate::{
    locales::locale,
    tui::{App, Page},
};

impl App {
    pub(crate) fn main_page(&mut self, frame: &mut Frame, area: Rect) {
        let base = Self::block().title_top(Line::from(Page::Main.as_str()));
        let base_inner = base.inner(area);
        frame.render_widget(base, area);

        let positions = Layout::vertical([
            Constraint::Length(1); 3
        ]).split(base_inner);

        frame.render_widget(
            Self::text(format!(
                "{}, v{}",
                locale().app_name,
                env!("CARGO_PKG_VERSION")
            )),
            positions[0],
        );
        frame.render_widget(Self::text(locale().sidebar_swipe_hint), positions[1]);
        frame.render_widget(
            Self::text(format!(
                "{}{}",
                locale().config_path,
                self.config
                    .path()
                    .unwrap_or_else(|_| "Unknown".into())
                    .to_string_lossy()
            )),
            positions[2],
        );
    }
}
