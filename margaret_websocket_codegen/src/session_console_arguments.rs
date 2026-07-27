use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_console_arguments::binding_console_arguments;

use crate::handler_binding::HandlerBinding;
use crate::session_plan::SessionPlan;
use crate::websocket_codegen_error::WebSocketCodegenError;

pub(crate) fn handler_console_arguments(
    handlers: &[HandlerBinding],
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleArgument>, WebSocketCodegenError> {
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for handler in handlers {
        collected.extend_from_slice(bindings.console_arguments(&handler.handler_path)?);
    }

    Ok(collected)
}

pub(crate) fn session_console_arguments(
    plan: &SessionPlan,
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleArgument>, WebSocketCodegenError> {
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for parameter in &plan.session.parameters {
        collected.extend(binding_console_arguments(&parameter.binding, bindings)?);
    }

    collected.extend(handler_console_arguments(&plan.request_handlers, bindings)?);
    collected.extend(handler_console_arguments(
        &plan.notification_handlers,
        bindings,
    )?);

    bindings.console_union(&collected).map_err(Into::into)
}
