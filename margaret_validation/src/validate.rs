use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

use crate::validation_result::ValidationResult;

#[must_use]
pub fn validate<Model>(data: &HashMap<String, String>) -> ValidationResult<Model>
where
    Model: DeserializeOwned + Validate,
{
    let value = Value::Object(
        data.iter()
            .map(|(name, value)| (name.clone(), Value::String(value.clone())))
            .collect(),
    );

    ValidationResult::from_value(&value)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::validate;
    use crate::validation_result::ValidationResult;

    #[derive(Debug, serde::Deserialize, validator::Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        required: String,
        #[validate(length(min = 1))]
        optional: Option<String>,
    }

    fn outcome(data: &HashMap<String, String>) -> Result<Sample, &'static str> {
        match validate::<Sample>(data) {
            ValidationResult::Valid(sample) => Ok(sample),
            ValidationResult::Invalid(_) => Err("invalid"),
            ValidationResult::Malformed(_) => Err("malformed"),
        }
    }

    #[test]
    fn accepts_valid_fields() {
        let data = HashMap::from([
            ("required".to_string(), "here".to_string()),
            ("optional".to_string(), "present".to_string()),
        ]);
        let sample = outcome(&data).expect("a valid form");

        assert_eq!(sample.required, "here");
        assert_eq!(sample.optional.as_deref(), Some("present"));
    }

    #[test]
    fn treats_a_missing_optional_field_as_absent() {
        let data = HashMap::from([("required".to_string(), "here".to_string())]);

        assert_eq!(outcome(&data).expect("a valid form").optional, None);
    }

    #[test]
    fn reports_a_rule_violation_as_invalid() {
        let data = HashMap::from([("required".to_string(), String::new())]);

        assert_eq!(outcome(&data).expect_err("an invalid form"), "invalid");
    }

    #[test]
    fn reports_missing_required_data_as_malformed() {
        let data = HashMap::new();

        assert_eq!(outcome(&data).expect_err("a malformed form"), "malformed");
    }
}
