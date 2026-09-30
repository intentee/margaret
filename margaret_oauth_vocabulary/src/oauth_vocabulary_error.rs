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

    #[error("the client secret contains a character outside the visible ascii range")]
    ClientSecretInvisibleCharacter,

    #[error("the client secret is empty")]
    EmptyClientSecret,

    #[error("the scope is empty")]
    EmptyScope,

    #[error("the scope contains a character outside the scope token grammar of RFC 6749")]
    ScopeCharacter,
}
