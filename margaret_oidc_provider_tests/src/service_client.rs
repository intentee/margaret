use std::collections::BTreeSet;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

use crate::fixture_resources::fixture_resources;
use crate::service_secret::SERVICE_SECRET;

/// # Panics
///
/// Panics when the fixture client identifier, secret or scope is rejected.
#[must_use]
pub fn service_client() -> AcceptedClient {
    AcceptedClient {
        authentication: AcceptedClientAuthentication::ClientSecretBasic {
            privileges: ConfidentialPrivileges {
                client_credentials: ClientCredentialsGrant::Granted {
                    scopes: BTreeSet::from(["artifacts:read"
                        .parse()
                        .expect("the scope grants a resource")]),
                },
                introspection: IntrospectionPermission::Forbidden,
            },
            secret: SERVICE_SECRET
                .parse()
                .expect("the service secret is not empty"),
        },
        authorization_code: AuthorizationCodeGrant::Withheld,
        client_id: "service"
            .parse()
            .expect("the service identifier is visible"),
        resources: fixture_resources("artifacts", &[]),
        token_exchange: TokenExchangeGrant::Withheld,
    }
}
