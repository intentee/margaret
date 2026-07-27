use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

use crate::handler_binding::HandlerBinding;
use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) fn handler_console_arguments(
    handlers: &[HandlerBinding],
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleArgument>, WebSocketCodegenError> {
    let mut collected = Vec::new();

    for handler in handlers {
        collected.extend_from_slice(bindings.console_arguments(&handler.handler_path)?);
    }

    Ok(collected)
}
