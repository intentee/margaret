use std::collections::HashSet;

use crate::canonical_path::CanonicalPath;

#[derive(Default)]
pub struct ItemPaths {
    paths: HashSet<CanonicalPath>,
}

impl ItemPaths {
    #[must_use]
    pub fn contains(&self, path: &CanonicalPath) -> bool {
        self.paths.contains(path)
    }

    pub(crate) fn insert(&mut self, path: CanonicalPath) {
        self.paths.insert(path);
    }
}

impl FromIterator<CanonicalPath> for ItemPaths {
    fn from_iter<Paths: IntoIterator<Item = CanonicalPath>>(paths: Paths) -> Self {
        Self {
            paths: paths.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::canonical_path::CanonicalPath;

    use super::ItemPaths;

    fn path(segment: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), segment.to_string()])
    }

    #[test]
    fn recognises_a_path_that_was_indexed() {
        let mut paths = ItemPaths::default();

        paths.insert(path("Indexed"));

        assert!(paths.contains(&path("Indexed")));
    }

    #[test]
    fn rejects_a_path_that_was_never_indexed() {
        assert!(!ItemPaths::default().contains(&path("Absent")));
    }
}
