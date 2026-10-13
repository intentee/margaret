use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

use crate::validation_result::ValidationResult;

#[must_use]
pub fn validate_json<Model>(value: &Value) -> ValidationResult<Model>
where
    Model: DeserializeOwned + Validate,
{
    ValidationResult::from_value(value)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::Value;
    use serde_json::json;
    use validator::Validate;

    use super::validate_json;
    use crate::validation_result::ValidationResult;

    #[derive(Debug, Deserialize, Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        required: String,
        #[validate(length(min = 1))]
        optional: Option<String>,
    }

    fn outcome(value: &Value) -> Result<Sample, &'static str> {
        match validate_json::<Sample>(value) {
            ValidationResult::Valid(sample) => Ok(sample),
            ValidationResult::Invalid(_) => Err("invalid"),
            ValidationResult::Malformed(_) => Err("malformed"),
        }
    }

    #[test]
    fn builds_the_model_from_a_json_body_that_passes_every_rule_so_the_handler_gets_its_values() {
        let sample =
            outcome(&json!({ "required": "here", "optional": "present" })).expect("a valid body");

        assert_eq!(sample.required, "here");
        assert_eq!(sample.optional.as_deref(), Some("present"));
    }

    #[test]
    fn reports_a_rule_violation_as_invalid_not_malformed_so_users_can_correct_it() {
        assert_eq!(
            outcome(&json!({ "required": "" })).expect_err("an invalid body"),
            "invalid"
        );
    }

    #[test]
    fn reports_an_uninterpretable_body_as_malformed_since_no_model_can_be_built() {
        assert_eq!(
            outcome(&json!({})).expect_err("an uninterpretable body"),
            "malformed"
        );
    }
}
