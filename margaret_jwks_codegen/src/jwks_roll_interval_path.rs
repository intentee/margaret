use margaret_attributes::canonical_path::CanonicalPath;

#[must_use]
pub fn jwks_roll_interval_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "jwks_roller_server".to_string(),
        "jwks_roll_interval".to_string(),
        "JWKS_ROLL_INTERVAL".to_string(),
    ])
}
