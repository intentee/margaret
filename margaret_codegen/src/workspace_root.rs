use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use toml::Table;

use crate::codegen_error::CodegenError;
use crate::workspace_location::WorkspaceLocation;

fn is_workspace_manifest(manifest_path: &Path) -> Result<bool, CodegenError> {
    let contents = match fs::read_to_string(manifest_path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(source) => {
            return Err(CodegenError::ReadWorkspaceManifest {
                path: manifest_path.to_path_buf(),
                source,
            });
        }
    };

    let document =
        contents
            .parse::<Table>()
            .map_err(|source| CodegenError::ParseWorkspaceManifest {
                path: manifest_path.to_path_buf(),
                source: Box::new(source),
            })?;

    Ok(document.contains_key("workspace"))
}

pub(crate) fn workspace_root(manifest_directory: &Path) -> Result<WorkspaceLocation, CodegenError> {
    let mut current = manifest_directory;
    let mut depth = 0;

    loop {
        if is_workspace_manifest(&current.join("Cargo.toml"))? {
            return Ok(WorkspaceLocation::new(current.to_path_buf(), depth));
        }

        match current.parent() {
            Some(parent) => {
                current = parent;
                depth += 1;
            }
            None => {
                return Err(CodegenError::WorkspaceRootNotFound {
                    start: manifest_directory.to_path_buf(),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::workspace_root;
    use crate::codegen_error::CodegenError;

    #[test]
    fn resolves_a_manifest_that_is_its_own_workspace_root() {
        let directory = tempdir().expect("a temporary workspace directory");
        fs::write(directory.path().join("Cargo.toml"), "[workspace]\n")
            .expect("the workspace manifest is written");

        let location = workspace_root(directory.path()).expect("the workspace root is resolved");

        assert_eq!(location.root(), directory.path());
        assert_eq!(location.embed_relative(), ".");
    }

    #[test]
    fn resolves_the_nearest_ancestor_that_declares_a_workspace() {
        let root = tempdir().expect("a temporary workspace directory");
        fs::write(root.path().join("Cargo.toml"), "[workspace]\n")
            .expect("the workspace manifest is written");
        let member = root.path().join("member");
        fs::create_dir(&member).expect("the member directory is created");
        fs::write(
            member.join("Cargo.toml"),
            "[package]\nname = \"member\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .expect("the member manifest is written");

        let location = workspace_root(&member).expect("the workspace root is resolved");

        assert_eq!(location.root(), root.path());
        assert_eq!(location.embed_relative(), "..");
    }

    #[test]
    fn reports_a_manifest_directory_without_any_workspace_ancestor() {
        let directory = tempdir().expect("a temporary directory");
        let error = workspace_root(directory.path())
            .err()
            .expect("a missing workspace is reported");

        assert!(matches!(
            &error,
            CodegenError::WorkspaceRootNotFound { start } if start == directory.path()
        ));
        assert!(
            error
                .to_string()
                .contains("no Cargo workspace root was found")
        );
    }

    #[test]
    fn reports_a_workspace_manifest_that_cannot_be_read() {
        let directory = tempdir().expect("a temporary directory");
        fs::create_dir(directory.path().join("Cargo.toml"))
            .expect("the blocking directory is created");
        let error = workspace_root(directory.path())
            .err()
            .expect("an unreadable manifest is reported");

        assert!(matches!(
            &error,
            CodegenError::ReadWorkspaceManifest { path, .. }
                if *path == directory.path().join("Cargo.toml")
        ));
        assert!(
            error
                .to_string()
                .contains("failed to read the workspace manifest")
        );
    }

    #[test]
    fn reports_a_workspace_manifest_that_is_not_valid_toml() {
        let directory = tempdir().expect("a temporary directory");
        fs::write(directory.path().join("Cargo.toml"), "not = = toml")
            .expect("the invalid manifest is written");
        let error = workspace_root(directory.path())
            .err()
            .expect("an invalid manifest is reported");

        assert!(matches!(
            &error,
            CodegenError::ParseWorkspaceManifest { path, .. }
                if *path == directory.path().join("Cargo.toml")
        ));
        assert!(
            error
                .to_string()
                .contains("failed to parse the workspace manifest")
        );
    }
}
