use std::path::Path;

use margaret_container::generate_container_source::generate_container_source;

#[test]
fn generates_provider_factory() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provider");
    let generated = generate_container_source("crate", &directory)
        .expect("the provider fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("greeter:std::sync::OnceLock<std::sync::Arc<dyncrate::Greeter>>"));
    assert!(
        source
            .contains("letprovider=crate::GreeterProvider::new(self.config());provider.provide()")
    );
}
