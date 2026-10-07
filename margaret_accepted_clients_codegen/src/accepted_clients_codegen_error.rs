use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_https_url::https_url_error::HttpsUrlError;
use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[derive(Debug, Error)]
pub enum AcceptedClientsCodegenError {
    #[error(
        "#[accepts_oauth_client] on '{anchor}' accepts a client, but no struct declares #[issues_tokens] for the provider"
    )]
    AcceptedWithoutTokenIssuance { anchor: String },

    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error("the client '{client_id}' is accepted by both '{first}' and '{second}'")]
    DuplicateClientId {
        client_id: String,
        first: String,
        second: String,
    },

    #[error("#[accepts_oauth_client] on '{anchor}' declares an empty list of redirect_uris")]
    EmptyRedirectUris { anchor: String },

    #[error("#[accepts_oauth_client] on '{anchor}' declares an empty list of resources")]
    EmptyResources { anchor: String },

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(
        "#[accepts_oauth_client] on '{anchor}' redirects to '{redirect_uri}', which is neither https nor http to a loopback ip address"
    )]
    InsecureRedirectUri {
        anchor: String,
        redirect_uri: String,
    },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' names the client '{client_id}', which is the issuer of the provider itself"
    )]
    IssuerClientId { anchor: String, client_id: String },

    #[error(
        "the authentication #[accepts_oauth_client] declares on '{anchor}' is malformed: {source}"
    )]
    MalformedAuthentication {
        anchor: String,
        #[source]
        source: OAuthVocabularyError,
    },

    #[error(
        "a client_credentials scope #[accepts_oauth_client] declares on '{anchor}' is malformed: {source}"
    )]
    MalformedClientCredentialsScope {
        anchor: String,
        #[source]
        source: OAuthVocabularyError,
    },

    #[error("the client_id #[accepts_oauth_client] declares on '{anchor}' is malformed: {source}")]
    MalformedClientId {
        anchor: String,
        #[source]
        source: OAuthVocabularyError,
    },

    #[error(
        "an authorization_code scope #[accepts_oauth_client] declares on '{anchor}' is malformed: {source}"
    )]
    MalformedCodeScope {
        anchor: String,
        #[source]
        source: OAuthVocabularyError,
    },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' declares the consent '{keyword}', but the consent is prompted or implicit"
    )]
    MalformedConsent { anchor: String, keyword: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' declares the id_token_signing '{keyword}', but id tokens are signed with rsa or elliptic_curve"
    )]
    MalformedIdTokenSigning { anchor: String, keyword: String },

    #[error("the jwks_uri #[accepts_oauth_client] declares on '{anchor}' is malformed: {source}")]
    MalformedJwksUri {
        anchor: String,
        #[source]
        source: HttpsUrlError,
    },

    #[error(
        "the redirect uri '{redirect_uri}' #[accepts_oauth_client] declares on '{anchor}' is malformed: {source}"
    )]
    MalformedRedirectUri {
        anchor: String,
        redirect_uri: String,
        #[source]
        source: url::ParseError,
    },

    #[error("a resource #[accepts_oauth_client] declares on '{anchor}' is malformed: {source}")]
    MalformedResource {
        anchor: String,
        #[source]
        source: RegisteredClaimsError,
    },

    #[error("#[accepts_oauth_client] on '{anchor}' does not declare how the client authenticates")]
    MissingAuthentication { anchor: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' grants client_credentials without declaring their scopes"
    )]
    MissingClientCredentialsScopes { anchor: String },

    #[error("#[accepts_oauth_client] on '{anchor}' does not declare its client_id")]
    MissingClientId { anchor: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' grants authorization_code without declaring its scopes"
    )]
    MissingCodeScopes { anchor: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' grants authorization_code without declaring its consent"
    )]
    MissingConsent { anchor: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' grants authorization_code without declaring its id_token_signing"
    )]
    MissingIdTokenSigning { anchor: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' authenticates with private_key_jwt without declaring its jwks_uri"
    )]
    MissingJwksUri { anchor: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' grants authorization_code without declaring its redirect_uris"
    )]
    MissingRedirectUris { anchor: String },

    #[error("#[accepts_oauth_client] on '{anchor}' does not declare its resources")]
    MissingResources { anchor: String },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' declares the redirect uri '{redirect_uri}' with a fragment"
    )]
    RedirectUriHasFragment {
        anchor: String,
        redirect_uri: String,
    },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' declares the redirect uri '{redirect_uri}', which must be written as '{canonical}'"
    )]
    RedirectUriNotCanonical {
        anchor: String,
        canonical: String,
        redirect_uri: String,
    },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' names the session audience '{audience}' as one of its resources"
    )]
    SessionAudienceResource { anchor: String, audience: String },

    #[error(
        "the clients accepted by '{first}' and '{second}' share the jwks_uri '{jwks_uri}', so each could impersonate the other"
    )]
    SharedClientJwksUri {
        first: String,
        jwks_uri: String,
        second: String,
    },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' authenticates with '{method}', but an accepted client authenticates with private_key_jwt or none"
    )]
    UnsupportedAuthentication {
        anchor: String,
        method: &'static str,
    },

    #[error(
        "#[accepts_oauth_client] on '{anchor}' names the client '{client_id}', which is a uuid that could be mistaken for the subject of a user"
    )]
    UuidClientId { anchor: String, client_id: String },
}
