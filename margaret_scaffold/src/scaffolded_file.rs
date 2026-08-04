use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ScaffoldedFile {
    pub contents: String,
    pub relative_path: PathBuf,
}

impl ScaffoldedFile {
    #[must_use]
    pub(crate) fn nested_under(self, directory: &str) -> Self {
        Self {
            contents: self.contents,
            relative_path: Path::new(directory).join(self.relative_path),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::ScaffoldedFile;

    #[test]
    fn prefixes_the_relative_path_with_the_directory() {
        let nested = ScaffoldedFile {
            contents: "pub mod routes;\n".to_string(),
            relative_path: PathBuf::from("src").join("lib.rs"),
        }
        .nested_under("acme_base");

        assert_eq!(
            nested.relative_path,
            PathBuf::from("acme_base").join("src").join("lib.rs")
        );
        assert_eq!(nested.contents, "pub mod routes;\n");
    }
}
