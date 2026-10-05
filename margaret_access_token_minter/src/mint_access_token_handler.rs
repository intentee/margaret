use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;

use margaret_http::response::Response;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_token_signer::access_token_minting::AccessTokenMinting;

use crate::mint_access_token_request::MintAccessTokenRequest;

pub struct MintAccessTokenHandler {
    secret_store: Arc<JwksSecretStore>,
}

impl MintAccessTokenHandler {
    #[must_use]
    pub fn create(secret_store: Arc<JwksSecretStore>) -> Self {
        Self { secret_store }
    }

    #[must_use]
    pub fn respond(
        &self,
        MintAccessTokenRequest { refresh_token }: MintAccessTokenRequest,
        now: DateTime<Utc>,
    ) -> Response {
        match self.secret_store.mint_access_token(&refresh_token, now) {
            AccessTokenMinting::RefreshTokenSignedWithNextKey
            | AccessTokenMinting::RejectedRefreshToken(_) => Response::unauthorized(),
            AccessTokenMinting::Minted(minted) => Response::json(200, &minted),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_jwks_keygen::jwks_secret::JwksSecret;
    use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
    use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
    use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
    use margaret_token_signer_tests::refresh_claims::refresh_claims;
    use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
    use margaret_token_signer_tests::unix_time::unix_time;

    use super::MintAccessTokenHandler;
    use crate::mint_access_token_request::MintAccessTokenRequest;

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
        let handler = MintAccessTokenHandler::create(Arc::new(rolled_store(secret)));

        assert_eq!(
            handler
                .respond(mint_request(refresh_token), unix_time(500))
                .status(),
            200
        );
    }

    #[test]
    fn reports_unauthorized_for_an_expired_refresh_token() {
        let secret = fresh_p256_secret();
        let refresh_token = signed_refresh_token(&secret, 1_000);
        let handler = MintAccessTokenHandler::create(Arc::new(rolled_store(secret)));

        assert_eq!(
            handler
                .respond(mint_request(refresh_token), unix_time(2_000))
                .status(),
            401
        );
    }
}
