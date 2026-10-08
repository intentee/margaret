use margaret::framework::macros::admits_oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

#[admits_oauth_client(
    ci_app,
    authentication = ClientAuthenticationMethod::None,
    client_id = "ci",
    resources = [attachments],
    token_exchange(scopes = []),
)]
pub struct AcceptedCiClient;
