use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_error::ContainerError;
use margaret_container::provided_singleton::ProvidedSingleton;
use margaret_container::render_container::render_container;
use margaret_generated_module::generated_module::GeneratedModule;

pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedModule, ContainerError> {
    generate_container_source_with_provided(crate_name, source_directory, &[])
}

pub fn generate_container_source_with_provided(
    crate_name: &str,
    source_directory: &Path,
    provided: &[ProvidedSingleton],
) -> Result<GeneratedModule, ContainerError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))?
        .build();

    let source = render_container(&index, provided)?
        .modules
        .into_iter()
        .map(|module| {
            module
                .format()
                .expect("the module formats")
                .source()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(GeneratedModule::new("container", source))
}
