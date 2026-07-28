use margaret_attributes::canonical_path::CanonicalPath;

#[must_use]
pub fn cancellation_token_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "tokio_util".to_string(),
        "sync".to_string(),
        "CancellationToken".to_string(),
    ])
}
