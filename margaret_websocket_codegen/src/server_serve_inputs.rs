use margaret_container::container_bindings::ContainerBindings;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::session_plan::SessionPlan;
use crate::session_serve_inputs::session_serve_inputs;
use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) fn server_serve_inputs(
    sessions: &[&SessionPlan],
    bindings: &ContainerBindings,
) -> Result<Vec<ServeInput>, WebSocketCodegenError> {
    let mut collected = Vec::new();

    for session_plan in sessions {
        collected.extend(session_serve_inputs(session_plan, bindings)?);

        for layer in &session_plan.session.layers {
            collected.extend_from_slice(&bindings.provider_serve_inputs(&layer.concrete)?.inputs);
        }
    }

    bindings.serve_input_union(&collected).map_err(Into::into)
}
