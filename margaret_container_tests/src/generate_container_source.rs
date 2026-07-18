use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_error::ContainerError;
use margaret_container::render_container::render_container;
use margaret_generated_module::generated_module::GeneratedModule;

pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedModule, ContainerError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))?
        .build();

    let source = render_container(&index)?
        .into_iter()
        .map(|module| module.format().source().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    Ok(GeneratedModule::new("container", source))
}
