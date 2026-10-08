use std::collections::BTreeSet;

use bytes::Bytes;

use margaret_accepted_clients::assertion_signing::AssertionSigning;
use margaret_http::response::Response;
use margaret_identity_session::id_token_members::ID_TOKEN_MEMBERS;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::code_challenge_method::CodeChallengeMethod;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oauth_vocabulary::prompt_value::PromptValue;
use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
use margaret_oidc_discovery::provider_metadata_document::ProviderMetadataDocument;
use margaret_oidc_discovery::served_endpoint::ServedEndpoint;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::endpoint_authentication::EndpointAuthentication;
use crate::provider_support::ProviderSupport;

fn wire_values<'value>(values: impl IntoIterator<Item = &'value str>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn served(endpoint: ServedEndpoint) -> Option<String> {
    match endpoint {
        ServedEndpoint::Served(url) => Some(url.to_string()),
        ServedEndpoint::Unserved => None,
    }
}

fn own_algorithm() -> JwsAlgorithm {
    JWKS_CURVE.curve().algorithm()
}

fn authentication_methods(
    EndpointAuthentication { methods, .. }: EndpointAuthentication,
) -> Vec<String> {
    wire_values(
        methods
            .iter()
            .copied()
            .map(ClientAuthenticationMethod::wire_name),
    )
}

fn assertion_signing_algorithms(
    EndpointAuthentication { signing, .. }: EndpointAuthentication,
) -> Option<Vec<String>> {
    (!signing.is_empty()).then(|| {
        wire_values(signing.iter().map(|signing| match signing {
            AssertionSigning::Own => own_algorithm().wire_name(),
            AssertionSigning::Pinned(algorithm) => algorithm.wire_name(),
        }))
    })
}

fn id_token_signing_algorithms(signings: &[IdTokenSigning]) -> Vec<String> {
    wire_values(signings.iter().map(|signing| match signing {
        IdTokenSigning::EllipticCurve => own_algorithm().wire_name(),
        IdTokenSigning::Rsa => JwsAlgorithm::Rs256.wire_name(),
    }))
}

fn served_with<Member>(
    endpoint: ServedEndpoint,
    member: impl FnOnce() -> Member,
) -> Option<Member> {
    match endpoint {
        ServedEndpoint::Served(_) => Some(member()),
        ServedEndpoint::Unserved => None,
    }
}

pub struct ProviderMetadataHandler {
    document: Bytes,
}

impl ProviderMetadataHandler {
    /// # Errors
    ///
    /// Returns the `serde_json::Error` of a provider metadata document that cannot be serialized.
    pub fn create(
        ProviderSupport {
            grant_types,
            id_token_signing,
            introspection,
            revocation,
            scopes,
            token,
        }: ProviderSupport,
        endpoints: ProviderEndpoints,
        issuance: TokenIssuance,
    ) -> Result<Self, serde_json::Error> {
        let authorization = endpoints.authorization;

        serde_json::to_vec(&ProviderMetadataDocument {
            authorization_endpoint: served(authorization),
            authorization_response_iss_parameter_supported: matches!(
                authorization,
                ServedEndpoint::Served(_)
            ),
            claims_supported: served_with(endpoints.userinfo, || wire_values(ID_TOKEN_MEMBERS)),
            code_challenge_methods_supported: served_with(authorization, || {
                wire_values([CodeChallengeMethod::S256.wire_name()])
            }),
            grant_types_supported: Some(wire_values(
                grant_types.iter().copied().map(GrantType::wire_name),
            )),
            id_token_signing_alg_values_supported: Some(id_token_signing_algorithms(
                id_token_signing,
            )),
            introspection_endpoint: served(endpoints.introspection),
            introspection_endpoint_auth_methods_supported: served_with(
                endpoints.introspection,
                || authentication_methods(introspection),
            ),
            introspection_endpoint_auth_signing_alg_values_supported: served_with(
                endpoints.introspection,
                || assertion_signing_algorithms(introspection),
            )
            .flatten(),
            issuer: issuance.issuer.to_string(),
            jwks_uri: endpoints.jwks.to_string(),
            prompt_values_supported: served_with(authorization, || {
                wire_values(PromptValue::SUPPORTED.map(PromptValue::wire_name))
            }),
            request_uri_parameter_supported: Some(false),
            response_modes_supported: served_with(authorization, || wire_values(["query"])),
            response_types_supported: Some(match authorization {
                ServedEndpoint::Served(_) => wire_values(["code"]),
                ServedEndpoint::Unserved => Vec::new(),
            }),
            revocation_endpoint: served(endpoints.revocation),
            revocation_endpoint_auth_methods_supported: served_with(endpoints.revocation, || {
                authentication_methods(revocation)
            }),
            revocation_endpoint_auth_signing_alg_values_supported: served_with(
                endpoints.revocation,
                || assertion_signing_algorithms(revocation),
            )
            .flatten(),
            scopes_supported: Some(wire_values(scopes.iter().copied())),
            subject_types_supported: Some(wire_values(["public"])),
            token_endpoint: Some(endpoints.token.to_string()),
            token_endpoint_auth_methods_supported: Some(authentication_methods(token)),
            token_endpoint_auth_signing_alg_values_supported: assertion_signing_algorithms(token),
            userinfo_endpoint: served(endpoints.userinfo),
        })
        .map(|document| Self {
            document: Bytes::from(document),
        })
    }

    #[must_use]
    pub fn respond(&self) -> Response {
        Response::bytes(200, "application/json", self.document.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;

    use margaret_accepted_clients::assertion_signing::AssertionSigning;
    use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
    use margaret_oauth_vocabulary::grant_type::GrantType;
    use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
    use margaret_oidc_discovery::served_endpoint::ServedEndpoint;
    use margaret_token_issuance::token_issuance::TokenIssuance;

    use super::ProviderMetadataHandler;
    use crate::endpoint_authentication::EndpointAuthentication;
    use crate::provider_support::ProviderSupport;

    const UNUSED_AUTHENTICATION: EndpointAuthentication = EndpointAuthentication {
        methods: &[],
        signing: &[],
    };

    #[test]
    fn advertises_no_authorization_members_without_a_code_grant() {
        let handler = ProviderMetadataHandler::create(
            ProviderSupport {
                grant_types: &[GrantType::ClientCredentials],
                id_token_signing: &[],
                introspection: UNUSED_AUTHENTICATION,
                revocation: UNUSED_AUTHENTICATION,
                scopes: &["reports"],
                token: EndpointAuthentication {
                    methods: &[ClientAuthenticationMethod::PrivateKeyJwt],
                    signing: &[AssertionSigning::Own],
                },
            },
            ProviderEndpoints {
                authorization: ServedEndpoint::Unserved,
                introspection: ServedEndpoint::Unserved,
                issuer_origin: "https://localhost",
                jwks: "https://localhost/jwks.json",
                revocation: ServedEndpoint::Unserved,
                token: "https://localhost/token",
                userinfo: ServedEndpoint::Unserved,
            },
            TokenIssuance {
                audience: "session",
                issuer: "https://localhost",
            },
        )
        .expect("the provider metadata serializes");

        assert_eq!(
            serde_json::from_slice::<Value>(&handler.document)
                .expect("the provider metadata is json"),
            json!({
                "authorization_response_iss_parameter_supported": false,
                "grant_types_supported": ["client_credentials"],
                "id_token_signing_alg_values_supported": [],
                "issuer": "https://localhost",
                "jwks_uri": "https://localhost/jwks.json",
                "request_uri_parameter_supported": false,
                "response_types_supported": [],
                "scopes_supported": ["reports"],
                "subject_types_supported": ["public"],
                "token_endpoint": "https://localhost/token",
                "token_endpoint_auth_methods_supported": ["private_key_jwt"],
                "token_endpoint_auth_signing_alg_values_supported": ["ES256"],
            })
        );
    }
}
