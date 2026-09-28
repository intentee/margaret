use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegisteredClaimsError {
    #[error("the audience is empty")]
    AudienceEmpty,

    #[error("the issuer identifier '{original}' has a fragment")]
    IssuerHasFragment { original: String },

    #[error("the issuer identifier '{original}' has a query")]
    IssuerHasQuery { original: String },

    #[error("the issuer identifier '{original}' is not a url: {source}")]
    IssuerMalformed {
        original: String,
        #[source]
        source: url::ParseError,
    },

    #[error("the issuer identifier '{original}' uses the '{scheme}' scheme instead of https")]
    IssuerNotHttps { original: String, scheme: String },
}
