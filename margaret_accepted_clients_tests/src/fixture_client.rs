use std::collections::BTreeSet;

use url::Url;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::grant_type::GrantType;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

/// # Panics
///
/// Panics when the fixture client identifier, secret, callback, resource or scope is rejected.
#[must_use]
pub fn fixture_client(
    client_id: &str,
    authentication: AcceptedClientAuthentication,
) -> AcceptedClient {
    AcceptedClient {
        authentication,
        client_id: client_id.parse().expect("the client identifier is visible"),
        consent: ConsentPolicy::Prompted,
        grants: BTreeSet::from([GrantType::AuthorizationCode, GrantType::RefreshToken]),
        id_token_signing: IdTokenSigning::Rsa,
        introspection: IntrospectionPermission::Forbidden,
        redirect_uris: BTreeSet::from([
            Url::parse("https://client.example/callback").expect("the callback is a url")
        ]),
        resources: BTreeSet::from(["artifacts".parse().expect("the resource is an audience")]),
        scopes: BTreeSet::from(["openid".parse().expect("the scope is a scope token")]),
    }
}
