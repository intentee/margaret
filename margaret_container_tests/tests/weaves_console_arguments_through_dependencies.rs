use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn weaves_console_arguments_through_dependencies() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");
    let source: String = generate_container_source("crate", &directory)
        .expect("the propagation crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(
        source.contains("pubfnconstruct_config(arguments:super::construct_config_arguments::ConstructConfigArguments,)")
    );
    assert!(source.contains("crate::Config::create(arguments.argument1)"));
    assert!(
        source.contains(
            "pubfnconstruct_alpha_plugin(arguments:super::construct_alpha_plugin_arguments::ConstructAlphaPluginArguments,)"
        )
    );
    assert!(
        source.contains(
            "pubfnconstruct_service(arguments:super::construct_service_arguments::ConstructServiceArguments,)"
        )
    );
    assert!(source.contains(
        "crate::Service::create(::std::sync::Arc::clone(&config),::std::sync::Arc::clone(&alpha_plugin),)"
    ));
}
