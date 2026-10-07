use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

pub const RFC_EXAMPLE_CLIENT: AcceptedClient = AcceptedClient {
    authorization_code: AuthorizationCodeGrant::Withheld,
    client_id: "s6BhdRkqt3",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Withheld,
};
