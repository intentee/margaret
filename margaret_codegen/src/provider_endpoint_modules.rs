use margaret_accepted_clients_codegen::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use margaret_accepted_clients_codegen::declared_accepted_clients::DeclaredAcceptedClients;
use margaret_accepted_clients_codegen::declared_code_grant::DeclaredCodeGrant;
use margaret_accepted_clients_codegen::declared_code_policy::DeclaredCodePolicy;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::route_location::RouteLocation;
use margaret_oauth_vocabulary::scope::Scope;
use margaret_oidc_provider_codegen::derive_provider_endpoints::derive_provider_endpoints;
use margaret_oidc_provider_codegen::oidc_provider_codegen_error::OidcProviderCodegenError;
use margaret_oidc_provider_codegen::provider_endpoint::ProviderEndpoint;
use margaret_oidc_provider_codegen::reject_unadmitted_endpoint_routes::reject_unadmitted_endpoint_routes;
use margaret_oidc_provider_codegen::render_provider_endpoints::render_provider_endpoints;
use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

use crate::provider_declarations::ProviderDeclarations;
use crate::provider_endpoint_artifacts::ProviderEndpointArtifacts;
use crate::provider_server::ProviderServer;

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

pub(crate) fn provider_endpoint_modules(
    locations: &[RouteLocation<'_>],
    bindings: &ContainerBindings,
    ProviderDeclarations {
        accepted,
        token_issuance,
    }: &ProviderDeclarations,
) -> Result<ProviderEndpointArtifacts, OidcProviderCodegenError> {
    match token_issuance {
        DeclaredTokenIssuance::Declared(declaration) if !accepted.clients.is_empty() => {
            derive_provider_endpoints(
                locations,
                bindings,
                &declaration.issuer,
                &capable_endpoints(accepted),
            )
            .map(|endpoints| ProviderEndpointArtifacts {
                modules: render_provider_endpoints(&endpoints),
                server: ProviderServer::Named(endpoints.server.clone()),
            })
        }
        DeclaredTokenIssuance::Absent | DeclaredTokenIssuance::Declared(_) => {
            reject_unadmitted_endpoint_routes(locations, bindings).map(|()| {
                ProviderEndpointArtifacts {
                    modules: Vec::new(),
                    server: ProviderServer::Absent,
                }
            })
        }
    }
}
