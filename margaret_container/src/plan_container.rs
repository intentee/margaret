use margaret_attributes::attribute_index::AttributeIndex;
use margaret_serve_input_codegen::declared_serve_inputs::DeclaredServeInputs;
use margaret_tag_codegen::tag_pool::TagPool;

use crate::build_plan::build_plan;
use crate::container_bindings::ContainerBindings;
use crate::container_error::ContainerError;
use crate::framework_provider::FrameworkProvider;
use crate::planned_container::PlannedContainer;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn plan_container(
    index: &AttributeIndex,
    serve_inputs: &DeclaredServeInputs,
    framework_providers: &[FrameworkProvider],
    tags: &TagPool,
) -> Result<PlannedContainer, ContainerError> {
    let plan = build_plan(index, serve_inputs, framework_providers, tags)?;
    let bindings = ContainerBindings::from_plan(&plan);

    Ok(PlannedContainer::new(bindings, plan))
}
