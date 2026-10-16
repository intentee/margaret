use margaret_diagnostic::report_diagnostic::report_diagnostic;
use margaret_handler_error::handler_error::HandlerError;

use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub(crate) fn respond_to_outcome(outcome: Result<ResponseContinuation, HandlerError>) -> Response {
    match outcome {
        Ok(continuation) => continuation.into_response(),
        Err(error) => {
            report_diagnostic(format_args!(
                "margaret_http: request handling failed: {error:#}"
            ));

            Response::text(500, "Internal Server Error")
        }
    }
}
