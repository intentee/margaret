use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

use crate::consent_policy::ConsentPolicy;
use crate::refresh_token_grant::RefreshTokenGrant;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodeGrantPolicy {
    pub consent: ConsentPolicy,
    pub id_token_signing: IdTokenSigning,
    pub refresh: RefreshTokenGrant,
    pub scopes: &'static [&'static str],
}
