use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_oauth_vocabulary::client_id_rejection::ClientIdRejection;

#[derive(Debug, Error)]
pub enum OAuthClientCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error("the client_id #[oauth_client] declares on '{anchor}' is malformed: {rejection}")]
    MalformedClientId {
        anchor: String,
        rejection: ClientIdRejection,
    },

    #[error(
        "#[oauth_client] on '{anchor}' reads its client secret from '{name}', which is not an environment variable name"
    )]
    MalformedClientSecretSource { anchor: String, name: String },

    #[error(
        "#[oauth_client] on '{anchor}' must name the client the provider admits it as by a plain tag"
    )]
    MalformedAdmittedClient { anchor: String },

    #[error("#[oauth_client] on '{anchor}' must name its issuer as `issuer = <tag>`")]
    MalformedIssuer { anchor: String },

    #[error("#[oauth_client] on '{anchor}' names a tag that is not a single plain name")]
    MalformedTag { anchor: String },

    #[error("#[oauth_client] on '{anchor}' does not declare how the client authenticates")]
    MissingAuthentication { anchor: String },

    #[error("#[oauth_client] on '{anchor}' does not declare its client_id")]
    MissingClientId { anchor: String },

    #[error(
        "#[oauth_client] on '{anchor}' authenticates with ClientAuthenticationMethod::ClientSecretBasic, but does not name the environment variable it reads the secret from"
    )]
    MissingClientSecretSource { anchor: String },

    #[error("#[oauth_client] on '{anchor}' signs in without declaring the scopes it requests")]
    MissingSignInScopes { anchor: String },

    #[error("#[oauth_client] on '{anchor}' does not name a tag")]
    MissingTag { anchor: String },

    #[error(
        "#[oauth_client] on '{anchor}' authenticates with '{method}', but an oauth client authenticates with ClientAuthenticationMethod::PrivateKeyJwt or ClientAuthenticationMethod::ClientSecretBasic"
    )]
    UnsupportedAuthentication {
        anchor: String,
        method: &'static str,
    },

    #[error(
        "#[oauth_client] on '{anchor}' authenticates with '{written}', which is not a variant of margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod"
    )]
    UnknownAuthentication { anchor: String, written: String },

    #[error(
        "#[oauth_client] on '{anchor}' signs in with the scope '{written}', which no #[oauth_scope] declares and which is not margaret::framework::oauth_vocabulary::openid_scope::OpenidScope"
    )]
    UnknownSignInScope { anchor: String, written: String },
}
