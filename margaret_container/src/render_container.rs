use margaret_attributes::attribute_index::AttributeIndex;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::build_plan::build_plan;
use crate::container_error::ContainerError;
use crate::provided_singleton::ProvidedSingleton;
use crate::render::render;
use crate::render_build::render_build;
use crate::topological_order::topological_order;

pub fn render_container(
    index: &AttributeIndex,
    provided: &[ProvidedSingleton],
) -> Result<Vec<GeneratedModuleTokens>, ContainerError> {
    let plan = build_plan(index, provided)?;

    topological_order(&plan.providers, &plan.collections)?;

    Ok(vec![
        GeneratedModuleTokens::new("container", render(&plan)),
        GeneratedModuleTokens::new("container/build", render_build(&plan)),
    ])
}
