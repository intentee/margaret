mod collection_table;
mod construction_source;
pub mod container_error;
mod container_plan;
mod dependency_kind;
pub mod generated_source;
mod path_text;
mod provided_type;
mod provider;
mod provider_construction;
mod raw_target;
mod render;
mod resolution;
mod resolve_construction;
mod topological_order;
mod type_text;

use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

use crate::container_error::ContainerError;
use crate::container_plan::build_plan;
use crate::generated_source::GeneratedSource;
use crate::render::render;
use crate::topological_order::topological_order;

pub fn render_container(index: &AttributeIndex) -> Result<GeneratedSource, ContainerError> {
    let plan = build_plan(index)?;

    topological_order(&plan.providers, &plan.collections)?;

    Ok(GeneratedSource::new(render(&plan)))
}

pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedSource, ContainerError> {
    let index = AttributeIndex::from_crate_root(crate_name, source_directory)?;

    render_container(&index)
}
