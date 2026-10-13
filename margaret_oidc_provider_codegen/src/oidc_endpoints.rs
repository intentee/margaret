use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::marked_endpoint::MarkedEndpoint;

fn oidc_endpoint_name(endpoint: MarkedEndpoint) -> &'static str {
    match endpoint {
        MarkedEndpoint::Authorization => "Authorization",
        MarkedEndpoint::Consent => "Consent",
        MarkedEndpoint::Discovery => "Discovery",
        MarkedEndpoint::Introspection => "Introspection",
        MarkedEndpoint::Jwks => "Jwks",
        MarkedEndpoint::Revocation => "Revocation",
        MarkedEndpoint::Token => "Token",
        MarkedEndpoint::Userinfo => "Userinfo",
    }
}

pub(crate) const OIDC_ENDPOINTS: FrameworkVocabulary<MarkedEndpoint> = FrameworkVocabulary {
    enum_path: &[
        "margaret",
        "framework",
        "oidc_provider",
        "oidc_endpoint",
        "OidcEndpoint",
    ],
    name: oidc_endpoint_name,
    variants: &[
        MarkedEndpoint::Authorization,
        MarkedEndpoint::Consent,
        MarkedEndpoint::Discovery,
        MarkedEndpoint::Introspection,
        MarkedEndpoint::Jwks,
        MarkedEndpoint::Revocation,
        MarkedEndpoint::Token,
        MarkedEndpoint::Userinfo,
    ],
};
