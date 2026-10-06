use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;

use crate::fixture_resources::fixture_resources;

/// # Panics
///
/// Panics when the client identifier or secret of the RFC 6749 examples is rejected.
#[must_use]
pub fn rfc_example_client() -> AcceptedClient {
    AcceptedClient {
        authentication: AcceptedClientAuthentication::ClientSecretBasic {
            privileges: ConfidentialPrivileges {
                client_credentials: ClientCredentialsGrant::Withheld,
                introspection: IntrospectionPermission::Permitted,
            },
            secret: "gX1fBat3bV"
                .parse()
                .expect("the example secret is not empty"),
        },
        authorization_code: AuthorizationCodeGrant::Withheld,
        client_id: "s6BhdRkqt3"
            .parse()
            .expect("the example identifier is visible"),
        resources: fixture_resources("artifacts", &[]),
        token_exchange: TokenExchangeGrant::Withheld,
    }
}
