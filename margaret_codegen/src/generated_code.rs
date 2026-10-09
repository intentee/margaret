use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use margaret_generated_module::generated_module::GeneratedModule;

use crate::codegen_error::CodegenError;

fn stale_entry_remains(path: &Path, written: &BTreeSet<PathBuf>) -> Result<bool, CodegenError> {
    if path.extension() == Some(OsStr::new("rs")) {
        if written.contains(path) {
            Ok(true)
        } else {
            fs::remove_file(path)
                .map(|()| false)
                .map_err(|source| CodegenError::RemoveEntry {
                    path: path.to_path_buf(),
                    source,
                })
        }
    } else if path.is_dir() {
        if remove_stale_sources(path, written)? {
            fs::remove_dir(path)
                .map(|()| false)
                .map_err(|source| CodegenError::RemoveEntry {
                    path: path.to_path_buf(),
                    source,
                })
        } else {
            Ok(true)
        }
    } else {
        Ok(true)
    }
}

fn remove_stale_sources(
    directory: &Path,
    written: &BTreeSet<PathBuf>,
) -> Result<bool, CodegenError> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(directory)
        .and_then(Iterator::collect)
        .map_err(|source| CodegenError::ReadDirectory {
            path: directory.to_path_buf(),
            source,
        })?;
    let remaining = entries
        .iter()
        .map(|entry| stale_entry_remains(&entry.path(), written))
        .collect::<Result<Vec<bool>, CodegenError>>()?;

    Ok(!remaining.contains(&true))
}

#[derive(Debug)]
pub struct GeneratedCode {
    modules: Vec<GeneratedModule>,
}

impl GeneratedCode {
    #[must_use]
    pub fn new(modules: Vec<GeneratedModule>) -> Self {
        Self { modules }
    }

    #[must_use]
    pub fn modules(&self) -> &[GeneratedModule] {
        &self.modules
    }

    /// # Errors
    ///
    /// Returns `CodegenError::CreateDirectory` or `CodegenError::WriteSource`.
    pub fn write_to(&self, directory: &Path) -> Result<(), CodegenError> {
        let mut written = BTreeSet::new();

        for module in &self.modules {
            let path = directory.join(format!("{}.rs", module.name()));
            let parent = path.parent().unwrap_or(directory);

            fs::create_dir_all(parent).map_err(|source| CodegenError::CreateDirectory {
                path: parent.to_path_buf(),
                source,
            })?;

            module
                .write_if_changed(&path)
                .map_err(|source| CodegenError::WriteSource {
                    path: path.clone(),
                    source,
                })?;
            written.insert(path);
        }

        remove_stale_sources(directory, &written).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;
    use std::fs::Permissions;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use tempfile::tempdir;

    use margaret_generated_module::generated_module::GeneratedModule;

    use super::GeneratedCode;
    use super::remove_stale_sources;
    use crate::codegen_error::CodegenError;

    #[test]
    fn writes_modules_into_nested_directories() {
        let directory = tempdir().expect("a temporary directory");
        let code = GeneratedCode::new(vec![
            GeneratedModule::new("container", "container"),
            GeneratedModule::new("container/build", "build"),
        ]);

        code.write_to(directory.path())
            .expect("the nested sources are written");

        assert_eq!(
            fs::read_to_string(directory.path().join("container/build.rs"))
                .expect("the nested module exists"),
            "build"
        );
    }

    #[test]
    fn removes_a_stale_source_and_recurses_into_directories() {
        let directory = tempdir().expect("a temporary directory");
        let nested = directory.path().join("http");
        fs::create_dir_all(&nested).expect("the nested directory exists");
        fs::write(directory.path().join("stale.rs"), "stale").expect("the stale source exists");
        fs::write(directory.path().join("notes.md"), "notes").expect("the foreign file exists");
        fs::write(nested.join("server_public.rs"), "stale").expect("the nested source exists");

        GeneratedCode::new(vec![GeneratedModule::new("kept", "kept")])
            .write_to(directory.path())
            .expect("the fresh source is written");

        assert!(!directory.path().join("stale.rs").exists());
        assert!(!nested.join("server_public.rs").exists());
        assert!(directory.path().join("kept.rs").exists());
        assert!(directory.path().join("notes.md").exists());
    }

    #[test]
    fn removes_directories_left_empty_by_stale_sources() {
        let directory = tempdir().expect("a temporary directory");
        let emptied = directory.path().join("trusted_issuers").join("auth");
        let occupied = directory.path().join("http");
        fs::create_dir_all(&emptied).expect("the emptied directory exists");
        fs::create_dir_all(&occupied).expect("the occupied directory exists");
        fs::write(emptied.join("trusted_issuer.rs"), "stale").expect("the stale source exists");
        fs::write(occupied.join("notes.md"), "notes").expect("the foreign file exists");

        GeneratedCode::new(vec![GeneratedModule::new("kept", "kept")])
            .write_to(directory.path())
            .expect("the fresh source is written");

        assert!(!directory.path().join("trusted_issuers").exists());
        assert!(occupied.join("notes.md").exists());
    }

    #[test]
    fn reports_an_emptied_directory_that_cannot_be_removed() {
        let directory = tempdir().expect("a temporary directory");
        let locked = directory.path().join("locked");
        fs::create_dir_all(locked.join("emptied")).expect("the emptied directory exists");
        fs::set_permissions(&locked, Permissions::from_mode(0o555))
            .expect("the parent directory is locked");

        let error = remove_stale_sources(directory.path(), &BTreeSet::new())
            .expect_err("removing a directory from a locked parent fails");

        fs::set_permissions(&locked, Permissions::from_mode(0o755))
            .expect("the parent directory is unlocked");

        assert!(matches!(
            error,
            CodegenError::RemoveEntry { path, .. } if path == locked.join("emptied")
        ));
    }

    #[test]
    fn reports_a_directory_that_cannot_be_created() {
        let directory = tempdir().expect("a temporary directory");
        let blocker = directory.path().join("blocker");
        fs::write(&blocker, "not a directory").expect("the blocking file exists");

        let error = GeneratedCode::new(vec![GeneratedModule::new("mod", "content")])
            .write_to(&blocker)
            .expect_err("writing beneath a file fails");

        assert!(
            error
                .to_string()
                .contains("failed to create the generated directory")
        );
    }

    #[test]
    fn reports_a_source_that_cannot_be_written() {
        let directory = tempdir().expect("a temporary directory");
        fs::create_dir(directory.path().join("container.rs"))
            .expect("the blocking directory exists");

        let error = GeneratedCode::new(vec![GeneratedModule::new("container", "content")])
            .write_to(directory.path())
            .expect_err("writing over a directory fails");

        assert!(
            error
                .to_string()
                .contains("failed to write the generated source")
        );
    }

    #[test]
    fn reports_a_directory_that_cannot_be_read() {
        let error = remove_stale_sources(Path::new("/does/not/exist"), &BTreeSet::new())
            .expect_err("reading a missing directory fails");

        assert!(
            error
                .to_string()
                .contains("failed to read the generated directory")
        );
    }

    #[test]
    fn reports_a_stale_entry_that_cannot_be_removed() {
        let directory = tempdir().expect("a temporary directory");
        fs::create_dir_all(directory.path().join("nested").join("stale.rs"))
            .expect("the undeletable nested stale entry exists");

        let error = remove_stale_sources(directory.path(), &BTreeSet::new())
            .expect_err("removing a directory as a file fails");

        assert!(
            error
                .to_string()
                .contains("failed to remove the stale generated entry")
        );
    }
}
