use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

pub const LEDGER_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "ledger",
    resources: &["artifacts", "reports"],
    token_exchange: TokenExchangeGrant::Withheld,
};
