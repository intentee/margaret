use crate::canonical_path::CanonicalPath;

#[must_use]
pub(crate) fn copy_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "std".to_string(),
        "marker".to_string(),
        "Copy".to_string(),
    ])
}
