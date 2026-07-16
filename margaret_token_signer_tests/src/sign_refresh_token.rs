use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_key_gen::jwk_signing::JwkSigning;
use margaret_jwks_key_gen::signs_claims::SignsClaims;

pub async fn sign_refresh_token(signing: &JwkSigning, claims: &RefreshTokenClaims) -> String {
    signing
        .sign(claims)
        .await
        .expect("signing the refresh token fixture succeeds")
}
