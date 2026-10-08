use margaret_attributes::attribute_index::AttributeIndex;
use margaret_serve_input_codegen::declared_serve_inputs::DeclaredServeInputs;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

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
    token_issuance: &DeclaredTokenIssuance,
) -> Result<RenderedContainer, ContainerError> {
    let planned = plan_container(index, serve_inputs, framework_providers, token_issuance)?;
    let roots = planned.roots();

    planned.render(&roots, &roots)
}
