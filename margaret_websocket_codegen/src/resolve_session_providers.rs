use margaret_container::container_bindings::ContainerBindings;

use crate::session_plan::SessionPlan;
use crate::session_serve_inputs::session_serve_inputs;
use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) fn resolve_session_providers(
    session_plan: &SessionPlan,
    bindings: &ContainerBindings,
) -> Result<(), WebSocketCodegenError> {
    session_serve_inputs(session_plan, bindings)?;

    for layer in &session_plan.session.layers {
        bindings.provider_serve_inputs(&layer.concrete)?;
    }

    Ok(())
}
