use margaret_attributes::canonical_path::CanonicalPath;

#[must_use]
pub fn database_canonical_path() -> CanonicalPath {
    CanonicalPath::new(
        ["margaret", "framework", "database", "database", "Database"]
            .iter()
            .map(ToString::to_string)
            .collect(),
    )
}
