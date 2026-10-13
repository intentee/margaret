use std::sync::LazyLock;

use uuid::Uuid;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::assertion_signing::AssertionSigning;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::introspection_permission::IntrospectionPermission;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_accepted_clients::token_exchange_grant::TokenExchangeGrant;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oidc_provider::endpoint_authentication::EndpointAuthentication;
use margaret_oidc_provider::provider_support::ProviderSupport;

use crate::client_credentials::ClientCredentials;
use crate::fixture_issuer::FixtureIssuer;
use crate::provider_issuance::provider_issuance;
use crate::provider_route_paths::ProviderRoutePaths;

const PORTAL_AUTHENTICATION: EndpointAuthentication = EndpointAuthentication {
    methods: &[ClientAuthenticationMethod::PrivateKeyJwt],
    signing: &[AssertionSigning::Pinned(JwsAlgorithm::Es256)],
};

pub const CI_SUBJECT: Uuid = Uuid::from_u128(0x0192_c0de_0000_7000_8000_0000_0000_00c1);

pub const END_USER_SUBJECT: Uuid = Uuid::from_u128(0x0192_c0de_0000_7000_8000_0000_0000_0007);

pub const FIXTURE_ACCEPTED_RESOURCES: &[&str] = &["artifacts", "reports"];

pub const FIXTURE_ENDPOINT_PATHS: ProviderRoutePaths = ProviderRoutePaths {
    authorization: "/authorize",
    discovery: "/.well-known/openid-configuration",
    introspection: "/introspect",
    jwks: "/jwks.json",
    revocation: "/revoke",
    token: "/token",
    userinfo: "/userinfo",
};

pub const FIXTURE_PROVIDER_SUPPORT: ProviderSupport = ProviderSupport {
    grant_types: &GrantType::ALL,
    id_token_signing: &[IdTokenSigning::EllipticCurve, IdTokenSigning::Rsa],
    introspection: PORTAL_AUTHENTICATION,
    revocation: PORTAL_AUTHENTICATION,
    scopes: &["artifacts:read", "openid", "profile"],
    token: EndpointAuthentication {
        methods: &[
            ClientAuthenticationMethod::None,
            ClientAuthenticationMethod::PrivateKeyJwt,
        ],
        signing: &[AssertionSigning::Pinned(JwsAlgorithm::Es256)],
    },
};

pub const LEDGER_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "ledger",
    resources: &["artifacts", "reports"],
    token_exchange: TokenExchangeGrant::Withheld,
};

pub const LEDGER_CREDENTIALS: ClientCredentials = ClientCredentials::Asserted {
    client_id: "ledger",
};

pub const LEDGER_PRIVILEGES: ConfidentialPrivileges = ConfidentialPrivileges {
    client_credentials: ClientCredentialsGrant::Granted {
        scopes: &["artifacts:read"],
    },
    introspection: IntrospectionPermission::Forbidden,
};

pub static LOCALHOST_ISSUER: LazyLock<FixtureIssuer> = LazyLock::new(|| {
    FixtureIssuer::located(
        provider_issuance()
            .issuer
            .parse()
            .expect("the fixture issuer is an https url"),
    )
});

pub const PKCE_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";

pub const PORTAL_CALLBACK: &str = "https://portal.localhost/callback";

pub const PORTAL_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "portal",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Granted {
        scopes: &["profile"],
    },
};

pub const PORTAL_CODE_GRANT: CodeGrantPolicy = CodeGrantPolicy {
    consent: ConsentPolicy::Implicit,
    id_token_signing: IdTokenSigning::Rsa,
    refresh: RefreshTokenGrant::Granted,
    scopes: &["openid", "profile"],
};

pub const PORTAL_CREDENTIALS: ClientCredentials = ClientCredentials::Asserted {
    client_id: "portal",
};

pub const PORTAL_PRIVILEGES: ConfidentialPrivileges = ConfidentialPrivileges {
    client_credentials: ClientCredentialsGrant::Granted {
        scopes: &["profile"],
    },
    introspection: IntrospectionPermission::Permitted,
};

pub const PROVIDER_SESSION_AUDIENCE: &str = "provider-browser";

pub const RFC_EXAMPLE_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "s6BhdRkqt3",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Withheld,
};

pub const RFC_EXAMPLE_PRIVILEGES: ConfidentialPrivileges = ConfidentialPrivileges {
    client_credentials: ClientCredentialsGrant::Withheld,
    introspection: IntrospectionPermission::Permitted,
};

pub const SERVICE_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "service",
    resources: &["artifacts"],
    token_exchange: TokenExchangeGrant::Withheld,
};

pub const SERVICE_CREDENTIALS: ClientCredentials = ClientCredentials::Asserted {
    client_id: "service",
};

pub const SERVICE_PRIVILEGES: ConfidentialPrivileges = ConfidentialPrivileges {
    client_credentials: ClientCredentialsGrant::Granted {
        scopes: &["artifacts:read"],
    },
    introspection: IntrospectionPermission::Forbidden,
};

pub const SPA_CALLBACK: &str = "https://spa.localhost/callback";

pub const SPA_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "spa",
    resources: &["artifacts", "reports"],
    token_exchange: TokenExchangeGrant::Withheld,
};

pub const SPA_CODE_GRANT: CodeGrantPolicy = CodeGrantPolicy {
    consent: ConsentPolicy::Prompted,
    id_token_signing: IdTokenSigning::EllipticCurve,
    refresh: RefreshTokenGrant::Withheld,
    scopes: &["openid"],
};

pub const STAFF_CALLBACK: &str = "https://staff.localhost/callback";

pub const STAFF_CLIENT: AcceptedClient = AcceptedClient {
    client_id: "staff",
    resources: &[],
    token_exchange: TokenExchangeGrant::Granted {
        scopes: &["profile"],
    },
};

pub const STAFF_CODE_GRANT: CodeGrantPolicy = CodeGrantPolicy {
    consent: ConsentPolicy::Implicit,
    id_token_signing: IdTokenSigning::Rsa,
    refresh: RefreshTokenGrant::Granted,
    scopes: &["openid", "profile"],
};

pub const STAFF_CREDENTIALS: ClientCredentials = ClientCredentials::Asserted { client_id: "staff" };

pub const STAFF_PRIVILEGES: ConfidentialPrivileges = ConfidentialPrivileges {
    client_credentials: ClientCredentialsGrant::Granted {
        scopes: &["profile"],
    },
    introspection: IntrospectionPermission::Forbidden,
};
