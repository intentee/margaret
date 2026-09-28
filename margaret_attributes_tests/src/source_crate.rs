use std::fs;
use std::path::Path;
use std::path::PathBuf;

use tempfile::TempDir;
use tempfile::tempdir;

pub struct SourceCrate {
    directory: TempDir,
}

impl SourceCrate {
    /// # Panics
    ///
    /// Panics when the crate directory or its lib.rs cannot be written.
    #[must_use]
    pub fn new(lib_source: &str) -> Self {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        Self { directory }
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        self.directory.path()
    }

    #[must_use]
    pub fn source_directory(&self) -> PathBuf {
        self.root().join("src")
    }
}
