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
    fn accepts_valid_fields() {
        let sample =
            outcome(&json!({ "required": "here", "optional": "present" })).expect("a valid body");

        assert_eq!(sample.required, "here");
        assert_eq!(sample.optional.as_deref(), Some("present"));
    }

    #[test]
    fn reports_a_rule_violation_as_invalid() {
        assert_eq!(
            outcome(&json!({ "required": "" })).expect_err("an invalid body"),
            "invalid"
        );
    }

    #[test]
    fn reports_an_uninterpretable_body_as_malformed() {
        assert_eq!(
            outcome(&json!({})).expect_err("an uninterpretable body"),
            "malformed"
        );
    }
}
