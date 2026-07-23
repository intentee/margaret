use std::path::Path;

use syn::Type;
use syn::parse_str;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_console_argument_codegen::scan::scan;
use margaret_container::injectable_resolution::InjectableResolution;
use margaret_container::render_container::render_container;
use margaret_container::resolve_injectable::resolve_injectable;

#[must_use]
pub fn resolve_full_fixture(consumer: &str, declared: &str) -> InjectableResolution {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("full", &directory))
        .expect("the full fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");
    let bindings = render_container(&index, &registry)
        .expect("the full fixture renders")
        .bindings;
    let item = index
        .items()
        .iter()
        .find(|item| item.identifier() == consumer)
        .expect("the consumer item is indexed");
    let declared: Type = parse_str(declared).expect("the declared type parses");

    resolve_injectable(&index, item, &declared, &bindings)
}
