use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::build_plan::build_plan;
use crate::container_bindings::ContainerBindings;
use crate::container_error::ContainerError;
use crate::render::render;
use crate::render_build::render_build;
use crate::rendered_container::RenderedContainer;
use crate::topological_order::topological_order;

pub fn render_container(
    index: &AttributeIndex,
    framework_provided: &[CanonicalPath],
) -> Result<RenderedContainer, ContainerError> {
    let plan = build_plan(index, framework_provided)?;

    topological_order(&plan.providers, &plan.collections)?;

    let bindings = ContainerBindings::from_plan(&plan);

    Ok(RenderedContainer {
        bindings,
        modules: vec![
            GeneratedModuleTokens::new("container", render(&plan)),
            GeneratedModuleTokens::new("container/build", render_build(&plan)),
        ],
    })
}
