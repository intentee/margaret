use margaret::framework::accepted_clients::accepted_client::AcceptedClient;
use margaret::framework::accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret::framework::accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret::framework::accepted_clients::declares_accepted_client::DeclaresAcceptedClient;
use margaret::framework::accepted_clients::non_empty_set::NonEmptySet;
use margaret::framework::accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret::framework::macros::accepts_oauth_client;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::auth::attachments_resource::ATTACHMENTS_RESOURCE;

#[singleton]
#[accepts_oauth_client]
pub struct AcceptedCiClient {
    accepted_client: AcceptedClient,
}

impl AcceptedCiClient {
    /// # Errors
    ///
    /// Returns an error when the client identifier or the resource is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            accepted_client: AcceptedClient {
                authentication: AcceptedClientAuthentication::Public,
                authorization_code: AuthorizationCodeGrant::Withheld,
                client_id: "ci".parse()?,
                resources: NonEmptySet::of(ATTACHMENTS_RESOURCE.parse()?, []),
                token_exchange: TokenExchangeGrant::Granted,
            },
        })
    }
}

impl DeclaresAcceptedClient for AcceptedCiClient {
    fn accepted_client(&self) -> &AcceptedClient {
        &self.accepted_client
    }
}
