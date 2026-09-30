use std::collections::BTreeSet;

use url::Url;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::grant_type::GrantType;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

use crate::fixture_resources::fixture_resources;
use crate::fixture_scopes::fixture_scopes;
use crate::portal_callback::PORTAL_CALLBACK;
use crate::portal_secret::PORTAL_SECRET;

/// # Panics
///
/// Panics when the fixture client identifier, secret or callback is rejected.
#[must_use]
pub fn portal_client() -> AcceptedClient {
    AcceptedClient {
        authentication: AcceptedClientAuthentication::ClientSecretBasic(
            PORTAL_SECRET
                .parse()
                .expect("the portal secret is not empty"),
        ),
        client_id: "portal".parse().expect("the portal identifier is visible"),
        consent: ConsentPolicy::Implicit,
        grants: BTreeSet::from([
            GrantType::AuthorizationCode,
            GrantType::ClientCredentials,
            GrantType::RefreshToken,
            GrantType::TokenExchange,
        ]),
        id_token_signing: IdTokenSigning::Rsa,
        introspection: IntrospectionPermission::Permitted,
        redirect_uris: BTreeSet::from(
            [Url::parse(PORTAL_CALLBACK).expect("the callback is a url")],
        ),
        resources: fixture_resources(&["artifacts"]),
        scopes: fixture_scopes(&["openid", "profile"]),
    }
}
