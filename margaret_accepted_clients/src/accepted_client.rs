use crate::authorization_code_grant::AuthorizationCodeGrant;
use crate::token_exchange_grant::TokenExchangeGrant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcceptedClient {
    pub authorization_code: AuthorizationCodeGrant,
    pub client_id: &'static str,
    pub resources: &'static [&'static str],
    pub token_exchange: TokenExchangeGrant,
}
