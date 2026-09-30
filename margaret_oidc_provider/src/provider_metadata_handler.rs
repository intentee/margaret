use std::collections::BTreeSet;

use bytes::Bytes;
use serde_json::json;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::grant_type::GrantType;
use margaret_http::response::Response;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::provider_endpoints::ProviderEndpoints;

pub struct ProviderMetadataHandler {
    document: Bytes,
}

impl ProviderMetadataHandler {
    #[must_use]
    pub fn create(
        clients: &AcceptedClients,
        endpoints: &ProviderEndpoints,
        issuance: &dyn DeclaresTokenIssuance,
    ) -> Self {
        let grants = clients
            .clients()
            .flat_map(|client| client.grants.iter().copied())
            .collect::<BTreeSet<GrantType>>();
        let scopes = clients
            .clients()
            .flat_map(|client| client.scopes.iter().map(Scope::as_str))
            .collect::<BTreeSet<&str>>();

        Self {
            document: Bytes::from(
                json!({
                    "authorization_endpoint": endpoints.authorization.as_str(),
                    "authorization_response_iss_parameter_supported": true,
                    "code_challenge_methods_supported": ["S256"],
                    "grant_types_supported": grants
                        .into_iter()
                        .map(GrantType::wire_name)
                        .collect::<Vec<&str>>(),
                    "id_token_signing_alg_values_supported": [
                        JWKS_CURVE.curve().algorithm().wire_name(),
                        JwsAlgorithm::Rs256.wire_name(),
                    ],
                    "introspection_endpoint": endpoints.introspection.as_str(),
                    "introspection_endpoint_auth_methods_supported": ["client_secret_basic"],
                    "issuer": issuance.token_issuance().issuer.as_str(),
                    "jwks_uri": endpoints.jwks.as_str(),
                    "response_modes_supported": ["query"],
                    "response_types_supported": ["code"],
                    "revocation_endpoint": endpoints.revocation.as_str(),
                    "revocation_endpoint_auth_methods_supported": ["client_secret_basic", "none"],
                    "scopes_supported": scopes,
                    "subject_types_supported": ["public"],
                    "token_endpoint": endpoints.token.as_str(),
                    "token_endpoint_auth_methods_supported": ["client_secret_basic", "none"],
                    "userinfo_endpoint": endpoints.userinfo.as_str(),
                })
                .to_string(),
            ),
        }
    }

    #[must_use]
    pub fn respond(&self) -> Response {
        Response::bytes(200, "application/json", self.document.clone())
    }
}
