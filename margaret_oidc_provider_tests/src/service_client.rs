use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

pub const SERVICE_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "service",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Withheld,
};
