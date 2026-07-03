use std::borrow::Cow;
use std::fmt::Display;

use validator::Validate;
use validator::ValidationError;
use validator::ValidationErrors;

const DESERIALIZE_FIELD: &str = "_input";

fn deserialize_errors<Source>(source: Source) -> ValidationErrors
where
    Source: Display,
{
    let mut errors = ValidationErrors::new();

    errors.add(
        DESERIALIZE_FIELD,
        ValidationError::new("deserialize").with_message(Cow::Owned(source.to_string())),
    );

    errors
}

#[derive(Debug)]
pub enum ValidationResult<Model> {
    Valid(Model),
    Invalid(ValidationErrors),
}

impl<Model> ValidationResult<Model> {
    pub(crate) fn from_deserialize_error<Source>(source: Source) -> Self
    where
        Source: Display,
    {
        Self::Invalid(deserialize_errors(source))
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
