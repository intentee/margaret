use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_error::ContainerError;
use margaret_container::provided_singleton::ProvidedSingleton;
use margaret_container_tests::generate_container_source::generate_container_source_with_provided;
use quote::quote;

#[test]
fn reports_a_duplicate_provided_singleton() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provided_singleton");
    let path = CanonicalPath::new(vec!["baked_crate".to_string(), "BakedThing".to_string()]);
    let provided = [
        ProvidedSingleton {
            construction: quote! { baked_crate::BakedThing::first() },
            path: path.clone(),
        },
        ProvidedSingleton {
            construction: quote! { baked_crate::BakedThing::second() },
            path,
        },
    ];

    let error =
        generate_container_source_with_provided("provided_singleton", &directory, &provided)
            .expect_err("two provided singletons for the same type must be rejected");

    assert!(matches!(error, ContainerError::DuplicateProvider { .. }));
}
