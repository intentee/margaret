use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;
use margaret_input_weaving::input_weaving_error::InputWeavingError;

#[derive(Debug, Error)]
pub enum ConsoleArgumentCodegenError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

    #[error(
        "console argument {site} declares both `from` and `positional`; an argument is either named or positional, never both"
    )]
    NamedAndPositional { site: ConstructorParameter },

    #[error(
        "console argument {site} declares neither `from = \"...\"` nor `positional`; it must be exactly one of them"
    )]
    NeitherNamedNorPositional { site: ConstructorParameter },

    #[error(
        "console argument {site} is `positional` but its type is a boolean; a boolean is a named flag, never positional"
    )]
    BooleanPositional { site: ConstructorParameter },

    #[error(
        "console argument {site} is `positional`, but its owner is not a #[console_command]; positional arguments are only allowed on console commands"
    )]
    PositionalOutsideCommand { site: ConstructorParameter },

    #[error(transparent)]
    ValueType {
        #[from]
        source: InputWeavingError,
    },
}
