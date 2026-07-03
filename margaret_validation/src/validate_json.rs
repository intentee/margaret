use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

use crate::validation_result::ValidationResult;

pub fn validate_json<Model>(data: &Value) -> ValidationResult<Model>
where
    Model: DeserializeOwned + Validate,
{
    match Model::deserialize(data) {
        Ok(model) => ValidationResult::from_model(model),
        Err(source) => ValidationResult::from_deserialize_error(source),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;
    use validator::ValidationErrors;

    use super::validate_json;
    use crate::validation_result::ValidationResult;

    #[derive(Debug, serde::Deserialize, validator::Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        value: String,
    }

    fn outcome(body: &Value) -> Result<Sample, ValidationErrors> {
        match validate_json::<Sample>(body) {
            ValidationResult::Valid(sample) => Ok(sample),
            ValidationResult::Invalid(errors) => Err(errors),
        }
    }

    #[test]
    fn maps_and_accepts_a_valid_json_body() {
        let body = json!({ "value": "hello" });

        assert_eq!(outcome(&body).expect("a valid body").value, "hello");
    }

    #[test]
    fn reports_a_missing_json_field_as_invalid() {
        let body = json!({});

        assert!(!outcome(&body).expect_err("an invalid body").is_empty());
    }
}
