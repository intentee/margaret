use serde::Deserialize;
use validator::Validate;

use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;

use crate::token_grant::TokenGrant;

#[derive(Debug, Deserialize, Validate)]
pub struct TokenRequest {
    #[serde(flatten)]
    pub client_authentication: ClientAuthenticationParameters,
    #[serde(flatten)]
    pub grant: TokenGrant,
}
