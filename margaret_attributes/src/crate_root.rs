use std::path::PathBuf;

#[derive(Clone)]
pub struct CrateRoot {
    pub name: String,
    pub source_directory: PathBuf,
}

impl CrateRoot {
    pub fn new(name: impl Into<String>, source_directory: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            source_directory: source_directory.into(),
        }
    }
}
