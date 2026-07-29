use std::collections::HashMap;

use crate::canonical_path::CanonicalPath;

#[derive(Debug, Default)]
pub struct ModuleImports {
    by_name: HashMap<String, CanonicalPath>,
}

impl ModuleImports {
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<&CanonicalPath> {
        self.by_name.get(name)
    }

    pub(crate) fn insert(&mut self, name: String, path: CanonicalPath) {
        self.by_name.insert(name, path);
    }
}

#[cfg(test)]
mod tests {
    use crate::canonical_path::CanonicalPath;

    use super::ModuleImports;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(
            segments
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        )
    }

    #[test]
    fn resolves_an_inserted_name_to_its_path() {
        let mut imports = ModuleImports::default();
        imports.insert(
            "Routes".to_string(),
            path(&["crate", "margaret", "routes", "Routes"]),
        );

        assert_eq!(
            imports.resolve("Routes"),
            Some(&path(&["crate", "margaret", "routes", "Routes"]))
        );
    }

    #[test]
    fn returns_none_for_an_unknown_name() {
        let imports = ModuleImports::default();

        assert_eq!(imports.resolve("Routes"), None);
    }
}
