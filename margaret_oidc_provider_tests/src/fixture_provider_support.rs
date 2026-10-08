use margaret_accepted_clients::assertion_signing::AssertionSigning;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oidc_provider::endpoint_authentication::EndpointAuthentication;
use margaret_oidc_provider::provider_support::ProviderSupport;

const PORTAL_AUTHENTICATION: EndpointAuthentication = EndpointAuthentication {
    methods: &[ClientAuthenticationMethod::PrivateKeyJwt],
    signing: &[AssertionSigning::Pinned(JwsAlgorithm::Es256)],
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
