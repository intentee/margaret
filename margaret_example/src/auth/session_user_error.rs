use thiserror::Error;

use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_identity::responds_to_inference_failure::RespondsToInferenceFailure;

#[derive(Debug, Error)]
pub enum SessionUserError {
    #[error("the session cookie does not carry a session identifier: {source}")]
    MalformedSession {
        #[source]
        source: uuid::Error,
    },
}

impl RespondsToInferenceFailure for SessionUserError {
    fn into_response_continuation(self) -> ResponseContinuation {
        match self {
            Self::MalformedSession { .. } => {
                ResponseContinuation::from(Response::text(400, "Malformed session cookie"))
            }
        }
    }
}
