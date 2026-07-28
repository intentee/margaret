use serde::de::DeserializeOwned;
use validator::Validate;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_validation::validation_result::ValidationResult;

use crate::request_input::RequestInput;
use crate::validate_input::validate_input;

pub fn require_input<Model>(request: &Request, source: RequestInput) -> Result<Model, Response>
where
    Model: DeserializeOwned + Validate,
{
    match validate_input(request, source) {
        ValidationResult::Valid(model) => Ok(model),
        ValidationResult::Invalid(errors) => Err(Response::text(422, errors.to_string())),
        ValidationResult::Malformed(malformation) => {
            Err(Response::text(400, malformation.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use http::Method;
    use tokio_util::sync::CancellationToken;

    use margaret_http::request::Request;

    use super::require_input;
    use crate::request_input::RequestInput;

    #[derive(serde::Deserialize, validator::Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        value: String,
    }

    fn request_with_form(form: HashMap<String, String>) -> Request {
        let mut request = Request::new(Method::POST, "/".to_string(), CancellationToken::new());
        request.inputs.form = form;
        request
    }

    #[test]
    fn returns_the_model_when_valid() {
        let request = request_with_form(HashMap::from([("value".to_string(), "ok".to_string())]));

        assert_eq!(
            require_input::<Sample>(&request, RequestInput::Form)
                .ok()
                .map(|model| model.value),
            Some("ok".to_string())
        );
    }

    #[test]
    fn responds_with_422_when_invalid() {
        let request = request_with_form(HashMap::from([("value".to_string(), String::new())]));

        assert_eq!(
            require_input::<Sample>(&request, RequestInput::Form)
                .err()
                .map(|response| response.status()),
            Some(422)
        );
    }

    #[test]
    fn responds_with_400_when_malformed() {
        let request = request_with_form(HashMap::new());

        assert_eq!(
            require_input::<Sample>(&request, RequestInput::Form)
                .err()
                .map(|response| response.status()),
            Some(400)
        );
    }
}
