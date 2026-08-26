use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;
use margaret_input_weaving::input_weaving_error::InputWeavingError;

#[derive(Debug, Error)]
pub enum EnvironmentVariableCodegenError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

    #[error(
        "environment variable {site} declares no `from = \"...\"`; an environment variable is always named by the variable it reads"
    )]
    MissingName { site: ConstructorParameter },

    #[error(
        "environment variable {site} reads '{name}', which is not a usable environment variable name; a name starts with an ASCII letter or an underscore and continues with ASCII letters, digits or underscores"
    )]
    MalformedName {
        name: String,
        site: ConstructorParameter,
    },

    #[error(transparent)]
    ValueType {
        #[from]
        source: InputWeavingError,
    },
}
