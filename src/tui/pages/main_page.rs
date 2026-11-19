use ratatui::{Frame, layout::Rect, text::Line};

use crate::{locales::locale, tui::{App, Page}};

impl App {
    pub(crate) fn main_page(&mut self, frame: &mut Frame, area: Rect) {
        let base = Self::block()
            .title_top(Line::from(Page::Main.as_str()));
        let base_inner = base.inner(area);
        frame.render_widget(base, area);
        
        frame.render_widget(
            //Self::text(concat!("Almost Free Models, v", env!("CARGO_PKG_VERSION"))),
            Self::text(&format!("{}, v{}", locale().app_name, env!("CARGO_PKG_VERSION"))),
            base_inner
        );
        frame.render_widget(
            Self::text(locale().sidebar_swipe_hint),
            base_inner
        );
    }
}