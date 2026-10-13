use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

#[test]
fn indexes_several_anonymous_imports_since_an_underscore_binds_no_name() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/anonymous_imports");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("anonymous_imports", &directory))
        .expect("anonymous imports bind no name, so they never collide")
        .build();

    assert!(
        index
            .item(&CanonicalPath::new(vec![
                "anonymous_imports".to_string(),
                "Holder".to_string(),
            ]))
            .is_some()
    );
}
