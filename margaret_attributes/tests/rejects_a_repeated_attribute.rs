use std::path::Path;

use margaret_attributes::attribute_error::AttributeError;
use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

fn rejection(fixture: &str) -> AttributeError {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(fixture, &directory))
        .err()
        .expect("the repeated attribute is rejected")
}

#[test]
fn rejects_an_item_attribute_declared_twice() {
    assert!(matches!(
        rejection("repeated_item_attribute"),
        AttributeError::RepeatedAttribute { attribute_path, target }
            if attribute_path == "singleton" && target == "repeated_item_attribute::Twice"
    ));
}

#[test]
fn rejects_a_field_marker_declared_twice() {
    assert!(matches!(
        rejection("repeated_field_marker"),
        AttributeError::RepeatedAttribute { attribute_path, target }
            if attribute_path == "column" && target == "repeated_field_marker::Note::title"
    ));
}

#[test]
fn rejects_a_method_attribute_declared_twice() {
    assert!(matches!(
        rejection("repeated_method_attribute"),
        AttributeError::RepeatedAttribute { attribute_path, target }
            if attribute_path == "process" && target == "repeated_method_attribute::Twice::run"
    ));
}

#[test]
fn rejects_a_parameter_marker_declared_twice() {
    assert!(matches!(
        rejection("repeated_parameter_marker"),
        AttributeError::RepeatedAttribute { attribute_path, target }
            if attribute_path == "environment_variable"
                && target == "argument #0 of 'repeated_parameter_marker::Config::create'"
    ));
}
