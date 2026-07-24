use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
use margaret_tag_codegen::tag_error::TagError;

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(transparent)]
    ConsoleArgument {
        #[from]
        source: ConsoleArgumentCodegenError,
    },

    #[error(transparent)]
    Tag {
        #[from]
        source: TagError,
    },

    #[error("#[provides_endpoint] is only supported on structs, but '{path}' is not a struct")]
    NotAnEndpointStruct { path: String },

    #[error("the endpoint provider '{path}' must not also carry a role attribute")]
    ConflictingEndpointRole { path: String },

    #[error("#[singleton] does not take any arguments, but '{path}' declares some")]
    SingletonHasArguments { path: String },

    #[error("the endpoint provider '{path}' must also be declared as a #[singleton]")]
    EndpointProviderRequiresSingleton { path: String },

    #[error(
        "the endpoint provider '{path}' does not implement margaret_endpoint::provides_endpoint::ProvidesEndpoint"
    )]
    EndpointProviderMissingTrait { path: String },

    #[error("#[singleton] is only supported on structs, but '{path}' is not a struct")]
    NotASingletonStruct { path: String },

    #[error("a role attribute is only supported on structs, but '{path}' is not a struct")]
    RoleNotAStruct { path: String },

    #[error(
        "singleton '{singleton}' has {field_count} field(s) but no #[constructor] method; only fieldless singletons may omit a #[constructor]"
    )]
    SingletonRequiresConstructor {
        singleton: String,
        field_count: usize,
    },

    #[error("singleton '{singleton}' has more than one #[constructor] method: {methods}")]
    AmbiguousConstructor { singleton: String, methods: String },

    #[error("the #[constructor] of singleton '{singleton}' must return Self")]
    ConstructorReturnTypeMismatch { singleton: String },

    #[error(
        "the framework-provided injectable '{path}' collides with a singleton declared at the same path"
    )]
    AmbiguousFrameworkProvider { path: String },

    #[error(
        "parameter '{parameter}' of singleton '{singleton}' has an unsupported type '{written}'"
    )]
    UnsupportedParameterShape {
        singleton: String,
        parameter: String,
        written: String,
    },

    #[error(
        "parameter '{parameter}' of singleton '{singleton}' depends on '{written}', which no singleton provides"
    )]
    MissingProvider {
        singleton: String,
        parameter: String,
        written: String,
    },

    #[error("dependency cycle detected: {path}")]
    DependencyCycle { path: String },
}
