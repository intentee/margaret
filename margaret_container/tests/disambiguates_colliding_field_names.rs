use std::path::Path;

use margaret_container::generate_container_source::generate_container_source;

#[test]
fn disambiguates_colliding_field_names() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/duplicate_field");
    let source: String = generate_container_source("duplicate_field", &directory)
        .expect("singletons whose field names collide are disambiguated, not rejected")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("pubasyncfnuser_account_detail(&self,)"));
    assert!(source.contains("pubasyncfnuser_account_detail_2(&self,)"));
}
