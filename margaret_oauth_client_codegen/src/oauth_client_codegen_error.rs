use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;

#[derive(Debug, Error)]
pub enum OAuthClientCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error("the authentication #[oauth_client] declares on '{anchor}' is malformed: {source}")]
    MalformedAuthentication {
        anchor: String,
        #[source]
        source: OAuthVocabularyError,
    },

    #[error("the client_id #[oauth_client] declares on '{anchor}' is malformed: {source}")]
    MalformedClientId {
        anchor: String,
        #[source]
        source: OAuthVocabularyError,
    },

    #[error(
        "#[oauth_client] on '{anchor}' reads its client secret from '{name}', which is not an environment variable name"
    )]
    MalformedClientSecretSource { anchor: String, name: String },

    #[error("#[oauth_client] on '{anchor}' must name its issuer as `issuer = <tag>`")]
    MalformedIssuer { anchor: String },

    #[error("#[oauth_client] on '{anchor}' names a tag that is not a single plain name")]
    MalformedTag { anchor: String },

    #[error("#[oauth_client] on '{anchor}' does not declare how the client authenticates")]
    MissingAuthentication { anchor: String },

    #[error("#[oauth_client] on '{anchor}' does not declare its client_id")]
    MissingClientId { anchor: String },

    #[error(
        "#[oauth_client] on '{anchor}' authenticates with client_secret_basic, but does not name the environment variable it reads the secret from"
    )]
    MissingClientSecretSource { anchor: String },

    #[error("#[oauth_client] on '{anchor}' does not name a tag")]
    MissingTag { anchor: String },

    #[error(
        "#[oauth_client] on '{anchor}' authenticates with '{method}', but an oauth client authenticates with private_key_jwt or client_secret_basic"
    )]
    UnsupportedAuthentication {
        anchor: String,
        method: &'static str,
    },
}
