use margaret::framework::macros::admits_oauth_client;
use margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;

#[admits_oauth_client(
    exchange_app,
    authentication = ClientAuthenticationMethod::None,
    client_id = "exchange",
    resources = [notes],
    token_exchange(scopes = []),
)]
pub struct AcceptedExchangeClient;
