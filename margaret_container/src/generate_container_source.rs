use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

use crate::container_error::ContainerError;
use crate::generated_source::GeneratedSource;
use crate::render_container::render_container;

pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedSource, ContainerError> {
    let index = AttributeIndex::from_crate_root(crate_name, source_directory)?;

    render_container(&index)
}
