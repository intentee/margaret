use async_trait::async_trait;

use margaret_bearer_token_verification::admit_bearer_token::admit_bearer_token;
use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

pub struct AdmittingHandler {
    pub verifier: BearerTokenVerifier,
}

#[async_trait]
impl Handler for AdmittingHandler {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(
            match admit_bearer_token::<TestClaims>(
                &self.verifier,
                request.inputs.server.authorization(),
            )
            .expect("the system clock reads as a numeric date")
            {
                BearerTokenAdmission::Anonymous => {
                    ResponseContinuation::from(Response::text(200, "anonymous"))
                }
                BearerTokenAdmission::Presented(verified) => {
                    ResponseContinuation::from(Response::text(200, verified.claims.sub))
                }
                BearerTokenAdmission::Refused(continuation) => continuation,
            },
        )
    }
}
