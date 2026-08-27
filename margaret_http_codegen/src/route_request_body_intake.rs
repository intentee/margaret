use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::request_binding::RequestBinding;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
use margaret_request_binding_codegen::request_body_intake::RequestBodyIntake;

/// # Errors
///
/// Returns `RequestBindingError::ConflictingRequestBodyIntake` when the responder, its middleware
/// and its authenticated user provider do not agree on how the body is read.
pub(crate) fn route_request_body_intake(
    arguments: &[BoundParameter],
    layers: &[LayerApplication],
    registries: &BindingRegistries,
    subject: &str,
) -> Result<RequestBodyIntake, RequestBindingError> {
    let mut intake = RequestBodyIntake::of(arguments);

    for layer in layers {
        intake = intake.combine(layer.body_intake, subject)?;
    }

    for argument in arguments {
        if let RequestBinding::AuthenticatedUser { application, .. } = &argument.binding
            && let Some(provider) = registries.authenticated_users.get(&application.model)
        {
            intake = intake.combine(RequestBodyIntake::of(&provider.parameters), subject)?;
        }
    }

    Ok(intake)
}
