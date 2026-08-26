use margaret_container::container_bindings::ContainerBindings;
use margaret_serve_input_codegen::serve_input::ServeInput;

pub(crate) fn serve_inputs(bindings: &ContainerBindings) -> Vec<ServeInput> {
    bindings.all_serve_inputs().to_vec()
}
