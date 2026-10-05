use std::collections::BTreeSet;

use url::Url;

use margaret::framework::accepted_clients::accepted_client::AcceptedClient;
use margaret::framework::accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret::framework::accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret::framework::accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret::framework::accepted_clients::consent_policy::ConsentPolicy;
use margaret::framework::accepted_clients::declares_accepted_client::DeclaresAcceptedClient;
use margaret::framework::accepted_clients::non_empty_set::NonEmptySet;
use margaret::framework::accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret::framework::accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret::framework::macros::accepts_oauth_client;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
#[accepts_oauth_client]
pub struct PortalClient {
    accepted_client: AcceptedClient,
}

impl PortalClient {
    /// # Errors
    ///
    /// Returns an error when the client identifier, callback, resource or scope is malformed.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self {
            accepted_client: AcceptedClient {
                authentication: AcceptedClientAuthentication::Public,
                authorization_code: AuthorizationCodeGrant::Granted(CodeGrantPolicy {
                    consent: ConsentPolicy::Prompted,
                    id_token_signing: IdTokenSigning::Rsa,
                    redirect_uris: NonEmptySet::of(
                        Url::parse("https://portal.fixture/callback")?,
                        [],
                    ),
                    refresh: RefreshTokenGrant::Granted,
                    scopes: BTreeSet::from(["openid".parse()?]),
                }),
                client_id: "portal".parse()?,
                resources: NonEmptySet::of("artifacts".parse()?, []),
                token_exchange: TokenExchangeGrant::Granted,
            },
        })
    }
}

impl DeclaresAcceptedClient for PortalClient {
    fn accepted_client(&self) -> &AcceptedClient {
        &self.accepted_client
    }
}
