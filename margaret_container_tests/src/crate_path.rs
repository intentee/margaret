use margaret_attributes::canonical_path::CanonicalPath;

#[must_use]
pub fn crate_path(name: &str) -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
}
