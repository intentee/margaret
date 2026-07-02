use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("#[singleton] is only supported on structs, but '{path}' is not a struct")]
    NotASingletonStruct { path: String },

    #[error("#[provider] is only supported on structs, but '{path}' is not a struct")]
    ProviderNotAStruct { path: String },

    #[error("#[provider] '{provider}' must declare what it provides via 'provides = ...'")]
    ProviderMissingProvides { provider: String },

    #[error("#[provider] '{provider}' has no #[provide] method")]
    ProviderMissingProvideMethod { provider: String },

    #[error("#[provider] '{provider}' has more than one #[provide] method: {methods}")]
    AmbiguousProvideMethod { provider: String, methods: String },

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

    #[error("two singletons provide the same type '{provided}': '{first}' and '{second}'")]
    DuplicateProvider {
        provided: String,
        first: String,
        second: String,
    },

    #[error("two singletons map to the same container field '{field}': '{first}' and '{second}'")]
    DuplicateFieldName {
        field: String,
        first: String,
        second: String,
    },

    #[error("the provided interface '{written}' of singleton '{singleton}' matches no trait")]
    ProvidesUnresolvable { singleton: String, written: String },

    #[error(
        "the provided interface '{written}' of singleton '{singleton}' is ambiguous: {candidates}"
    )]
    ProvidesAmbiguous {
        singleton: String,
        written: String,
        candidates: String,
    },

    #[error("the collection trait '{written}' of singleton '{singleton}' matches no trait")]
    CollectionUnresolvable { singleton: String, written: String },

    #[error(
        "the collection trait '{written}' of singleton '{singleton}' is ambiguous: {candidates}"
    )]
    CollectionAmbiguous {
        singleton: String,
        written: String,
        candidates: String,
    },

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

    #[error(
        "parameter '{parameter}' of singleton '{singleton}' depends on '{written}', which is ambiguous: {candidates}"
    )]
    AmbiguousReference {
        singleton: String,
        parameter: String,
        written: String,
        candidates: String,
    },

    #[error("dependency cycle detected: {path}")]
    DependencyCycle { path: String },
}
