use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

use crate::session_console_arguments::session_console_arguments;
use crate::session_plan::SessionPlan;
use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) fn server_console_arguments(
    sessions: &[&SessionPlan],
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleArgument>, WebSocketCodegenError> {
    let mut collected = Vec::new();

    for session_plan in sessions {
        collected.extend(session_console_arguments(session_plan, bindings)?);

        for layer in &session_plan.session.layers {
            collected.extend_from_slice(bindings.console_arguments(&layer.concrete)?);
        }
    }

    bindings.console_union(&collected).map_err(Into::into)
}
