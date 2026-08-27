use margaret_container::container_error::ContainerError;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;

#[must_use]
pub fn serve_input_rejection(error: &ContainerError) -> Option<&ServeInputCodegenError> {
    match error {
        ContainerError::ServeInput { source } => Some(source),
        _ => None,
    }
}
