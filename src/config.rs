use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::locales::Language;

#[derive(Debug, Default, Serialize, Deserialize)]
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

    pub async fn new_file<T: AsRef<str>, B: AsRef<[u8]>>(&self, file_name: T, content: B) -> eyre::Result<()> {
        self.file(file_name, content, false).await
    }

    pub async fn append_to_file<T: AsRef<str>, B: AsRef<[u8]>>(&self, file_name: T, content: B) -> eyre::Result<()> {
        self.file(file_name, content, true).await
    }

    pub async fn file<T: AsRef<str>, B: AsRef<[u8]>>(&self, file_name: T, content: B, append: bool) -> eyre::Result<()> {
        let cfg_path = self.path()?;
        let Some(folder_path) = cfg_path.parent() else {
            eyre::bail!("Failed to get a config parent folder");
        };
        let file_path = folder_path.join(file_name.as_ref());
        OpenOptions::new()
            .create(true)
            .append(append)
            .truncate(!append)
            .write(true)
            .open(file_path).await?
            .write_all(content.as_ref())
            .await?;
        Ok(())
    }
}
