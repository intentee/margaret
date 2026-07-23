use std::path::Path;
use std::path::PathBuf;

pub(crate) struct WorkspaceLocation {
    embed_relative: String,
    root: PathBuf,
}

impl WorkspaceLocation {
    pub(crate) fn new(root: PathBuf, depth: usize) -> Self {
        let embed_relative = if depth == 0 {
            ".".to_string()
        } else {
            vec![".."; depth].join("/")
        };

        Self {
            embed_relative,
            root,
        }
    }

    pub(crate) fn embed_relative(&self) -> &str {
        &self.embed_relative
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::WorkspaceLocation;

    #[test]
    fn represents_a_manifest_that_is_its_own_workspace_root_as_the_current_directory() {
        let location = WorkspaceLocation::new(PathBuf::from("/workspace"), 0);

        assert_eq!(location.embed_relative(), ".");
        assert_eq!(location.root(), PathBuf::from("/workspace"));
    }

    #[test]
    fn represents_a_nested_manifest_with_one_parent_step_per_level() {
        let location = WorkspaceLocation::new(PathBuf::from("/workspace"), 2);

        assert_eq!(location.embed_relative(), "../..");
        assert_eq!(location.root(), PathBuf::from("/workspace"));
    }
}
