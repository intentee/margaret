use std::collections::BTreeSet;

use url::Url;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_accepted_clients::non_empty_set::NonEmptySet;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

use crate::fixture_resources::fixture_resources;
use crate::fixture_scopes::fixture_scopes;
use crate::portal_callback::PORTAL_CALLBACK;
use crate::portal_secret::PORTAL_SECRET;

/// # Panics
///
/// Panics when the fixture client identifier, secret, callback or scope is rejected.
#[must_use]
pub fn portal_client() -> AcceptedClient {
    AcceptedClient {
        authentication: AcceptedClientAuthentication::ClientSecretBasic {
            privileges: ConfidentialPrivileges {
                client_credentials: ClientCredentialsGrant::Granted {
                    scopes: BTreeSet::from(["profile"
                        .parse()
                        .expect("the scope grants a resource")]),
                },
                introspection: IntrospectionPermission::Permitted,
            },
            secret: PORTAL_SECRET
                .parse()
                .expect("the portal secret is not empty"),
        },
        authorization_code: AuthorizationCodeGrant::Granted(CodeGrantPolicy {
            consent: ConsentPolicy::Implicit,
            id_token_signing: IdTokenSigning::Rsa,
            redirect_uris: NonEmptySet::of(
                Url::parse(PORTAL_CALLBACK).expect("the callback is a url"),
                [],
            ),
            refresh: RefreshTokenGrant::Granted,
            scopes: fixture_scopes(&["openid", "profile"]),
        }),
        client_id: "portal".parse().expect("the portal identifier is visible"),
        resources: fixture_resources("artifacts", &[]),
        token_exchange: TokenExchangeGrant::Granted,
    }
}
