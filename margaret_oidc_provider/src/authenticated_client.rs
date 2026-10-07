use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_http::basic_challenged::basic_challenged;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::oauth_error::oauth_error;
use crate::provider_error::ProviderError;
use crate::temporarily_unavailable::temporarily_unavailable;

fn refused_client(refusal: &ClientRefusal) -> Response {
    let response = oauth_error(
        401,
        BasicErrorResponseType::InvalidClient,
        "the client could not be authenticated",
    );

    match refusal {
        ClientRefusal::HeaderAuthentication => basic_challenged(response),
        ClientRefusal::AssertionIdentifierMissing
        | ClientRefusal::AssertionMissing
        | ClientRefusal::AssertionOutlivesLimit { .. }
        | ClientRefusal::AssertionRefused(_)
        | ClientRefusal::AssertionRejected(_)
        | ClientRefusal::AssertionRequired
        | ClientRefusal::AssertionTypeMissing
        | ClientRefusal::ConflictingClientIds
        | ClientRefusal::MissingCredentials
        | ClientRefusal::PublicClientAssertion
        | ClientRefusal::SubjectMismatch { .. }
        | ClientRefusal::UnknownClient
        | ClientRefusal::UnsupportedAssertionType => response,
    }
}

pub(crate) async fn authenticated_client<'clients>(
    clients: &'clients AcceptedClients,
    request: &Request,
    parameters: &ClientAuthenticationParameters,
    state: &dyn StoresProviderState,
    now: NumericDate,
) -> Result<ControlFlow<Response, &'clients RegisteredClient>, ProviderError> {
    clients
        .authenticate(
            request.inputs.server.authorization(),
            parameters,
            state,
            now,
        )
        .await
        .map_err(ProviderError::State)
        .map(|outcome| match outcome {
            ClientAuthenticationOutcome::Authenticated(client) => ControlFlow::Continue(client),
            ClientAuthenticationOutcome::KeysAwaited => ControlFlow::Break(
                temporarily_unavailable("the signing keys of the client are not available yet"),
            ),
            ClientAuthenticationOutcome::Refused(refusal) => {
                ControlFlow::Break(refused_client(&refusal))
            }
        })
}
