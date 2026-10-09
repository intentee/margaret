use margaret_accepted_clients_codegen::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_accepted_clients_codegen::declared_code_grant::DeclaredCodeGrant;
use margaret_accepted_clients_codegen::declared_code_policy::DeclaredCodePolicy;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oidc_provider_codegen::declared_endpoint_routes::DeclaredEndpointRoutes;
use margaret_oidc_provider_codegen::derive_provider_endpoints::derive_provider_endpoints;
use margaret_oidc_provider_codegen::derived_authorization::DerivedAuthorization;
use margaret_oidc_provider_codegen::derived_provider_endpoints::DerivedProviderEndpoints;
use margaret_oidc_provider_codegen::oidc_provider_codegen_error::OidcProviderCodegenError;
use margaret_oidc_provider_codegen::provider_endpoint::ProviderEndpoint;
use margaret_oidc_provider_codegen::reject_unadmitted_endpoint_routes::reject_unadmitted_endpoint_routes;
use margaret_sessions_codegen::declared_sessions::DeclaredSessions;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

fn grants_codes(
    accepted: &DeclaredAcceptedClients,
    granted: impl Fn(&DeclaredCodePolicy) -> bool,
) -> bool {
    accepted.clients.iter().any(|client| {
        matches!(&client.authorization_code, DeclaredCodeGrant::Granted(policy) if granted(policy))
    })
}

fn capable_endpoints(accepted: &DeclaredAcceptedClients) -> Vec<ProviderEndpoint> {
    let mut capable = Vec::new();

    if grants_codes(accepted, |_| true) {
        capable.push(ProviderEndpoint::Authorization);
    }

    if accepted.clients.iter().any(|client| {
        matches!(
            &client.authentication,
            DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential) if confidential.introspection
        )
    }) {
        capable.push(ProviderEndpoint::Introspection);
    }

    if grants_codes(accepted, |policy| policy.refresh_token) {
        capable.push(ProviderEndpoint::Revocation);
    }

    if grants_codes(accepted, |policy| {
        policy.scopes.iter().any(Scope::is_openid)
    }) {
        capable.push(ProviderEndpoint::Userinfo);
    }

    capable
}

pub(crate) enum ProvidedEndpoints {
    Derived(Box<DerivedProviderEndpoints>),
    Unprovided,
}

impl ProvidedEndpoints {
    pub(crate) fn derive(
        accepted: &DeclaredAcceptedClients,
        token_issuance: &DeclaredTokenIssuance,
        marked: &DeclaredEndpointRoutes,
        sessions: &DeclaredSessions,
    ) -> Result<Self, OidcProviderCodegenError> {
        match token_issuance {
            DeclaredTokenIssuance::Declared(declaration) if !accepted.clients.is_empty() => {
                derive_provider_endpoints(
                    marked,
                    &declaration.issuer,
                    &capable_endpoints(accepted),
                    sessions,
                )
                .map(|endpoints| Self::Derived(Box::new(endpoints)))
            }
            DeclaredTokenIssuance::Absent | DeclaredTokenIssuance::Declared(_) => {
                reject_unadmitted_endpoint_routes(marked).map(|()| Self::Unprovided)
            }
        }
    }

    pub(crate) fn authorization(&self) -> &DerivedAuthorization {
        match self {
            Self::Derived(endpoints) => &endpoints.authorization,
            Self::Unprovided => &DerivedAuthorization::Unserved,
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_oauth_vocabulary_codegen::declared_scopes::DeclaredScopes;
    use margaret_oidc_provider_codegen::provider_endpoint::ProviderEndpoint;
    use margaret_token_issuance_codegen::declared_resource_issuances::DeclaredResourceIssuances;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::capable_endpoints;

    fn capable(authentication: &str, grants: &str) -> Vec<ProviderEndpoint> {
        let indexed = IndexedSource::new(&format!(
            "use margaret::framework::accepted_clients::client_keys::ClientKeys;\nuse margaret::framework::accepted_clients::consent_policy::ConsentPolicy;\nuse margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm;\nuse margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning;\nuse margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod;\nuse margaret::framework::oauth_vocabulary::openid_scope::OpenidScope;\n\n#[oauth_scope(name = \"artifacts:read\")]\npub struct ArtifactsReadScope;\n#[oauth_scope(name = \"profile\")]\npub struct ProfileScope;\n\n#[issues_tokens(provider, issuer = \"https://issuer.example\")]\npub struct Issuer;\n#[issues_resource_tokens(artifacts, audience = \"artifacts\")]\npub struct Artifacts;\n#[admits_oauth_client(portal_app, authentication = ClientAuthenticationMethod::PrivateKeyJwt({authentication}keys = ClientKeys::Published(jwks_uri = \"https://portal.example/jwks.json\", signing = JwsAlgorithm::Es256)), {grants}client_id = \"portal\", resources = [artifacts])]\npub struct Portal;\n"
        ));
        let issuance =
            DeclaredTokenIssuance::read(&indexed.index).expect("the token issuance is read");
        let resources = DeclaredResourceIssuances::read(&indexed.index, &issuance)
            .expect("the resources are read");

        capable_endpoints(
            &DeclaredAcceptedClients::read(
                &indexed.index,
                &issuance,
                &resources,
                &DeclaredScopes::read(&indexed.index).expect("the scopes are read"),
            )
            .expect("the admitted client is read"),
        )
    }

    fn code_grant(scopes: &str, refresh_token: &str) -> String {
        format!(
            "authorization_code(consent = ConsentPolicy::Implicit, id_token_signing = IdTokenSigning::Rsa, redirect_uris = [\"https://portal.example/callback\"], scopes = [{scopes}]{refresh_token}), "
        )
    }

    #[test]
    fn needs_no_endpoint_for_a_client_that_only_requests_client_credentials() {
        assert_eq!(
            capable("client_credentials(scopes = [ArtifactsReadScope]), ", ""),
            Vec::<ProviderEndpoint>::new()
        );
    }

    #[test]
    fn needs_the_authorization_endpoint_for_a_client_granted_codes() {
        assert_eq!(
            capable("", &code_grant("ProfileScope", "")),
            vec![ProviderEndpoint::Authorization]
        );
    }

    #[test]
    fn needs_the_introspection_endpoint_for_a_client_that_introspects() {
        assert_eq!(
            capable("introspection, ", ""),
            vec![ProviderEndpoint::Introspection]
        );
    }

    #[test]
    fn needs_the_revocation_endpoint_for_a_client_granted_refresh_tokens() {
        assert_eq!(
            capable("", &code_grant("ProfileScope", ", refresh_token")),
            vec![
                ProviderEndpoint::Authorization,
                ProviderEndpoint::Revocation
            ]
        );
    }

    #[test]
    fn needs_the_userinfo_endpoint_for_a_client_granted_the_openid_scope() {
        assert_eq!(
            capable("", &code_grant("OpenidScope, ProfileScope", "")),
            vec![ProviderEndpoint::Authorization, ProviderEndpoint::Userinfo]
        );
    }
}
