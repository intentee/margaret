use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;

fn fixture(name: &str) -> CrateRoot {
    CrateRoot::new(
        name,
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name),
    )
}

fn path(segments: &[&str]) -> CanonicalPath {
    CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
}

#[test]
fn finds_the_items_of_every_indexed_crate() {
    let index = AttributeIndexBuilder::new()
        .index_crate(&fixture("valid_crate"))
        .expect("the first crate is indexed")
        .index_crate(&fixture("second_crate"))
        .expect("the second crate is indexed")
        .build();

    assert!(
        index
            .item(&path(&["second_crate", "SecondStruct"]))
            .is_some()
    );
    assert!(index.item(&path(&["valid_crate", "RootStruct"])).is_some());
}
