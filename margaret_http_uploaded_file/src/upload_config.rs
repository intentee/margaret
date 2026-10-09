use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub enum UploadConfig {
    Disabled,
    Enabled { directory: PathBuf },
}

impl UploadConfig {
    #[must_use]
    pub fn enabled(directory: PathBuf) -> Self {
        Self::Enabled { directory }
    }
}
