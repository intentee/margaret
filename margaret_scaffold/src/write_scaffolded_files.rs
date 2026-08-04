use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use tempfile::TempDir;

use crate::scaffold_error::ScaffoldError;
use crate::scaffolded_file::ScaffoldedFile;

fn staging_parent(target_directory: &Path) -> &Path {
    match target_directory.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    }
}

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

fn publish_scaffold(staging: &Path, target_directory: &Path) -> Result<(), ScaffoldError> {
    fs::rename(staging, target_directory).map_err(|source| ScaffoldError::PublishScaffold {
        path: target_directory.to_path_buf(),
        source,
    })
}

fn stage_scaffolded_files(
    target_directory: &Path,
    files: &[ScaffoldedFile],
) -> Result<TempDir, ScaffoldError> {
    let parent = staging_parent(target_directory);
    let staging = TempDir::new_in(parent).map_err(|source| ScaffoldError::StageScaffold {
        path: parent.to_path_buf(),
        source,
    })?;

    for file in files {
        let relative_directory = file.relative_path.parent().unwrap_or(Path::new(""));

        fs::create_dir_all(staging.path().join(relative_directory)).map_err(|source| {
            ScaffoldError::CreateDirectory {
                path: target_directory.join(relative_directory),
                source,
            }
        })?;
        fs::write(staging.path().join(&file.relative_path), &file.contents).map_err(|source| {
            ScaffoldError::WriteFile {
                path: target_directory.join(&file.relative_path),
                source,
            }
        })?;
    }

    Ok(staging)
}

pub(crate) fn write_scaffolded_files(
    target_directory: &Path,
    files: &[ScaffoldedFile],
) -> Result<(), ScaffoldError> {
    let staging = stage_scaffolded_files(target_directory, files)?;

    claim_target_directory(target_directory)?;

    publish_scaffold(staging.path(), target_directory)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::claim_target_directory;
    use super::publish_scaffold;
    use super::staging_parent;
    use super::write_scaffolded_files;
    use crate::scaffold_error::ScaffoldError;
    use crate::scaffolded_file::ScaffoldedFile;

    fn blocked_files() -> Vec<ScaffoldedFile> {
        vec![
            ScaffoldedFile {
                contents: String::new(),
                relative_path: PathBuf::from("src").join("lib.rs"),
            },
            ScaffoldedFile {
                contents: String::new(),
                relative_path: PathBuf::from("src"),
            },
        ]
    }

    #[test]
    fn writes_every_file_below_the_published_directory() {
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
    fn never_creates_the_target_when_a_file_cannot_be_written() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        let error = write_scaffolded_files(&target, &blocked_files())
            .expect_err("a blocked file is reported");

        assert!(matches!(
            &error,
            ScaffoldError::WriteFile { path, .. } if *path == target.join("src")
        ));
        assert!(!target.exists());
    }

    #[test]
    fn scaffolds_into_the_same_directory_after_an_earlier_attempt_failed() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        write_scaffolded_files(&target, &blocked_files()).expect_err("the first attempt fails");
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
    fn leaves_nothing_behind_in_the_directory_it_stages_into() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let target = workspace.path().join("acme");

        write_scaffolded_files(&target, &blocked_files()).expect_err("the attempt fails");

        assert_eq!(
            fs::read_dir(workspace.path())
                .expect("the workspace is readable")
                .count(),
            0
        );
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
    fn reports_a_directory_the_scaffold_cannot_be_staged_in() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let missing = workspace.path().join("missing");
        let error = write_scaffolded_files(&missing.join("acme"), &[])
            .expect_err("a missing staging parent is reported");

        assert!(matches!(
            &error,
            ScaffoldError::StageScaffold { path, .. } if *path == missing
        ));
    }

    #[test]
    fn reports_a_target_directory_that_cannot_be_claimed() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let blocker = workspace.path().join("blocker");

        fs::write(&blocker, "").expect("the blocking file is written");

        let target = blocker.join("acme");
        let error =
            claim_target_directory(&target).expect_err("a blocked target directory is reported");

        assert!(matches!(
            &error,
            ScaffoldError::CreateDirectory { path, .. } if *path == target
        ));
    }

    #[test]
    fn reports_a_scaffold_that_cannot_be_published() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let staging = workspace.path().join("staging");

        fs::create_dir(&staging).expect("the staging directory is created");

        let target = workspace.path().join("missing").join("acme");
        let error = publish_scaffold(&staging, &target).expect_err("a blocked move is reported");

        assert!(matches!(
            &error,
            ScaffoldError::PublishScaffold { path, .. } if *path == target
        ));
    }

    #[test]
    fn stages_next_to_the_target_directory() {
        assert_eq!(
            staging_parent(Path::new("/home/margaret/acme")),
            Path::new("/home/margaret")
        );
    }

    #[test]
    fn stages_in_the_working_directory_for_a_bare_project_name() {
        assert_eq!(staging_parent(Path::new("acme")), Path::new("."));
    }
}
