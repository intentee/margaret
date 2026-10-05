use std::borrow::Borrow;
use std::collections::BTreeSet;

use bytes::Bytes;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::client_credentials_grant::ClientCredentialsGrant;
use margaret_accepted_clients::confidential_privileges::ConfidentialPrivileges;
use margaret_http::response::Response;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;
use margaret_oauth_vocabulary::code_challenge_method::CodeChallengeMethod;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oidc_discovery::provider_metadata_document::ProviderMetadataDocument;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::provider_endpoints::ProviderEndpoints;

fn wire_values<'value>(values: impl IntoIterator<Item = &'value str>) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn supported_scopes(client: &AcceptedClient) -> impl Iterator<Item = &str> {
    let code_scopes = match &client.authorization_code {
        AuthorizationCodeGrant::Granted(policy) => {
            policy.scopes.iter().map(Scope::as_str).collect()
        }
        AuthorizationCodeGrant::Withheld => Vec::new(),
    };
    let client_credentials_scopes = match &client.authentication {
        AcceptedClientAuthentication::ClientSecretBasic {
            privileges:
                ConfidentialPrivileges {
                    client_credentials: ClientCredentialsGrant::Granted { scopes },
                    ..
                },
            ..
        } => scopes
            .iter()
            .map(|scope| Borrow::<Scope>::borrow(scope).as_str())
            .collect(),
        AcceptedClientAuthentication::ClientSecretBasic { .. }
        | AcceptedClientAuthentication::Public => Vec::new(),
    };

    code_scopes.into_iter().chain(client_credentials_scopes)
}

pub struct ProviderMetadataHandler {
    document: Bytes,
}

impl ProviderMetadataHandler {
    /// # Errors
    ///
    /// Returns the `serde_json::Error` of a provider metadata document that cannot be serialized.
    pub fn create(
        clients: &AcceptedClients,
        endpoints: &ProviderEndpoints,
        issuance: &dyn DeclaresTokenIssuance,
    ) -> Result<Self, serde_json::Error> {
        let grant_types = clients
            .clients()
            .flat_map(|client| {
                GrantType::ALL
                    .into_iter()
                    .filter(|grant_type| client.grants(*grant_type))
            })
            .collect::<BTreeSet<GrantType>>();
        let scopes = clients
            .clients()
            .flat_map(supported_scopes)
            .collect::<BTreeSet<&str>>();

        serde_json::to_vec(&ProviderMetadataDocument {
            authorization_endpoint: Some(endpoints.authorization.to_string()),
            authorization_response_iss_parameter_supported: true,
            code_challenge_methods_supported: Some(wire_values([
                CodeChallengeMethod::S256.wire_name()
            ])),
            grant_types_supported: Some(wire_values(
                grant_types.into_iter().map(GrantType::wire_name),
            )),
            id_token_signing_alg_values_supported: Some(wire_values([
                JWKS_CURVE.curve().algorithm().wire_name(),
                JwsAlgorithm::Rs256.wire_name(),
            ])),
            introspection_endpoint: Some(endpoints.introspection.to_string()),
            introspection_endpoint_auth_methods_supported: Some(wire_values([
                "client_secret_basic",
            ])),
            issuer: issuance.token_issuance().issuer.as_str().to_string(),
            jwks_uri: endpoints.jwks.to_string(),
            response_modes_supported: Some(wire_values(["query"])),
            response_types_supported: Some(wire_values(["code"])),
            revocation_endpoint: Some(endpoints.revocation.to_string()),
            revocation_endpoint_auth_methods_supported: Some(wire_values([
                "client_secret_basic",
                "none",
            ])),
            scopes_supported: Some(wire_values(scopes)),
            subject_types_supported: Some(wire_values(["public"])),
            token_endpoint: Some(endpoints.token.to_string()),
            token_endpoint_auth_methods_supported: Some(wire_values([
                "client_secret_basic",
                "none",
            ])),
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
