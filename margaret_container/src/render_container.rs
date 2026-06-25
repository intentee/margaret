use margaret_attributes::attribute_index::AttributeIndex;

use crate::build_plan::build_plan;
use crate::container_error::ContainerError;
use crate::generated_source::GeneratedSource;
use crate::render::render;
use crate::topological_order::topological_order;

pub fn render_container(index: &AttributeIndex) -> Result<GeneratedSource, ContainerError> {
    let plan = build_plan(index)?;

    topological_order(&plan.providers, &plan.collections)?;

    Ok(GeneratedSource::new(render(&plan)))
}
