use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::crate_root::CrateRoot;

use crate::container_error::ContainerError;
use crate::generated_source::GeneratedSource;
use crate::render_container::render_container;

pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedSource, ContainerError> {
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new(crate_name, source_directory))?
        .build();

    render_container(&index)
}
