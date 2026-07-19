use std::io::ErrorKind;
use std::io::Result as IoResult;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct GeneratedModule {
    name: String,
    source: String,
}

impl GeneratedModule {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn write_if_changed(&self, path: &Path) -> IoResult<bool> {
        if self.matches_existing(path)? {
            return Ok(false);
        }

        std::fs::write(path, &self.source)?;

        Ok(true)
    }

    fn matches_existing(&self, path: &Path) -> IoResult<bool> {
        match std::fs::read_to_string(path) {
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

    #[test]
    fn writes_when_the_file_is_absent() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("container.rs");
        let wrote = GeneratedModule::new("container", "first")
            .write_if_changed(&path)
            .expect("the write succeeds");

        assert!(wrote);
        assert_eq!(
            fs::read_to_string(&path).expect("the file is readable"),
            "first"
        );
    }

    #[test]
    fn skips_writing_when_the_content_matches() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("container.rs");
        let generated = GeneratedModule::new("container", "same");
        generated
            .write_if_changed(&path)
            .expect("the first write succeeds");

        let wrote = generated
            .write_if_changed(&path)
            .expect("the second write succeeds");

        assert!(!wrote);
    }

    #[test]
    fn rewrites_when_the_content_differs() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("container.rs");
        GeneratedModule::new("container", "old")
            .write_if_changed(&path)
            .expect("the first write succeeds");

        let wrote = GeneratedModule::new("container", "new")
            .write_if_changed(&path)
            .expect("the second write succeeds");

        assert!(wrote);
        assert_eq!(
            fs::read_to_string(&path).expect("the file is readable"),
            "new"
        );
    }

    #[test]
    fn propagates_a_read_error() {
        let directory = tempdir().expect("a temporary directory");

        let result =
            GeneratedModule::new("container", "content").write_if_changed(directory.path());

        assert!(result.is_err());
    }

    #[test]
    fn propagates_a_write_error() {
        let directory = tempdir().expect("a temporary directory");
        let path = directory.path().join("missing").join("container.rs");

        let result = GeneratedModule::new("container", "content").write_if_changed(&path);

        assert!(result.is_err());
    }
}
