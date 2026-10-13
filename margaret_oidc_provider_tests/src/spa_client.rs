use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

pub const SPA_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "spa",
    resources: &["artifacts", "reports"],
    token_exchange: TokenExchangeGrant::Withheld,
};
