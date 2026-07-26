use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn anyhow_result_path() -> CanonicalPath {
    CanonicalPath::new(vec!["anyhow".to_string(), "Result".to_string()])
}
