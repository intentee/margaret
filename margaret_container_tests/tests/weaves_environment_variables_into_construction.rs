use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

fn fixture_source(fixture: &str) -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    generate_container_source("crate", &directory)
        .expect("the fixture renders")
        .source()
        .split_whitespace()
        .collect()
}

#[test]
fn declares_each_environment_variable_as_a_bootstrap_argument_field() {
    let source = fixture_source("environment_variables");

    assert!(source.contains("pubargument0:std::string::String,"));
    assert!(source.contains("pubargument1:::std::option::Option<u16>,"));
    assert!(source.contains("pubargument2:bool,"));
    assert!(source.contains("pubargument3:std::path::PathBuf,"));
}

#[test]
fn weaves_an_environment_variable_into_a_root_that_only_depends_on_it() {
    let source = fixture_source("environment_variable_propagation");

    assert!(source.contains("crate::Config::create(serve_input_0)"));
    assert!(source.contains("crate::Service::create(::std::sync::Arc::clone(&config))"));
}
