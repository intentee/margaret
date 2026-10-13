use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;

pub const RFC_EXAMPLE_PRIVILEGES: ConfidentialPrivileges = ConfidentialPrivileges {
    client_credentials: ClientCredentialsGrant::Withheld,
    introspection: IntrospectionPermission::Permitted,
};
