use margaret_attributes::canonical_path::CanonicalPath;

use crate::declared_endpoint_routes::DeclaredEndpointRoutes;
use crate::declared_userinfo_claims::DeclaredUserinfoClaims;
use crate::oidc_provider_codegen_error::OidcProviderCodegenError;
use crate::provider_endpoint::ProviderEndpoint;

pub enum UserinfoProvision {
    Served { claims: CanonicalPath },
    Unserved,
}

impl UserinfoProvision {
    /// # Errors
    ///
    /// Returns `OidcProviderCodegenError::MissingUserinfoClaimsProvider` when the userinfo
    /// endpoint is routed without a claims provider, and
    /// `OidcProviderCodegenError::UnconsumedUserinfoClaimsProvider` for a claims provider of an
    /// unrouted userinfo endpoint.
    pub fn of(
        marked: &DeclaredEndpointRoutes,
        claims: DeclaredUserinfoClaims,
    ) -> Result<Self, OidcProviderCodegenError> {
        match (marked.serves(ProviderEndpoint::Userinfo), claims) {
            (true, DeclaredUserinfoClaims::Declared(claims)) => Ok(Self::Served { claims }),
            (true, DeclaredUserinfoClaims::Absent) => {
                Err(OidcProviderCodegenError::MissingUserinfoClaimsProvider)
            }
            (false, DeclaredUserinfoClaims::Declared(provider)) => {
                Err(OidcProviderCodegenError::UnconsumedUserinfoClaimsProvider {
                    provider: provider.to_string(),
                })
            }
            (false, DeclaredUserinfoClaims::Absent) => Ok(Self::Unserved),
        }
    }
}
