use std::path::Path;

use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

#[test]
fn awaits_a_root_whose_dependency_graph_is_asynchronous() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/async_pool");
    let bindings = bindings_for_fixture("crate", &directory);
    let invocation: String = bindings
        .construction_invocation("pool", &[])
        .to_string()
        .split_whitespace()
        .collect();

    assert_eq!(
        invocation,
        "super::container::build::construct_pool().await"
    );
}
