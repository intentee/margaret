use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_accepted_clients::refresh_token_grant::RefreshTokenGrant;
use margaret_jwks_secret_store::id_token_signing::IdTokenSigning;

pub const SPA_CODE_GRANT: CodeGrantPolicy = CodeGrantPolicy {
    consent: ConsentPolicy::Prompted,
    id_token_signing: IdTokenSigning::EllipticCurve,
    refresh: RefreshTokenGrant::Withheld,
    scopes: &["openid"],
};
