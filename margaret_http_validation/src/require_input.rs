use margaret_http::requirement::Requirement;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_validation::validation_result::ValidationResult;

#[must_use]
pub fn require_input<Model>(validation: ValidationResult<Model>) -> Requirement<Model> {
    match validation {
        ValidationResult::Valid(model) => Requirement::Met(model),
        ValidationResult::Invalid(errors) => Requirement::Unmet(ResponseContinuation::from(
            Response::text(422, errors.to_string()),
        )),
        ValidationResult::Malformed(malformation) => Requirement::Unmet(
            ResponseContinuation::from(Response::text(400, malformation.to_string())),
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde::Deserialize;
    use validator::Validate;

    use margaret_http::requirement::Requirement;
    use margaret_http::response_continuation::ResponseContinuation;
    use margaret_validation::validate::validate;

    use super::require_input;

    #[derive(Deserialize, Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        value: String,
    }

    fn requirement(form: &[(&str, &str)]) -> Requirement<Sample> {
        require_input(validate(
            &form
                .iter()
                .map(|(name, value)| ((*name).to_string(), (*value).to_string()))
                .collect::<HashMap<_, _>>(),
        ))
    }

    fn is_unmet_with_status(requirement: Requirement<Sample>, status: u16) -> bool {
        matches!(
            requirement,
            Requirement::Unmet(ResponseContinuation::Done(response)) if response.status() == status
        )
    }

    #[test]
    fn meets_the_requirement_with_a_valid_model() {
        assert!(matches!(
            requirement(&[("value", "ok")]),
            Requirement::Met(model) if model.value == "ok"
        ));
    }

    #[test]
    fn answers_an_invalid_model_with_unprocessable_content() {
        assert!(is_unmet_with_status(requirement(&[("value", "")]), 422));
    }

    #[test]
    fn answers_a_malformed_model_with_bad_request() {
        assert!(is_unmet_with_status(requirement(&[]), 400));
    }
}
