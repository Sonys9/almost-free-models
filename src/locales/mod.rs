use std::sync::atomic::{AtomicU8, Ordering};

use crate::locales::en::EN;

macro_rules! locales {
    ($($key:ident),* $(,)? ) => {
        pub struct Locale {
            $(pub $key: &'static str,)*
        }
    };
}

locales!(
    main_page, app_name,
    
    quit_hint, sidebar_swipe_hint,
);

static LANGUAGE: AtomicU8 = AtomicU8::new(Language::En as u8);

pub mod en;

pub enum Language {
    En = 0,
}

impl TryFrom<u8> for Language {
    type Error = !;

    fn try_from(id: u8) -> Result<Self, !> {
        Ok(match id {
            0 => Language::En,
            _ => unreachable!()
        })
    }
}

pub fn locale() -> &'static Locale {
    Language::try_from(LANGUAGE.load(Ordering::Relaxed)).unwrap().locale()
}

impl Language {
    /// Should detect user's default Language
    /// If there's a config it should use it
    pub fn update() {
        // config is not implemented yet
        let language = match std::env::var("LANG").unwrap_or_else(|_| "en".to_string()) {
            // If the app isn't localised for some language it will
            // fallback to English (and if it's an English it will still fallback to English)
            _ => Language::En,
        };
        let language_id = language as u8;
        LANGUAGE.store(language_id, Ordering::Relaxed);
    }

    /// Get locale (Locale) by language
    pub fn locale(&self) -> &'static Locale {
        match self {
            _ => &EN
        }
    }
}