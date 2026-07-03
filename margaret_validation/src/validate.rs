use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::de::value::Error as DeserializeError;
use serde::de::value::MapDeserializer;
use validator::Validate;

use crate::validation_result::ValidationResult;

pub fn validate<Model>(data: &HashMap<String, String>) -> ValidationResult<Model>
where
    Model: DeserializeOwned + Validate,
{
    let deserialized: Result<Model, DeserializeError> = Model::deserialize(MapDeserializer::new(
        data.iter()
            .map(|(name, value)| (name.as_str(), value.as_str())),
    ));

    match deserialized {
        Ok(model) => ValidationResult::from_model(model),
        Err(source) => ValidationResult::from_deserialize_error(source),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use validator::ValidationErrors;

    use super::validate;
    use crate::validation_result::ValidationResult;

    #[derive(Debug, serde::Deserialize, validator::Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        value: String,
    }

    fn outcome(data: &HashMap<String, String>) -> Result<Sample, ValidationErrors> {
        match validate::<Sample>(data) {
            ValidationResult::Valid(sample) => Ok(sample),
            ValidationResult::Invalid(errors) => Err(errors),
        }
    }

    #[test]
    fn maps_and_accepts_a_valid_form() {
        let data = HashMap::from([("value".to_string(), "hello".to_string())]);

        assert_eq!(outcome(&data).expect("a valid form").value, "hello");
    }

    #[test]
    fn reports_a_rule_violation_as_invalid() {
        let data = HashMap::from([("value".to_string(), String::new())]);

        assert!(
            outcome(&data)
                .expect_err("an invalid form")
                .errors()
                .contains_key("value")
        );
    }

    #[test]
    fn reports_a_missing_field_as_invalid() {
        let data = HashMap::new();

        assert!(!outcome(&data).expect_err("an invalid form").is_empty());
    }
}
