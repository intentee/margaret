use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_console_argument_codegen::scan::scan;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::render_container::render_container;

#[must_use]
pub fn bindings_for_fixture(crate_name: &str, source_directory: &Path) -> ContainerBindings {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");

    render_container(&index, &registry)
        .expect("the fixture renders")
        .bindings
}
