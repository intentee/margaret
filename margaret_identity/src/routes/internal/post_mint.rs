use std::sync::Arc;

use spiffe::spiffe_id::SpiffeId;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer::token_signer_error::TokenSignerError;
use margaret_validation::validation_result::ValidationResult;

use crate::clock::Clock;
use crate::forms::mint_request::MintRequest;
use crate::models::mint_response::MintResponse;
use crate::stores::signing_key_store::SigningKeyStore;

#[singleton]
#[responds_to_http(method = "post", path = "/access_token/mint", server = "internal")]
pub struct PostMint {
    clock: Arc<dyn Clock>,
    store: Arc<SigningKeyStore>,
}

impl PostMint {
    #[constructor]
    pub fn create(clock: Arc<dyn Clock>, store: Arc<SigningKeyStore>) -> Self {
        Self { clock, store }
    }

    #[process]
    pub async fn respond(
        &self,
        _peer: &SpiffeId,
        #[form_request(from = Json)] request: ValidationResult<MintRequest>,
    ) -> Response {
        let MintRequest { refresh_token } = match request {
            ValidationResult::Valid(request) => request,
            ValidationResult::Invalid(errors) => return Response::text(422, errors.to_string()),
            ValidationResult::Malformed(malformation) => {
                return Response::text(400, malformation.to_string());
            }
        };

        let Some(secret) = self.store.current() else {
            return Response::text(503, "the signing keys are not ready");
        };

        match mint_access_token(&secret, &refresh_token, self.clock.now()).await {
            Ok(MintedTokens {
                access_token,
                refresh_token,
            }) => Response::json(
                200,
                &MintResponse {
                    access_token,
                    refresh_token,
                },
            ),
            Err(TokenSignerError::Signing { .. }) => {
                Response::text(500, "failed to sign the access token")
            }
            Err(
                TokenSignerError::ExpiredRefreshToken
                | TokenSignerError::InvalidRefreshToken
                | TokenSignerError::UnverifiableRefreshToken { .. },
            ) => Response::text(401, "invalid refresh token"),
        }
    }
}
