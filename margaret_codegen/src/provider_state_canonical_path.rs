use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn provider_state_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "provider_state_storage".to_string(),
        "stores_provider_state".to_string(),
        "StoresProviderState".to_string(),
    ])
}
