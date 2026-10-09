use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn sessions_tables_canonical_path() -> CanonicalPath {
    CanonicalPath::new(
        [
            "margaret",
            "framework",
            "sessions",
            "margaret",
            "tables",
            "TABLES",
        ]
        .iter()
        .map(ToString::to_string)
        .collect(),
    )
}
