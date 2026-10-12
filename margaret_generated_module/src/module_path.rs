use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct ModulePath {
    ancestors: Vec<String>,
    name: String,
}

impl ModulePath {
    pub fn root(name: impl Into<String>) -> Self {
        Self {
            ancestors: Vec::new(),
            name: name.into(),
        }
    }

    #[must_use]
    pub fn child(&self, name: impl Into<String>) -> Self {
        let mut ancestors = self.ancestors.clone();

        ancestors.push(self.name.clone());

        Self {
            ancestors,
            name: name.into(),
        }
    }

    #[must_use]
    pub fn directory_in(&self, root: &Path) -> PathBuf {
        root.join(self.ancestors.iter().collect::<PathBuf>())
    }

    #[must_use]
    pub fn file_name(&self) -> String {
        format!("{}.rs", self.name)
    }
}

impl Display for ModulePath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let segments: Vec<&str> = self
            .ancestors
            .iter()
            .map(String::as_str)
            .chain([self.name.as_str()])
            .collect();

        formatter.write_str(&segments.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ModulePath;

    #[test]
    fn places_a_root_module_directly_in_the_generated_directory() {
        assert_eq!(
            ModulePath::root("container").directory_in(Path::new("generated")),
            Path::new("generated")
        );
    }

    #[test]
    fn places_a_nested_module_in_the_directory_of_its_ancestors_where_rust_looks_for_it() {
        assert_eq!(
            ModulePath::root("routes")
                .child("servers")
                .child("public")
                .directory_in(Path::new("generated")),
            Path::new("generated/routes/servers")
        );
    }

    #[test]
    fn names_the_module_file_after_the_module_itself() {
        assert_eq!(
            ModulePath::root("routes").child("servers").file_name(),
            "servers.rs"
        );
    }

    #[test]
    fn displays_the_module_path_with_each_segment_separated_by_a_slash() {
        assert_eq!(
            ModulePath::root("routes")
                .child("servers")
                .child("public")
                .to_string(),
            "routes/servers/public"
        );
    }
}
