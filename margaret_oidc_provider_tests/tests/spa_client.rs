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
use crate::spa_callback::SPA_CALLBACK;

/// # Panics
///
/// Panics when the fixture client identifier or callback is rejected.
#[must_use]
pub fn spa_client() -> AcceptedClient {
    AcceptedClient {
        authentication: AcceptedClientAuthentication::Public,
        client_id: "spa".parse().expect("the spa identifier is visible"),
        consent: ConsentPolicy::Prompted,
        grants: BTreeSet::from([GrantType::AuthorizationCode]),
        id_token_signing: IdTokenSigning::EllipticCurve,
        introspection: IntrospectionPermission::Forbidden,
        redirect_uris: BTreeSet::from([Url::parse(SPA_CALLBACK).expect("the callback is a url")]),
        resources: fixture_resources(&["artifacts", "reports"]),
        scopes: fixture_scopes(&["openid"]),
    }
}
