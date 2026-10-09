use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;

pub const SERVICE_PRIVILEGES: ConfidentialPrivileges = ConfidentialPrivileges {
    client_credentials: ClientCredentialsGrant::Granted {
        scopes: &["artifacts:read"],
    },
    introspection: IntrospectionPermission::Forbidden,
};
