use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;
use margaret_oauth_vocabulary::grant_type::GrantType;

use crate::endpoint_authentication::EndpointAuthentication;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderSupport {
    pub grant_types: &'static [GrantType],
    pub id_token_signing: &'static [IdTokenSigning],
    pub introspection: EndpointAuthentication,
    pub revocation: EndpointAuthentication,
    pub scopes: &'static [&'static str],
    pub token: EndpointAuthentication,
}
