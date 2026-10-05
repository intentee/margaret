use serde::de::DeserializeOwned;
use validator::Validate;

use margaret_http::body_limit::BodyLimit;
use margaret_http::handler_error::HandlerError;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;

const FORM_LIMIT: BodyLimit = BodyLimit::new(16_384);

/// # Errors
///
/// Returns the `HandlerError` of an answer that fails.
pub async fn form_answer<TForm, TAnswer>(
    request: &Request,
    body: RequestBody,
    answer: impl FnOnce(ValidationResult<TForm>) -> TAnswer,
) -> Result<ResponseContinuation, HandlerError>
where
    TForm: DeserializeOwned + Validate,
    TAnswer: Future<Output = Result<ResponseContinuation, HandlerError>>,
{
    match read_form_fields(request, body, FORM_LIMIT)
        .await
        .into_requirement()
    {
        Requirement::Met(fields) => answer(validate(&fields)).await,
        Requirement::Unmet(refusal) => Ok(refusal),
    }
}
