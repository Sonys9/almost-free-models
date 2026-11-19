use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::locales::Language;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub language: Option<Language>,
    // ...
}

impl Config {
    pub fn load() -> Result<Self, confy::ConfyError> {
        confy::load("almost-free-models", "config")
    }

    pub fn save(&self) -> Result<(), confy::ConfyError> {
        confy::store("almost-free-models", "config", self)
    }

    pub fn path(&self) -> Result<PathBuf, confy::ConfyError> {
        confy::get_configuration_file_path("almost-free-models", "config")
    }
}
