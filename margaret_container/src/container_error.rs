use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(transparent)]
    ServeInput {
        #[from]
        source: ServeInputCodegenError,
    },

    #[error("#[{attribute}] is only supported on structs, but '{path}' is not a struct")]
    DeclarationNotAStruct {
        attribute: &'static str,
        path: String,
    },

    #[error("the #[{attribute}] singleton '{path}' must not also carry a role attribute")]
    ConflictingDeclarationRole {
        attribute: &'static str,
        path: String,
    },

    #[error("#[singleton] does not take any arguments, but '{path}' declares some")]
    SingletonHasArguments { path: String },

    #[error("'{path}' carries #[{attribute}], so it must also be declared as a #[singleton]")]
    DeclarationRequiresSingleton {
        attribute: &'static str,
        path: String,
    },

    #[error("the #[{attribute}] singleton '{path}' does not implement {required}")]
    DeclarationMissingTrait {
        attribute: &'static str,
        path: String,
        required: String,
    },

    #[error(
        "the framework provider '{provider}' stamps issued tokens, but no struct declares #[issues_tokens]"
    )]
    MissingTokenIssuance { provider: String },

    #[error(
        "parameter '{parameter}' of singleton '{singleton}' injects '{provider}', which only the framework may inject"
    )]
    FrameworkOnlyProvider {
        parameter: String,
        provider: String,
        singleton: String,
    },

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

    #[error("the container plan does not contain provider '{path}'")]
    MissingPlannedProvider { path: String },

    #[error("the container plan does not contain the serve inputs of '{path}'")]
    MissingProviderServeInputs { path: String },

    #[error("the container plan does not contain serve input slot '{key}'")]
    MissingServeInputSlot { key: String },
}
