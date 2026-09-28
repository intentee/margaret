use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_token_signer::access_token_minting::AccessTokenMinting;

use crate::mint_access_token_error::MintAccessTokenError;
use crate::mint_access_token_request::MintAccessTokenRequest;

fn error_response(error: &MintAccessTokenError) -> Response {
    match error {
        MintAccessTokenError::MissingBody | MintAccessTokenError::MalformedRequest { .. } => {
            Response::text(400, "Bad Request")
        }
        MintAccessTokenError::SecretStore { .. } => Response::text(500, "Internal Server Error"),
    }
}

pub struct MintAccessTokenHandler {
    secret_store: Arc<JwksSecretStore>,
}

impl MintAccessTokenHandler {
    #[must_use]
    pub fn create(secret_store: Arc<JwksSecretStore>) -> Self {
        Self { secret_store }
    }

    #[must_use]
    pub fn respond(&self, request: &Request, now: DateTime<Utc>) -> Response {
        match self.mint(request, now) {
            Ok(response) => response,
            Err(error) => error_response(&error),
        }
    }

    fn mint(
        &self,
        request: &Request,
        now: DateTime<Utc>,
    ) -> Result<Response, MintAccessTokenError> {
        let MintAccessTokenRequest { refresh_token } =
            MintAccessTokenRequest::from_request(request)?;

        let minting = self
            .secret_store
            .mint_access_token(&refresh_token, now)
            .map_err(|source| MintAccessTokenError::SecretStore { source })?;

        Ok(match minting {
            AccessTokenMinting::ExpiredRefreshToken
            | AccessTokenMinting::MalformedRefreshTokenClaims(_)
            | AccessTokenMinting::RefreshTokenSignedWithNextKey
            | AccessTokenMinting::RejectedRefreshToken(_) => Response::unauthorized(),
            AccessTokenMinting::Minted(minted) => Response::json(200, &minted),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use http::Method;
    use serde_json::Value;
    use serde_json::json;

    use margaret_http::request::Request;
    use margaret_jwks_keygen::jwks_secret::JwksSecret;
    use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
    use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
    use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
    use margaret_token_signer_tests::refresh_claims::refresh_claims;
    use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
    use margaret_token_signer_tests::unix_time::unix_time;

    use super::MintAccessTokenHandler;

    fn handler_from(secret: Option<JwksSecret>) -> MintAccessTokenHandler {
        let holder = JwksSecretHolder::default();

        holder.set(secret.map(Arc::new));

        MintAccessTokenHandler::create(Arc::new(JwksSecretStore::new(holder)))
    }

    fn request_with_body(body: Value) -> Request {
        let mut request = Request::new(Method::POST, "/mint".to_string());

        request.inputs.json = Some(body);

        request
    }

    fn signed_refresh_token(secret: &JwksSecret, exp: i64) -> String {
        sign_refresh_token(secret.current(), &refresh_claims(exp))
    }

    #[test]
    fn mints_tokens_for_a_valid_refresh_token() {
        let secret = fresh_p256_secret();
        let refresh_token = signed_refresh_token(&secret, 1_000);
        let handler = handler_from(Some(secret));

        let response = handler.respond(
            &request_with_body(json!({ "refresh_token": refresh_token })),
            unix_time(500),
        );

        assert_eq!(response.status(), 200);
    }

    #[test]
    fn reports_bad_request_when_the_body_is_missing() {
        let handler = handler_from(Some(fresh_p256_secret()));

        let response = handler.respond(
            &Request::new(Method::POST, "/mint".to_string()),
            unix_time(500),
        );

        assert_eq!(response.status(), 400);
    }

    #[test]
    fn reports_bad_request_for_a_malformed_body() {
        let handler = handler_from(Some(fresh_p256_secret()));

        let response = handler.respond(
            &request_with_body(json!({ "wrong": "field" })),
            unix_time(500),
        );

        assert_eq!(response.status(), 400);
    }

    #[test]
    fn reports_unauthorized_for_an_expired_refresh_token() {
        let secret = fresh_p256_secret();
        let refresh_token = signed_refresh_token(&secret, 1_000);
        let handler = handler_from(Some(secret));

        let response = handler.respond(
            &request_with_body(json!({ "refresh_token": refresh_token })),
            unix_time(2_000),
        );

        assert_eq!(response.status(), 401);
    }

    #[test]
    fn reports_internal_error_when_the_secret_is_unavailable() {
        let handler = handler_from(None);

        let response = handler.respond(
            &request_with_body(json!({ "refresh_token": "any.token.value" })),
            unix_time(500),
        );

        assert_eq!(response.status(), 500);
    }
}
