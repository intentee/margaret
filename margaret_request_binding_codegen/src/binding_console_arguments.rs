use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::container_error::ContainerError;

use crate::request_binding::RequestBinding;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn binding_console_arguments(
    binding: &RequestBinding,
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleArgument>, ContainerError> {
    Ok(match binding {
        RequestBinding::AuthenticatedUser { application, .. } => bindings
            .console_arguments(&application.concrete)?
            .arguments
            .to_vec(),
        RequestBinding::BoundRouteParameter {
            binder_provider, ..
        } => bindings
            .console_arguments(binder_provider)?
            .arguments
            .to_vec(),
        RequestBinding::Injectable { dependency } => {
            bindings.injected_console_arguments(dependency)?
        }
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Next
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => Vec::new(),
    })
}
