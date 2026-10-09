use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignInFlowError {
    #[error("the sign-in callback '{callback}' is not a url: {source}")]
    MalformedCallback {
        callback: String,
        #[source]
        source: url::ParseError,
    },
}
