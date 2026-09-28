use margaret_attributes::attribute_index::AttributeIndex;
use margaret_serve_input_codegen::declared_serve_inputs::DeclaredServeInputs;
use margaret_tag_codegen::tag_pool::TagPool;

use crate::container_error::ContainerError;
use crate::framework_provider::FrameworkProvider;
use crate::plan_container::plan_container;
use crate::rendered_container::RenderedContainer;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn render_container(
    index: &AttributeIndex,
    serve_inputs: &DeclaredServeInputs,
    framework_providers: &[FrameworkProvider],
    tags: &TagPool,
) -> Result<RenderedContainer, ContainerError> {
    let planned = plan_container(index, serve_inputs, framework_providers, tags)?;
    let roots = planned.roots();

    planned.render(&roots, &roots, &roots)
}
