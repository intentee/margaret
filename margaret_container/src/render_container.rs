use margaret_attributes::attribute_index::AttributeIndex;
use margaret_generated_module::generated_module::GeneratedModule;

use crate::build_plan::build_plan;
use crate::container_error::ContainerError;
use crate::render::render;
use crate::render::render_build;
use crate::topological_order::topological_order;

pub fn render_container(index: &AttributeIndex) -> Result<Vec<GeneratedModule>, ContainerError> {
    let plan = build_plan(index)?;

    topological_order(&plan.providers, &plan.collections)?;

    Ok(vec![
        GeneratedModule::new("container", render(&plan)),
        GeneratedModule::new("container/build", render_build(&plan)),
    ])
}
