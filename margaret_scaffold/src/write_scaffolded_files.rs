use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use crate::scaffold_error::ScaffoldError;
use crate::scaffolded_file::ScaffoldedFile;

fn claim_target_directory(target_directory: &Path) -> Result<(), ScaffoldError> {
    fs::create_dir(target_directory).map_err(|source| {
        if source.kind() == ErrorKind::AlreadyExists {
            ScaffoldError::TargetDirectoryExists {
                path: target_directory.to_path_buf(),
            }
        } else {
            ScaffoldError::CreateDirectory {
                path: target_directory.to_path_buf(),
                source,
            }
        }
    })
}

pub(crate) fn write_scaffolded_files(
    target_directory: &Path,
    files: &[ScaffoldedFile],
) -> Result<(), ScaffoldError> {
    claim_target_directory(target_directory)?;

    for file in files {
        let path = target_directory.join(&file.relative_path);
        let parent = path.parent().unwrap_or(target_directory);

        fs::create_dir_all(parent).map_err(|source| ScaffoldError::CreateDirectory {
            path: parent.to_path_buf(),
            source,
        })?;
        fs::write(&path, &file.contents).map_err(|source| ScaffoldError::WriteFile {
            path: path.clone(),
            source,
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::write_scaffolded_files;
    use crate::scaffold_error::ScaffoldError;
    use crate::scaffolded_file::ScaffoldedFile;

    #[test]
    fn writes_every_file_below_the_claimed_directory() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        write_scaffolded_files(
            &target,
            &[
                ScaffoldedFile {
                    contents: "[workspace]\n".to_string(),
                    relative_path: PathBuf::from("Cargo.toml"),
                },
                ScaffoldedFile {
                    contents: "pub mod routes;\n".to_string(),
                    relative_path: PathBuf::from("acme_base").join("src").join("lib.rs"),
                },
            ],
        )
        .expect("the files are written");

        assert_eq!(
            fs::read_to_string(target.join("Cargo.toml")).expect("the manifest is readable"),
            "[workspace]\n"
        );
        assert_eq!(
            fs::read_to_string(target.join("acme_base").join("src").join("lib.rs"))
                .expect("the library root is readable"),
            "pub mod routes;\n"
        );
    }

    #[test]
    fn refuses_to_write_into_a_directory_that_already_exists() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        fs::create_dir(&target).expect("the target directory is created up front");

        let error = write_scaffolded_files(&target, &[])
            .expect_err("an existing target directory is reported");

        assert!(matches!(
            &error,
            ScaffoldError::TargetDirectoryExists { path } if *path == target
        ));
    }

    #[test]
    fn reports_a_target_directory_that_cannot_be_claimed() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let blocker = workspace.path().join("blocker");

        fs::write(&blocker, "").expect("the blocking file is written");

        let target = blocker.join("acme");
        let error = write_scaffolded_files(&target, &[]).expect_err("a blocked target is reported");

        assert!(matches!(
            &error,
            ScaffoldError::CreateDirectory { path, .. } if *path == target
        ));
    }

    #[test]
    fn reports_a_nested_directory_that_cannot_be_created() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        let error = write_scaffolded_files(
            &target,
            &[
                ScaffoldedFile {
                    contents: String::new(),
                    relative_path: PathBuf::from("src"),
                },
                ScaffoldedFile {
                    contents: String::new(),
                    relative_path: PathBuf::from("src").join("lib.rs"),
                },
            ],
        )
        .expect_err("a blocked nested directory is reported");

        assert!(matches!(
            &error,
            ScaffoldError::CreateDirectory { path, .. } if *path == target.join("src")
        ));
    }

    #[test]
    fn reports_a_file_that_cannot_be_written() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        let error = write_scaffolded_files(
            &target,
            &[
                ScaffoldedFile {
                    contents: String::new(),
                    relative_path: PathBuf::from("src").join("lib.rs"),
                },
                ScaffoldedFile {
                    contents: String::new(),
                    relative_path: PathBuf::from("src"),
                },
            ],
        )
        .expect_err("a blocked file is reported");

        assert!(matches!(
            &error,
            ScaffoldError::WriteFile { path, .. } if *path == target.join("src")
        ));
    }
}
