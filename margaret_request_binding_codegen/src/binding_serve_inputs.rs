use margaret_container::container_bindings::ContainerBindings;
use margaret_container::container_error::ContainerError;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::request_binding::RequestBinding;

/// # Errors
///
/// Returns `ContainerError` propagated from the work it performs.
pub fn binding_serve_inputs(
    binding: &RequestBinding,
    bindings: &ContainerBindings,
) -> Result<Vec<ServeInput>, ContainerError> {
    Ok(match binding {
        RequestBinding::AuthenticatedUser { application, .. } => {
            let mut inputs = bindings
                .provider_serve_inputs(&application.concrete)?
                .inputs
                .to_vec();

            if let AuthenticatedUserChallenge::Bearer { issuer_client } = &application.challenge {
                inputs.extend(bindings.injected_serve_inputs(issuer_client)?);
            }

            inputs
        }
        RequestBinding::BoundRouteParameter {
            binder_provider, ..
        } => bindings
            .provider_serve_inputs(binder_provider)?
            .inputs
            .to_vec(),
        RequestBinding::Injectable { dependency } => bindings.injected_serve_inputs(dependency)?,
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Next
        | RequestBinding::BearerToken { .. }
        | RequestBinding::PeerSpiffeId
        | RequestBinding::RouteParameterValue { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => Vec::new(),
    })
}
