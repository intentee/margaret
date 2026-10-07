use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

pub const SERVICE_CLIENT: AcceptedClient = AcceptedClient {
    authorization_code: AuthorizationCodeGrant::Withheld,
    client_id: "service",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Withheld,
};
