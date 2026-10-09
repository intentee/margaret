use margaret_attributes::attribute_index::AttributeIndex;
use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_oidc_provider_codegen::declared_endpoint_routes::DeclaredEndpointRoutes;
use margaret_oidc_provider_codegen::declared_userinfo_claims::DeclaredUserinfoClaims;
use margaret_oidc_provider_codegen::userinfo_provision::UserinfoProvision;
use margaret_session_endpoints_codegen::declared_session_endpoints::DeclaredSessionEndpoints;

use crate::codegen_error::CodegenError;
use crate::identity_declarations::IdentityDeclarations;
use crate::provided_endpoints::ProvidedEndpoints;

pub(crate) struct ServedEndpoints {
    pub(crate) provider: ProvidedEndpoints,
    pub(crate) routes: DeclaredEndpointRoutes,
    pub(crate) sessions: DeclaredSessionEndpoints,
    pub(crate) userinfo: UserinfoProvision,
}

impl ServedEndpoints {
    pub(crate) fn read(
        index: &AttributeIndex,
        declared_routes: &DeclaredRoutes,
        IdentityDeclarations {
            accepted_clients,
            sessions,
            token_issuance,
            ..
        }: &IdentityDeclarations,
    ) -> Result<Self, CodegenError> {
        let routes = DeclaredEndpointRoutes::read(index, declared_routes)?;
        let userinfo = UserinfoProvision::of(&routes, DeclaredUserinfoClaims::read(index)?)?;

        Ok(Self {
            provider: ProvidedEndpoints::derive(
                accepted_clients,
                token_issuance,
                &routes,
                sessions,
            )?,
            sessions: DeclaredSessionEndpoints::read(index, declared_routes, sessions)?,
            routes,
            userinfo,
        })
    }
}
