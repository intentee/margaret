use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

pub(crate) fn serve_arguments(bindings: &ContainerBindings) -> Vec<ConsoleArgument> {
    bindings.all_console_arguments().to_vec()
}
