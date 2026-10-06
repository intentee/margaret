use std::str::Utf8Error;

use thiserror::Error;

use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[derive(Debug, Error)]
pub enum OAuthVocabularyError {
    #[error("the client identifier contains a character outside the visible ascii range")]
    ClientIdInvisibleCharacter,

    #[error("the client identifier cannot be a token audience: {source}")]
    ClientIdNotAnAudience {
        #[source]
        source: RegisteredClaimsError,
    },

    #[error("the form-encoded basic client credentials do not decode to utf-8: {source}")]
    ClientSecretBasicNotUtf8 {
        #[source]
        source: Utf8Error,
    },

    #[error("the client secret contains a character outside the visible ascii range")]
    ClientSecretInvisibleCharacter,

    #[error("the client secret is empty")]
    EmptyClientSecret,

    #[error("the scope is empty")]
    EmptyScope,

    #[error("the openid scope asks for an identity, so it cannot grant access to a resource")]
    OpenidResourceScope,

    #[error("the scope contains a character outside the scope token grammar of RFC 6749")]
    ScopeCharacter,

    #[error("the code challenge method '{value}' is not supported")]
    UnsupportedCodeChallengeMethod { value: String },

    #[error("the prompt value '{value}' is not supported")]
    UnsupportedPromptValue { value: String },

    #[error("the token type '{value}' is not supported")]
    UnsupportedTokenType { value: String },
}
