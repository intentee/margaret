use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::provided_singleton::ProvidedSingleton;
use margaret_container_tests::generate_container_source::generate_container_source_with_provided;
use quote::quote;

#[test]
fn injects_a_provided_singleton_into_a_constructor() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provided_dependency");
    let provided = [ProvidedSingleton {
        construction: quote! { baked_crate::BakedThing::assemble() },
        path: CanonicalPath::new(vec!["baked_crate".to_string(), "BakedThing".to_string()]),
    }];

    let source: String =
        generate_container_source_with_provided("provided_dependency", &directory, &provided)
            .expect("a constructor may depend on a provided singleton")
            .source()
            .split_whitespace()
            .collect();

    assert!(source.contains("provided_dependency::Consumer::new(self.baked_thing().await"));
}
