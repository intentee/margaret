use serde::de::DeserializeOwned;
use validator::Validate;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::read_form_fields::read_form_fields;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::requirement::Requirement;
use margaret_http::response::Response;
use margaret_validation::validate::validate;
use margaret_validation::validation_result::ValidationResult;

/// # Errors
///
/// Returns `HandlerError::Consumer` carrying the error the answer reported.
pub async fn responded_to_form<TForm, TAnswer, TError>(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
    answer: impl FnOnce(ValidationResult<TForm>) -> TAnswer,
) -> Result<Response, HandlerError>
where
    TForm: DeserializeOwned + Validate,
    TAnswer: Future<Output = Result<Response, TError>>,
    TError: Into<anyhow::Error>,
{
    match read_form_fields(request, body, limit)
        .await
        .into_requirement()
    {
        Requirement::Met(fields) => answer(validate(&fields))
            .await
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

    async fn answered(content: &'static [u8], limit: usize) -> Result<Response, HandlerError> {
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
    async fn hands_a_valid_form_to_the_answer_and_responds_with_what_it_returns() {
        assert!(matches!(
            answered(b"name=ada", 64).await,
            Ok(response) if response.status() == 200
        ));
    }

    #[tokio::test]
    async fn reports_the_failure_of_the_answer_as_a_consumer_error_so_its_cause_is_kept() {
        assert!(matches!(
            answered(b"other=ada", 64).await,
            Err(HandlerError::Consumer { source }) if source.to_string() == "the greeting is not valid"
        ));
    }

    #[tokio::test]
    async fn refuses_a_form_larger_than_its_limit() {
        assert!(matches!(
            answered(b"name=ada", 4).await,
            Ok(response) if response.status() == 413
        ));
    }
}
