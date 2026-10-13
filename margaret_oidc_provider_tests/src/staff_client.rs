use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

pub const STAFF_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "staff",
    resources: &[],
    token_exchange: TokenExchangeGrant::Granted {
        scopes: &["profile"],
    },
};
