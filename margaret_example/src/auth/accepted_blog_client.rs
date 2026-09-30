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
use margaret::framework::oauth_vocabulary::client_secret::ClientSecret;

use crate::auth::attachments_resource::ATTACHMENTS_RESOURCE;
use crate::auth::blog_client_id::BLOG_CLIENT_ID;
use crate::auth::profile_scope::PROFILE_SCOPE;

#[singleton]
#[accepts_oauth_client]
pub struct AcceptedBlogClient {
    accepted_client: AcceptedClient,
}

impl AcceptedBlogClient {
    /// # Errors
    ///
    /// Returns an error when the client identifier, the resource or a scope is malformed.
    #[constructor]
    pub fn create(
        #[environment_variable(from = "MARGARET_EXAMPLE_BLOG_CLIENT_SECRET")] secret: ClientSecret,
        #[console_argument(from = "blog-sign-in-callback")] sign_in_callback: Url,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            accepted_client: AcceptedClient {
                authentication: AcceptedClientAuthentication::ClientSecretBasic(secret),
                client_id: BLOG_CLIENT_ID.parse()?,
                consent: ConsentPolicy::Prompted,
                grants: BTreeSet::from([
                    GrantType::AuthorizationCode,
                    GrantType::ClientCredentials,
                ]),
                id_token_signing: IdTokenSigning::Rsa,
                introspection: IntrospectionPermission::Permitted,
                redirect_uris: BTreeSet::from([sign_in_callback]),
                resources: BTreeSet::from([ATTACHMENTS_RESOURCE.parse()?]),
                scopes: BTreeSet::from(["openid".parse()?, PROFILE_SCOPE.parse()?]),
            },
        })
    }
}

impl DeclaresAcceptedClient for AcceptedBlogClient {
    fn accepted_client(&self) -> &AcceptedClient {
        &self.accepted_client
    }
}
