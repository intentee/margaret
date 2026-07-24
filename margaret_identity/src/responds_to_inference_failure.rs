use margaret_http::response_continuation::ResponseContinuation;

pub trait RespondsToInferenceFailure {
    fn into_response_continuation(self) -> ResponseContinuation;
}
