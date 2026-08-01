use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn provides_expected_claims_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "jwt_claims".to_string(),
        "provides_expected_claims".to_string(),
        "ProvidesExpectedClaims".to_string(),
    ])
}
