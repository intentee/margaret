use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;

/// # Panics
///
/// Panics when the fixture secret is rejected.
#[must_use]
pub fn confidential_authentication(secret: &str) -> AcceptedClientAuthentication {
    AcceptedClientAuthentication::ClientSecretBasic {
        privileges: ConfidentialPrivileges {
            client_credentials: ClientCredentialsGrant::Withheld,
            introspection: IntrospectionPermission::Forbidden,
        },
        secret: secret.parse().expect("the secret is visible"),
    }
}
