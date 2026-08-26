use margaret_container::container_bindings::ContainerBindings;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::handler_binding::HandlerBinding;
use crate::web_socket_codegen_error::WebSocketCodegenError;

pub(crate) fn handler_serve_inputs(
    handlers: &[HandlerBinding],
    bindings: &ContainerBindings,
) -> Result<Vec<ServeInput>, WebSocketCodegenError> {
    let mut collected = Vec::new();

    for handler in handlers {
        collected.extend_from_slice(
            &bindings
                .provider_serve_inputs(&handler.handler_path)?
                .inputs,
        );
    }

    Ok(collected)
}
