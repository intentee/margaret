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

fn discard_partial_scaffold(
    target_directory: &Path,
    scaffold_failure: ScaffoldError,
) -> ScaffoldError {
    match fs::remove_dir_all(target_directory) {
        Ok(()) => scaffold_failure,
        Err(source) => ScaffoldError::DiscardPartialScaffold {
            path: target_directory.to_path_buf(),
            scaffold_failure: Box::new(scaffold_failure),
            source,
        },
    }
}

fn fill_claimed_directory(
    target_directory: &Path,
    files: &[ScaffoldedFile],
) -> Result<(), ScaffoldError> {
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

pub(crate) fn write_scaffolded_files(
    target_directory: &Path,
    files: &[ScaffoldedFile],
) -> Result<(), ScaffoldError> {
    claim_target_directory(target_directory)?;

    fill_claimed_directory(target_directory, files)
        .map_err(|scaffold_failure| discard_partial_scaffold(target_directory, scaffold_failure))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::discard_partial_scaffold;
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

    #[test]
    fn leaves_no_directory_behind_when_a_file_cannot_be_written() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");
        let blocked = [
            ScaffoldedFile {
                contents: String::new(),
                relative_path: PathBuf::from("src").join("lib.rs"),
            },
            ScaffoldedFile {
                contents: String::new(),
                relative_path: PathBuf::from("src"),
            },
        ];

        write_scaffolded_files(&target, &blocked).expect_err("a blocked file is reported");

        assert!(!target.exists());
    }

    #[test]
    fn scaffolds_into_the_same_directory_after_an_earlier_attempt_failed() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        write_scaffolded_files(
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
        .expect_err("the first attempt fails");

        write_scaffolded_files(
            &target,
            &[ScaffoldedFile {
                contents: "[workspace]\n".to_string(),
                relative_path: PathBuf::from("Cargo.toml"),
            }],
        )
        .expect("the retry succeeds");

        assert_eq!(
            fs::read_to_string(target.join("Cargo.toml")).expect("the manifest is readable"),
            "[workspace]\n"
        );
    }

    #[test]
    fn keeps_the_original_failure_when_the_partial_scaffold_cannot_be_removed() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let removed = workspace.path().join("already-gone");
        let error = discard_partial_scaffold(
            &removed,
            ScaffoldError::TargetDirectoryExists {
                path: removed.clone(),
            },
        );

        let reported = error.to_string();

        assert!(matches!(
            &error,
            ScaffoldError::DiscardPartialScaffold { path, .. } if *path == removed
        ));
        assert!(reported.contains("failed to remove the partially scaffolded directory"));
        assert!(reported.contains("the scaffolded directory"));
    }
}
