use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;

use margaret_http::response::Response;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_token_signer::access_token_minting::AccessTokenMinting;

use crate::mint_access_token_error::MintAccessTokenError;
use crate::mint_access_token_request::MintAccessTokenRequest;

pub struct MintAccessTokenHandler {
    secret_store: Arc<JwksSecretStore>,
}

impl MintAccessTokenHandler {
    #[must_use]
    pub fn create(secret_store: Arc<JwksSecretStore>) -> Self {
        Self { secret_store }
    }

    /// # Errors
    ///
    /// Returns `MintAccessTokenError::SecretStore` when the signing keys cannot mint the tokens.
    pub fn respond(
        &self,
        MintAccessTokenRequest { refresh_token }: MintAccessTokenRequest,
        now: DateTime<Utc>,
    ) -> Result<Response, MintAccessTokenError> {
        let minting = self
            .secret_store
            .mint_access_token(&refresh_token, now)
            .map_err(|source| MintAccessTokenError::SecretStore { source })?;

        Ok(match minting {
            AccessTokenMinting::RefreshTokenSignedWithNextKey
            | AccessTokenMinting::RejectedRefreshToken(_) => Response::unauthorized(),
            AccessTokenMinting::Minted(minted) => Response::json(200, &minted),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;
    use std::sync::Arc;

    use margaret_jwks_keygen::jwks_secret::JwksSecret;
    use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
    use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
    use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
    use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
    use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
    use margaret_token_signer_tests::refresh_claims::refresh_claims;
    use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
    use margaret_token_signer_tests::unix_time::unix_time;

    use super::MintAccessTokenHandler;
    use crate::mint_access_token_error::MintAccessTokenError;
    use crate::mint_access_token_request::MintAccessTokenRequest;

    fn handler_from(secret: Option<JwksSecret>) -> MintAccessTokenHandler {
        MintAccessTokenHandler::create(Arc::new(match secret {
            Some(secret) => rolled_store(secret),
            None => unrolled_store(),
        }))
    }

    fn mint_request(refresh_token: String) -> MintAccessTokenRequest {
        MintAccessTokenRequest { refresh_token }
    }

    fn signed_refresh_token(secret: &JwksSecret, exp: i64) -> String {
        sign_refresh_token(
            secret.current(),
            &fixture_issuance(),
            &refresh_claims(),
            exp,
        )
    }

    #[test]
    fn mints_tokens_for_a_valid_refresh_token() {
        let secret = fresh_p256_secret();
        let refresh_token = signed_refresh_token(&secret, 1_000);
        let handler = handler_from(Some(secret));

        let response = handler
            .respond(mint_request(refresh_token), unix_time(500))
            .expect("the signing keys are available");

        assert_eq!(response.status(), 200);
    }

    #[test]
    fn reports_unauthorized_for_an_expired_refresh_token() {
        let secret = fresh_p256_secret();
        let refresh_token = signed_refresh_token(&secret, 1_000);
        let handler = handler_from(Some(secret));

        let response = handler
            .respond(mint_request(refresh_token), unix_time(2_000))
            .expect("the signing keys are available");

        assert_eq!(response.status(), 401);
    }

    #[test]
    fn reports_a_secret_store_failure_when_the_signing_keys_are_unavailable() {
        let handler = handler_from(None);

        let error = handler
            .respond(mint_request("any.token.value".to_string()), unix_time(500))
            .err()
            .expect("unrolled signing keys cannot mint");

        assert_eq!(
            discriminant(&error),
            discriminant(&MintAccessTokenError::SecretStore {
                source: JwksSecretStoreError::SecretUnavailable,
            })
        );
    }
}
