use std::fs;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;

use crate::module_path::ModulePath;

#[derive(Clone, Debug)]
pub struct GeneratedModule {
    path: ModulePath,
    source: String,
}

impl GeneratedModule {
    pub fn new(path: ModulePath, source: impl Into<String>) -> Self {
        Self {
            path,
            source: source.into(),
        }
    }

    #[must_use]
    pub fn path(&self) -> &ModulePath {
        &self.path
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn write_if_changed(&self, path: &Path) -> Result<bool> {
        if self.matches_existing(path)? {
            return Ok(false);
        }

        fs::write(path, &self.source)?;

        Ok(true)
    }

    fn matches_existing(&self, path: &Path) -> Result<bool> {
        match fs::read_to_string(path) {
            Ok(existing) => Ok(existing == self.source),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::GeneratedModule;
    use crate::module_path::ModulePath;

    #[test]
    fn writes_a_module_whose_file_is_absent_so_the_generated_source_exists() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("container.rs");
        let wrote = GeneratedModule::new(ModulePath::root("container"), "first")
            .write_if_changed(&path)
            .expect("the write succeeds");

        assert!(wrote);
        assert_eq!(
            fs::read_to_string(&path).expect("the file is readable"),
            "first"
        );
    }

    #[test]
    fn skips_writing_matching_content_so_unchanged_files_keep_their_timestamps() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("container.rs");
        let generated = GeneratedModule::new(ModulePath::root("container"), "same");
        generated
            .write_if_changed(&path)
            .expect("the first write succeeds");

        let wrote = generated
            .write_if_changed(&path)
            .expect("the second write succeeds");

        assert!(!wrote);
    }

    #[test]
    fn rewrites_a_file_whose_content_differs_so_stale_generated_code_is_replaced() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("container.rs");
        GeneratedModule::new(ModulePath::root("container"), "old")
            .write_if_changed(&path)
            .expect("the first write succeeds");

        let wrote = GeneratedModule::new(ModulePath::root("container"), "new")
            .write_if_changed(&path)
            .expect("the second write succeeds");

        assert!(wrote);
        assert_eq!(
            fs::read_to_string(&path).expect("the file is readable"),
            "new"
        );
    }

    #[test]
    fn propagates_a_read_failure_other_than_absence_instead_of_overwriting_the_path() {
        let directory = tempdir().expect("a temporary directory");

        let result = GeneratedModule::new(ModulePath::root("container"), "content")
            .write_if_changed(directory.path());

        assert!(result.is_err());
    }

    #[test]
    fn propagates_a_write_failure_so_an_unwritten_module_is_never_reported_as_written() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("missing").join("container.rs");

        let result =
            GeneratedModule::new(ModulePath::root("container"), "content").write_if_changed(&path);

        assert!(result.is_err());
    }
}
