use serde::de::DeserializeOwned;
use serde_json::Value;
use validator::Validate;
use validator::ValidationErrors;

use crate::malformation::Malformation;

#[derive(Debug)]
pub enum ValidationResult<Model> {
    Valid(Model),
    Invalid(ValidationErrors),
    Malformed(Malformation),
}

impl<Model> ValidationResult<Model>
where
    Model: DeserializeOwned + Validate,
{
    pub(crate) fn from_value(value: &Value) -> Self {
        match Model::deserialize(value) {
            Ok(model) => Self::from_model(model),
            Err(_) => Self::Malformed(Malformation::Unreadable),
        }
    }
}

impl<Model> ValidationResult<Model>
where
    Model: Validate,
{
    pub(crate) fn from_model(model: Model) -> Self {
        match model.validate() {
            Ok(()) => Self::Valid(model),
            Err(errors) => Self::Invalid(errors),
        }
    }
}
