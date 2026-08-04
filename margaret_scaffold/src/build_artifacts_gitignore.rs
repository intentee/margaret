use std::path::PathBuf;

use crate::scaffolded_file::ScaffoldedFile;

const BUILD_ARTIFACTS_GITIGNORE: &str = "/target\n";

#[must_use]
pub(crate) fn build_artifacts_gitignore() -> ScaffoldedFile {
    ScaffoldedFile {
        contents: BUILD_ARTIFACTS_GITIGNORE.to_string(),
        relative_path: PathBuf::from(".gitignore"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::build_artifacts_gitignore;

    #[test]
    fn ignores_the_cargo_target_directory_at_the_workspace_root() {
        let gitignore = build_artifacts_gitignore();

        assert_eq!(gitignore.relative_path, PathBuf::from(".gitignore"));
        assert_eq!(gitignore.contents, "/target\n");
    }
}
