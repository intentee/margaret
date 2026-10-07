use bytes::Bytes;

use margaret_http::response::Response;
use margaret_identity_session::id_token_members::ID_TOKEN_MEMBERS;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;
use margaret_oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;
use margaret_oauth_vocabulary::code_challenge_method::CodeChallengeMethod;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oauth_vocabulary::prompt_value::PromptValue;
use margaret_oidc_discovery::provider_metadata_document::ProviderMetadataDocument;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::provider_endpoints::ProviderEndpoints;
use crate::provider_support::ProviderSupport;

const ASSERTED_AUTHENTICATION: [ClientAuthenticationMethod; 1] =
    [ClientAuthenticationMethod::PrivateKeyJwt];
const ASSERTED_OR_PUBLIC_AUTHENTICATION: [ClientAuthenticationMethod; 2] = [
    ClientAuthenticationMethod::PrivateKeyJwt,
    ClientAuthenticationMethod::None,
];

fn wire_values<'value>(values: impl IntoIterator<Item = &'value str>) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn authentication_methods(methods: &[ClientAuthenticationMethod]) -> Vec<String> {
    wire_values(
        methods
            .iter()
            .copied()
            .map(ClientAuthenticationMethod::wire_name),
    )
}

fn assertion_signing_algorithms() -> Vec<String> {
    wire_values(JwsAlgorithm::ALL.map(JwsAlgorithm::wire_name))
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
            scopes,
        }: ProviderSupport,
        endpoints: ProviderEndpoints,
        issuance: TokenIssuance,
    ) -> Result<Self, serde_json::Error> {
        serde_json::to_vec(&ProviderMetadataDocument {
            authorization_endpoint: Some(endpoints.authorization.to_string()),
            authorization_response_iss_parameter_supported: true,
            claims_supported: Some(wire_values(ID_TOKEN_MEMBERS)),
            code_challenge_methods_supported: Some(wire_values([
                CodeChallengeMethod::S256.wire_name()
            ])),
            grant_types_supported: Some(wire_values(
                grant_types.iter().copied().map(GrantType::wire_name),
            )),
            id_token_signing_alg_values_supported: Some(wire_values([
                JWKS_CURVE.curve().algorithm().wire_name(),
                JwsAlgorithm::Rs256.wire_name(),
            ])),
            introspection_endpoint: Some(endpoints.introspection.to_string()),
            introspection_endpoint_auth_methods_supported: Some(authentication_methods(
                &ASSERTED_AUTHENTICATION,
            )),
            introspection_endpoint_auth_signing_alg_values_supported: Some(
                assertion_signing_algorithms(),
            ),
            issuer: issuance.issuer.to_string(),
            jwks_uri: endpoints.jwks.to_string(),
            prompt_values_supported: Some(wire_values(
                PromptValue::SUPPORTED.map(PromptValue::wire_name),
            )),
            response_modes_supported: Some(wire_values(["query"])),
            response_types_supported: Some(wire_values(["code"])),
            revocation_endpoint: Some(endpoints.revocation.to_string()),
            revocation_endpoint_auth_methods_supported: Some(authentication_methods(
                &ASSERTED_OR_PUBLIC_AUTHENTICATION,
            )),
            revocation_endpoint_auth_signing_alg_values_supported: Some(
                assertion_signing_algorithms(),
            ),
            scopes_supported: Some(wire_values(scopes.iter().copied())),
            subject_types_supported: Some(wire_values(["public"])),
            token_endpoint: Some(endpoints.token.to_string()),
            token_endpoint_auth_methods_supported: Some(authentication_methods(
                &ASSERTED_OR_PUBLIC_AUTHENTICATION,
            )),
            token_endpoint_auth_signing_alg_values_supported: Some(assertion_signing_algorithms()),
            userinfo_endpoint: Some(endpoints.userinfo.to_string()),
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
