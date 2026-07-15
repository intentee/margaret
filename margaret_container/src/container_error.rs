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

    #[error("the provided interface '{written}' of singleton '{singleton}' matches no trait")]
    ProvidesUnresolvable { singleton: String, written: String },

    #[error("the collection trait '{written}' of singleton '{singleton}' matches no trait")]
    CollectionUnresolvable { singleton: String, written: String },

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
