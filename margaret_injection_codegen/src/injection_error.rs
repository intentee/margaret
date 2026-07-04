use thiserror::Error;

#[derive(Debug, Error)]
pub enum InjectionError {
    #[error("'{item}' has more than one #[process] method: {methods}")]
    AmbiguousProcessMethod { item: String, methods: String },

    #[error("'{item}' has no #[process] method")]
    MissingProcessMethod { item: String },
}
