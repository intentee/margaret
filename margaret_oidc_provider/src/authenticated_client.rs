use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::presented_client_credentials::PresentedClientCredentials;
use margaret_http::basic_challenged::basic_challenged;
use margaret_http::request::Request;
use margaret_http::response::Response;

use crate::oauth_error::oauth_error;

fn refused_client(presented: &PresentedClientCredentials) -> Response {
    let response = oauth_error(
        401,
        BasicErrorResponseType::InvalidClient,
        "the client could not be authenticated",
    );

    match presented {
        PresentedClientCredentials::Basic { .. }
        | PresentedClientCredentials::ConflictingClientIds
        | PresentedClientCredentials::Malformed => basic_challenged(response),
        PresentedClientCredentials::Absent | PresentedClientCredentials::ClientId(_) => response,
    }
}

pub(crate) fn authenticated_client<'clients>(
    clients: &'clients AcceptedClients,
    request: &Request,
    client_id: Option<&str>,
) -> ControlFlow<Response, &'clients AcceptedClient> {
    let presented =
        PresentedClientCredentials::of(request.inputs.server.authorization(), client_id);

    match clients.authenticate(&presented) {
        ClientAuthenticationOutcome::Authenticated(client) => ControlFlow::Continue(client),
        ClientAuthenticationOutcome::Refused(_) => ControlFlow::Break(refused_client(&presented)),
    }
}
