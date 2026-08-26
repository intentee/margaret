use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_serve_inputs::binding_serve_inputs;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::handler_serve_inputs::handler_serve_inputs;
use crate::session_plan::SessionPlan;
use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) fn session_serve_inputs(
    plan: &SessionPlan,
    bindings: &ContainerBindings,
) -> Result<Vec<ServeInput>, WebSocketCodegenError> {
    let mut collected: Vec<ServeInput> = Vec::new();

    for parameter in &plan.session.parameters {
        collected.extend(binding_serve_inputs(&parameter.binding, bindings)?);
    }

    collected.extend(handler_serve_inputs(&plan.request_handlers, bindings)?);
    collected.extend(handler_serve_inputs(&plan.notification_handlers, bindings)?);

    bindings.serve_input_union(&collected).map_err(Into::into)
}
