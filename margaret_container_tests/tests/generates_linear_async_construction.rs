use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn generates_linear_async_construction() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/async_pool");
    let generated = generate_container_source("crate", &directory)
        .expect("the async fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("pool:::std::sync::Arc<crate::Pool>"));
    assert!(source.contains("config:::std::sync::Arc<crate::Config>"));
    assert!(source.contains("pub(crate)asyncfnconstruct_pool"));
    assert!(source.contains("crate::Pool::new(::std::sync::Arc::clone(&config)).await"));
    assert!(source.contains("crate::Config::new()"));
    assert!(!source.contains("crate::Config::new().await"));
}
