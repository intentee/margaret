use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;

#[must_use]
pub fn console_argument_rejection(
    error: &ServeInputCodegenError,
) -> Option<&ConsoleArgumentCodegenError> {
    match error {
        ServeInputCodegenError::ConsoleArgument { source } => Some(source),
        _ => None,
    }
}
