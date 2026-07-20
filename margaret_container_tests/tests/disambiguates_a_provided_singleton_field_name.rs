use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::provided_singleton::ProvidedSingleton;
use margaret_container_tests::generate_container_source::generate_container_source_with_provided;
use quote::quote;

#[test]
fn disambiguates_a_provided_singleton_field_name() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provided_field_collision");
    let provided = [ProvidedSingleton {
        construction: quote! { baked_crate::BakedThing::assemble() },
        path: CanonicalPath::new(vec!["baked_crate".to_string(), "BakedThing".to_string()]),
    }];

    let source: String = generate_container_source_with_provided(
        "provided_field_collision",
        &directory,
        &provided,
    )
    .expect("a provided singleton whose field name collides is disambiguated")
    .source()
    .split_whitespace()
    .collect();

    assert!(source.contains("baked_thing:tokio::sync::OnceCell"));
    assert!(source.contains(
        "baked_thing_2:tokio::sync::OnceCell<std::sync::Arc<baked_crate::BakedThing>>"
    ));
}
