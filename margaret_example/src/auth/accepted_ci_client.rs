use std::collections::BTreeSet;

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
                client_id: "ci".parse()?,
                consent: ConsentPolicy::Implicit,
                grants: BTreeSet::from([GrantType::TokenExchange]),
                id_token_signing: IdTokenSigning::Rsa,
                introspection: IntrospectionPermission::Forbidden,
                redirect_uris: BTreeSet::new(),
                resources: BTreeSet::from([ATTACHMENTS_RESOURCE.parse()?]),
                scopes: BTreeSet::new(),
            },
        })
    }
}

impl DeclaresAcceptedClient for AcceptedCiClient {
    fn accepted_client(&self) -> &AcceptedClient {
        &self.accepted_client
    }
}
