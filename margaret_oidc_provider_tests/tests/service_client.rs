use std::collections::BTreeSet;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::grant_type::GrantType;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

use crate::fixture_resources::fixture_resources;
use crate::fixture_scopes::fixture_scopes;
use crate::service_secret::SERVICE_SECRET;

/// # Panics
///
/// Panics when the fixture client identifier or secret is rejected.
#[must_use]
pub fn service_client() -> AcceptedClient {
    AcceptedClient {
        authentication: AcceptedClientAuthentication::ClientSecretBasic(
            SERVICE_SECRET
                .parse()
                .expect("the service secret is not empty"),
        ),
        client_id: "service"
            .parse()
            .expect("the service identifier is visible"),
        consent: ConsentPolicy::Implicit,
        grants: BTreeSet::from([GrantType::ClientCredentials]),
        id_token_signing: IdTokenSigning::Rsa,
        introspection: IntrospectionPermission::Forbidden,
        redirect_uris: BTreeSet::new(),
        resources: fixture_resources(&["artifacts"]),
        scopes: fixture_scopes(&["artifacts:read"]),
    }
}
