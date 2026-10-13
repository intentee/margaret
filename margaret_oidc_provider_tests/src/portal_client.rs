use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

pub const PORTAL_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "portal",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Granted {
        scopes: &["profile"],
    },
};
