use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::provided_singleton::ProvidedSingleton;
use margaret_container_tests::generate_container_source::generate_container_source_with_provided;
use quote::quote;

#[test]
fn bakes_a_provided_singleton() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provided_singleton");
    let provided = [ProvidedSingleton {
        construction: quote! { baked_crate::BakedThing::assemble() },
        path: CanonicalPath::new(vec!["baked_crate".to_string(), "BakedThing".to_string()]),
    }];

    let source: String =
        generate_container_source_with_provided("provided_singleton", &directory, &provided)
            .expect("a provided singleton is baked into the container")
            .source()
            .split_whitespace()
            .collect();

    assert!(source.contains(
        "baked_thing:tokio::sync::OnceCell<std::sync::Arc<baked_crate::BakedThing>>"
    ));
    assert!(source.contains("pubasyncfnbaked_thing"));
    assert!(source.contains("std::sync::Arc::new(baked_crate::BakedThing::assemble()"));
}
