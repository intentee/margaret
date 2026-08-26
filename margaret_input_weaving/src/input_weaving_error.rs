use thiserror::Error;

use crate::constructor_parameter::ConstructorParameter;

#[derive(Debug, Error)]
pub enum InputWeavingError {
    #[error(
        "{site} has a generic value type '{value_type}'; an injected input value type must be a single concrete type, never a generic like Vec<T>"
    )]
    GenericValueType {
        site: ConstructorParameter,
        value_type: String,
    },

    #[error(
        "{site} has value type '{value_type}', which could not be resolved to a concrete type; import or fully qualify it"
    )]
    UnresolvableValueType {
        site: ConstructorParameter,
        value_type: String,
    },
}
