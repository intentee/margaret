mod collection_table;
pub mod container_error;
mod container_plan;
mod dependency_kind;
mod find_constructor;
pub mod generated_source;
mod input_parameter;
mod parameter_plan;
mod path_text;
mod provided_type;
mod provider;
mod raw_target;
mod render;
mod resolution;
mod topological_order;
mod type_text;

use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::container_error::ContainerError;
use crate::container_plan::build_plan;
use crate::generated_source::GeneratedSource;
use crate::render::render;
use crate::topological_order::topological_order;

pub fn render_container(
    index: &AttributeIndex,
    input_selectors: &[AttributeSelector],
) -> Result<GeneratedSource, ContainerError> {
    let plan = build_plan(index, input_selectors)?;

    topological_order(&plan.providers, &plan.collections)?;

    Ok(GeneratedSource::new(render(&plan)))
}

pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
    input_selectors: &[AttributeSelector],
) -> Result<GeneratedSource, ContainerError> {
    let index = AttributeIndex::from_crate_root(crate_name, source_directory)?;

    render_container(&index, input_selectors)
}
