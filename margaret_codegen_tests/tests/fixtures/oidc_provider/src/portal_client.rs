use std::collections::BTreeSet;

use url::Url;

use margaret::framework::accepted_clients::accepted_client::AcceptedClient;
use margaret::framework::accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret::framework::accepted_clients::consent_policy::ConsentPolicy;
use margaret::framework::accepted_clients::declares_accepted_client::DeclaresAcceptedClient;
use margaret::framework::accepted_clients::grant_type::GrantType;
use margaret::framework::accepted_clients::introspection_permission::IntrospectionPermission;
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
                client_id: "portal".parse()?,
                consent: ConsentPolicy::Prompted,
                grants: BTreeSet::from([
                    GrantType::AuthorizationCode,
                    GrantType::RefreshToken,
                    GrantType::TokenExchange,
                ]),
                id_token_signing: IdTokenSigning::Rsa,
                introspection: IntrospectionPermission::Forbidden,
                redirect_uris: BTreeSet::from([Url::parse("https://portal.fixture/callback")?]),
                resources: BTreeSet::from(["artifacts".parse()?]),
                scopes: BTreeSet::from(["openid".parse()?]),
            },
        })
    }
}

impl DeclaresAcceptedClient for PortalClient {
    fn accepted_client(&self) -> &AcceptedClient {
        &self.accepted_client
    }
}
