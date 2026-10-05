use crate::client_credentials_grant::ClientCredentialsGrant;
use crate::introspection_permission::IntrospectionPermission;

pub struct ConfidentialPrivileges {
    pub client_credentials: ClientCredentialsGrant,
    pub introspection: IntrospectionPermission,
}
