use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_attributes::is_copy_type::is_copy_type;

fn index() -> AttributeIndex {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/copy_types");

    AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", &directory))
        .expect("the copy fixture indexes cleanly")
        .build()
}

fn is_copy(segments: &[&str]) -> bool {
    let canonical = CanonicalPath::new(
        segments
            .iter()
            .map(std::string::ToString::to_string)
            .collect(),
    );

    is_copy_type(&index(), &canonical)
}

#[test]
fn detects_a_struct_deriving_copy() {
    assert!(is_copy(&["crate", "DerivedCopy"]));
}

#[test]
fn detects_an_enum_deriving_copy() {
    assert!(is_copy(&["crate", "DerivedCopyEnum"]));
}

#[test]
fn detects_a_manually_implemented_copy() {
    assert!(is_copy(&["crate", "ManuallyCopied"]));
}

#[test]
fn rejects_a_type_that_only_derives_clone() {
    assert!(!is_copy(&["crate", "ClonedOnly"]));
}

#[test]
fn detects_a_copy_primitive() {
    assert!(is_copy(&["u16"]));
}

#[test]
fn detects_a_standard_library_copy_type() {
    assert!(is_copy(&["std", "num", "NonZeroU32"]));
    assert!(is_copy(&["std", "net", "SocketAddr"]));
}

#[test]
fn rejects_a_standard_library_type_that_is_not_copy() {
    assert!(!is_copy(&["std", "string", "String"]));
}

#[test]
fn rejects_a_type_outside_the_indexed_crate() {
    assert!(!is_copy(&["uuid", "Uuid"]));
}
