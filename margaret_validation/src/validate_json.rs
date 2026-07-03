use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;

use crate::malformation::Malformation;
use crate::validation_result::ValidationResult;

pub fn validate_json<Model>(data: Option<&Value>) -> ValidationResult<Model>
where
    Model: DeserializeOwned + Validate,
{
    match data {
        Some(value) => ValidationResult::from_value(value),
        None => ValidationResult::Malformed(Malformation::Absent),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use serde_json::json;

    use super::validate_json;
    use crate::malformation::Malformation;
    use crate::validation_result::ValidationResult;

    #[derive(Debug, serde::Deserialize, validator::Validate)]
    struct Sample {
        #[validate(length(min = 1))]
        required: String,
        #[validate(length(min = 1))]
        optional: Option<String>,
    }

    fn outcome(data: Option<&Value>) -> Result<Sample, &'static str> {
        match validate_json::<Sample>(data) {
            ValidationResult::Valid(sample) => Ok(sample),
            ValidationResult::Invalid(_) => Err("invalid"),
            ValidationResult::Malformed(Malformation::Absent) => Err("absent"),
            ValidationResult::Malformed(Malformation::Unreadable) => Err("unreadable"),
        }
    }

    #[test]
    fn accepts_valid_fields() {
        let body = json!({ "required": "here", "optional": "present" });
        let sample = outcome(Some(&body)).expect("a valid body");

        assert_eq!(sample.required, "here");
        assert_eq!(sample.optional.as_deref(), Some("present"));
    }

    #[test]
    fn reports_a_rule_violation_as_invalid() {
        let body = json!({ "required": "" });

        assert_eq!(
            outcome(Some(&body)).expect_err("an invalid body"),
            "invalid"
        );
    }

    #[test]
    fn reports_an_absent_body_as_malformed() {
        assert_eq!(outcome(None).expect_err("an absent body"), "absent");
    }

    #[test]
    fn reports_an_unreadable_body_as_malformed() {
        let body = json!({});

        assert_eq!(
            outcome(Some(&body)).expect_err("an unreadable body"),
            "unreadable"
        );
    }
}
