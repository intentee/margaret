use std::path::Path;

use margaret_container::generate_container_source::generate_container_source;

#[test]
fn generates_async_provider_factory() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/async_provider");
    let generated = generate_container_source("crate", &directory)
        .expect("the async provider fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("clock:tokio::sync::OnceCell<std::sync::Arc<dyncrate::Clock>>"));
    assert!(source.contains("pubasyncfnclock"));
    assert!(source.contains("letprovider=crate::ClockProvider;provider.provide().await"));
}
