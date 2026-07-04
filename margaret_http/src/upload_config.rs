use std::path::PathBuf;

pub enum UploadConfig {
    Disabled,
    Enabled { directory: PathBuf },
}

impl UploadConfig {
    pub fn enabled(directory: PathBuf) -> Self {
        Self::Enabled { directory }
    }
}
