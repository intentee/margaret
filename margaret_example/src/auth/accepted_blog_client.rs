use std::collections::BTreeSet;

use url::Url;

use margaret::framework::accepted_clients::accepted_client::AcceptedClient;
use margaret::framework::accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret::framework::accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret::framework::accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret::framework::accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret::framework::accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret::framework::accepted_clients::consent_policy::ConsentPolicy;
use margaret::framework::accepted_clients::declares_accepted_client::DeclaresAcceptedClient;
use margaret::framework::accepted_clients::introspection_permission::IntrospectionPermission;
use margaret::framework::accepted_clients::non_empty_set::NonEmptySet;
use margaret::framework::accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret::framework::accepted_clients::token_exchange_grant::TokenExchangeGrant;
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
                authentication: AcceptedClientAuthentication::ClientSecretBasic {
                    privileges: ConfidentialPrivileges {
                        client_credentials: ClientCredentialsGrant::Granted {
                            scopes: BTreeSet::from([PROFILE_SCOPE.parse()?]),
                        },
                        introspection: IntrospectionPermission::Permitted,
                    },
                    secret,
                },
                authorization_code: AuthorizationCodeGrant::Granted(CodeGrantPolicy {
                    consent: ConsentPolicy::Prompted,
                    id_token_signing: IdTokenSigning::Rsa,
                    redirect_uris: NonEmptySet::of(sign_in_callback, []),
                    refresh: RefreshTokenGrant::Withheld,
                    scopes: BTreeSet::from(["openid".parse()?, PROFILE_SCOPE.parse()?]),
                }),
                client_id: BLOG_CLIENT_ID.parse()?,
                resources: NonEmptySet::of(ATTACHMENTS_RESOURCE.parse()?, []),
                token_exchange: TokenExchangeGrant::Withheld,
            },
        })
    }
}

impl DeclaresAcceptedClient for AcceptedBlogClient {
    fn accepted_client(&self) -> &AcceptedClient {
        &self.accepted_client
    }
}
