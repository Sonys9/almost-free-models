/// Locales
/// 
/// You can make your own locales by using locales!(locale LANGUAGE(..))
///
/// It automaticly creates <name>_with_spaces fields (example: "Main menu" -> " Main menu ")

use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize, de::Visitor};

use crate::{config::Config, locales::en::EN, locales::ru::RU};

macro_rules! locales {
    (keys ($($key:ident),* $(,)? )) => {
        paste::paste! {
            #[allow(dead_code)]
            pub struct Locale {
                $(
                    pub $key: &'static str,
                    pub [<$key _with_spaces>]: &'static str,
                )*
            }
        }
    };

    (locale $name:ident ( $($key:ident : $val:expr),* $(,)? )) => {
        paste::paste! {
            pub const $name: Locale = Locale {
                $(
                    $key: $val,
                    [<$key _with_spaces>]: concat!(" ", $val, " "),
                )*
            };
        }
    };
}

locales!(
    keys (
        main_page,
        app_name,
        config_path,
        quit_hint,
        sidebar_swipe_hint,
        test_page,
        locales_page,
    )
);

static LANGUAGE: AtomicU8 = AtomicU8::new(Language::En as u8);

pub mod en;
pub mod ru;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[default]
    En = 0,
    Ru = 1,
}

impl TryFrom<u8> for Language {
    type Error = !;

    fn try_from(id: u8) -> Result<Self, !> {
        Ok(match id {
            0 => Language::En,
            1 => Language::Ru,
            _ => unreachable!(),
        })
    }
}

impl From<&str> for Language {
    fn from(lang: &str) -> Self {
        match lang {
            _ => Language::En,
        }
    }
}

pub fn locale() -> &'static Locale {
    Language::current().locale()
}

impl Language {
    /// Should detect user's default Language
    /// If there's a config it should use it
    pub fn update(config: &Config) {
        let language: Language = config.language.unwrap_or_else(|| {
            std::env::var("LANG")
                .unwrap_or_else(|_| "en".to_string())
                .to_lowercase()
                .as_str()
                .into()
        });
        Self::set(language);
    }

    /// Get locale (Locale) by language
    pub fn locale(&self) -> &'static Locale {
        match self {
            Language::Ru => &RU,
            Language::En => &EN
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            // If the app isn't localised for some language it will
            // fallback to English (and if it's an English it will still fallback to English)
            Language::En => "en",
            Language::Ru => "ru",
        }
    }

    /// Updates LANGUAGE to the new language
    pub fn set(language: Language) {
        LANGUAGE.store(language as u8, Ordering::Relaxed);
    }

    pub fn current() -> Language {
        Language::try_from(LANGUAGE.load(Ordering::Relaxed))
            .unwrap()
    }
}