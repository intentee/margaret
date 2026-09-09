use serde::de::DeserializeOwned;
use validator::Validate;

use margaret_http::request::Request;
use margaret_validation::validation_result::ValidationResult;

use crate::request_input::RequestInput;

#[must_use]
pub fn validate_input<Model>(request: &Request, source: RequestInput) -> ValidationResult<Model>
where
    Model: DeserializeOwned + Validate,
{
    match source {
        RequestInput::Cookie => margaret_validation::validate::validate(&request.inputs.cookies),
        RequestInput::Form => margaret_validation::validate::validate(request.inputs.body.form()),
        RequestInput::Query => margaret_validation::validate::validate(&request.inputs.query),
        RequestInput::Json => {
            margaret_validation::validate_json::validate_json(request.inputs.body.json())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use http::Method;
    use serde_json::json;

    use margaret_http::request::Request;
    use margaret_http::request_body_inputs::RequestBodyInputs;
    use margaret_validation::validation_result::ValidationResult;

    use super::validate_input;
    use crate::request_input::RequestInput;

    #[derive(Debug, serde::Deserialize, validator::Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        value: String,
    }

    fn value(result: ValidationResult<Sample>) -> Option<String> {
        match result {
            ValidationResult::Valid(sample) => Some(sample.value),
            ValidationResult::Invalid(_) | ValidationResult::Malformed(_) => None,
        }
    }

    fn request() -> Request {
        Request::new(Method::GET, "/".to_string())
    }

    #[test]
    fn validates_the_form_source() {
        let mut request = request();
        request.inputs.body = RequestBodyInputs::UrlEncoded(HashMap::from([(
            "value".to_string(),
            "formed".to_string(),
        )]));

        assert_eq!(
            value(validate_input(&request, RequestInput::Form)),
            Some("formed".to_string())
        );
    }

    #[test]
    fn validates_the_query_source() {
        let mut request = request();
        request.inputs.query = HashMap::from([("value".to_string(), "queried".to_string())]);

        assert_eq!(
            value(validate_input(&request, RequestInput::Query)),
            Some("queried".to_string())
        );
    }

    #[test]
    fn validates_the_cookie_source() {
        let mut request = request();
        request.inputs.cookies = HashMap::from([("value".to_string(), "baked".to_string())]);

        assert_eq!(
            value(validate_input(&request, RequestInput::Cookie)),
            Some("baked".to_string())
        );
    }

    #[test]
    fn validates_the_json_source() {
        let mut request = request();
        request.inputs.body = RequestBodyInputs::Json(json!({ "value": "jsoned" }));

        assert_eq!(
            value(validate_input(&request, RequestInput::Json)),
            Some("jsoned".to_string())
        );
    }

    #[test]
    fn reports_invalid_source_data() {
        let mut request = request();
        request.inputs.body =
            RequestBodyInputs::UrlEncoded(HashMap::from([("value".to_string(), String::new())]));

        assert_eq!(value(validate_input(&request, RequestInput::Form)), None);
    }
}
