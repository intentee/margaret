use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

#[must_use]
pub fn fixture_client(client_id: &'static str) -> AcceptedClient {
    AcceptedClient {
        client_id,
        resources: &["artifacts"],
        token_exchange: TokenExchangeGrant::Withheld,
    }
}
