use chrono::DateTime;
use chrono::Utc;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::access_token_stamp::AccessTokenStamp;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::refresh_token_profile::RefreshTokenProfile;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::access_token_minting::AccessTokenMinting;
use crate::minted_tokens::MintedTokens;

#[must_use]
pub fn mint_access_token(
    secret: &JwksSecret,
    issuance: &TokenIssuance,
    refresh_token: &str,
    now: DateTime<Utc>,
) -> AccessTokenMinting {
    let access_stamp = AccessTokenStamp::issued_by(issuance, now);
    let VerifiedJwt {
        claims: refresh_claims,
        registered: refresh_registered,
        ..
    } = match secret.verify_jwt::<RefreshTokenClaims, RefreshTokenProfile>(
        refresh_token,
        &JwtExpectation {
            audience: &issuance.audience,
            issuer: &issuance.issuer,
        },
        access_stamp.registered.iat,
    ) {
        JwksSecretVerificationResult::Rejected(rejection) => {
            return AccessTokenMinting::RejectedRefreshToken(rejection);
        }
        JwksSecretVerificationResult::SignedWithNextKey => {
            return AccessTokenMinting::RefreshTokenSignedWithNextKey;
        }
        JwksSecretVerificationResult::SignedWithCurrent(verified)
        | JwksSecretVerificationResult::SignedWithPrevious(verified) => verified,
    };
    let signer = secret.current();
    let access_claims = AccessTokenClaims {
        sub: refresh_claims.sub,
    };

    AccessTokenMinting::Minted(MintedTokens {
        access_token: signer.sign_json(
            &access_claims.to_payload(&access_stamp),
            JwtType::AccessToken,
        ),
        refresh_token: signer.sign_json(
            &refresh_claims.to_payload(&refresh_registered),
            JwtType::Refresh,
        ),
    })
}
