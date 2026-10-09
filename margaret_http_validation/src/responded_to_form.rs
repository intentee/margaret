use serde::de::DeserializeOwned;
use validator::Validate;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;

/// # Errors
///
/// Returns `HandlerError::Consumer` carrying the error the answer reported.
pub async fn responded_to_form<TForm, TAnswer, TResponse, TError>(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
    answer: impl FnOnce(ValidationResult<TForm>) -> TAnswer,
) -> Result<ResponseContinuation, HandlerError>
where
    TForm: DeserializeOwned + Validate,
    TAnswer: Future<Output = Result<TResponse, TError>>,
    TResponse: Into<ResponseContinuation>,
    TError: Into<anyhow::Error>,
{
    match read_form_fields(request, body, limit)
        .await
        .into_requirement()
    {
        Requirement::Met(fields) => answer(validate(&fields))
            .await
            .map(Into::into)
            .map_err(|error| HandlerError::consumer(error.into())),
        Requirement::Unmet(refusal) => Ok(refusal),
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use http::HeaderValue;
    use http::Method;
    use http::header::CONTENT_TYPE;
    use serde::Deserialize;
    use validator::Validate;

    use margaret_handler_error::handler_error::HandlerError;
    use margaret_http::body_limit::BodyLimit;
    use margaret_http::request::Request;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;
    use margaret_http_tests::fixture_body::fixture_body;
    use margaret_http_tests::fixture_request::FixtureRequest;
    use margaret_validation::validation_result::ValidationResult;

    use super::responded_to_form;

    #[derive(Deserialize, Validate)]
    struct Greeting {
        name: String,
    }

    fn form_request() -> Request {
        let mut fixture = FixtureRequest::new(Method::POST, "/greeting");

        fixture.headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        fixture.into_request()
    }

    async fn answered(
        content: &'static [u8],
        limit: usize,
    ) -> Result<ResponseContinuation, HandlerError> {
        responded_to_form(
            &form_request(),
            fixture_body(content),
            BodyLimit::new(limit),
            async |greeting: ValidationResult<Greeting>| match greeting {
                ValidationResult::Valid(Greeting { name }) => {
                    Ok::<Response, io::Error>(Response::text(200, name))
                }
                ValidationResult::Invalid(_) | ValidationResult::Malformed(_) => {
                    Err(io::Error::other("the greeting is not valid"))
                }
            },
        )
        .await
    }

    #[tokio::test]
    async fn answers_the_validated_form() {
        assert!(matches!(
            answered(b"name=ada", 64).await,
            Ok(ResponseContinuation::Done(response)) if response.status() == 200
        ));
    }

    #[tokio::test]
    async fn reports_the_failure_of_the_answer() {
        assert!(matches!(
            answered(b"other=ada", 64).await,
            Err(HandlerError::Consumer { source }) if source.to_string() == "the greeting is not valid"
        ));
    }

    #[tokio::test]
    async fn refuses_a_form_larger_than_its_limit() {
        assert!(matches!(
            answered(b"name=ada", 4).await,
            Ok(ResponseContinuation::Done(response)) if response.status() == 413
        ));
    }
}
