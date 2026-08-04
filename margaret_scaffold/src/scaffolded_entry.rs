use std::fs;
use std::io::Result as IoResult;
use std::path::PathBuf;

pub(crate) enum ScaffoldedEntry {
    Directory(PathBuf),
    File(PathBuf),
}

impl ScaffoldedEntry {
    pub(crate) fn remove(&self) -> IoResult<()> {
        match self {
            Self::Directory(path) => fs::remove_dir(path),
            Self::File(path) => fs::remove_file(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::ScaffoldedEntry;

    #[test]
    fn removes_a_file_it_scaffolded() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let path = workspace.path().join("Cargo.toml");

        fs::write(&path, "[workspace]\n").expect("the file is written");

        ScaffoldedEntry::File(path.clone())
            .remove()
            .expect("the file is removed");

        assert!(!path.exists());
    }

    #[test]
    fn removes_an_empty_directory_it_scaffolded() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let path = workspace.path().join("src");

        fs::create_dir(&path).expect("the directory is created");

        ScaffoldedEntry::Directory(path.clone())
            .remove()
            .expect("the directory is removed");

        assert!(!path.exists());
    }

    #[test]
    fn refuses_to_remove_a_directory_that_holds_anything_else() {
        let workspace = tempdir().expect("a temporary workspace directory");
        let path = workspace.path().join("src");

        fs::create_dir(&path).expect("the directory is created");
        fs::write(path.join("foreign.rs"), "").expect("the foreign file is written");

        ScaffoldedEntry::Directory(path.clone())
            .remove()
            .expect_err("a directory that is not empty is kept");

        assert!(path.join("foreign.rs").is_file());
    }
}
