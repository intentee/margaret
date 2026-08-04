use std::path::PathBuf;

use crate::scaffolded_file::ScaffoldedFile;

const GENERATED_MODULE_GITIGNORE: &str = "/margaret/\n";

#[must_use]
pub(crate) fn generated_module_gitignore() -> ScaffoldedFile {
    ScaffoldedFile {
        contents: GENERATED_MODULE_GITIGNORE.to_string(),
        relative_path: PathBuf::from(".gitignore"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::generated_module_gitignore;

    #[test]
    fn ignores_the_umbrella_module_next_to_the_member_manifest() {
        let gitignore = generated_module_gitignore();

        assert_eq!(gitignore.relative_path, PathBuf::from(".gitignore"));
        assert_eq!(gitignore.contents, "/margaret/\n");
    }
}
