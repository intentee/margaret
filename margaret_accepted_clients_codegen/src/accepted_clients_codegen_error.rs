use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_https_url::https_url_rejection::HttpsUrlRejection;
use margaret_oauth_vocabulary::client_id_rejection::ClientIdRejection;
use margaret_oauth_vocabulary::resource_scope_rejection::ResourceScopeRejection;

#[derive(Debug, Error)]
pub enum AcceptedClientsCodegenError {
    #[error(
        "#[admits_oauth_client] on '{anchor}' accepts a client, but no struct declares #[issues_tokens] for the provider"
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

    #[error("#[admits_oauth_client] on '{anchor}' names the resource '{tag}' more than once")]
    DuplicateResource { anchor: String, tag: String },

    #[error("#[admits_oauth_client] on '{anchor}' redirects to the route '{route}' more than once")]
    DuplicateRedirectRoute { anchor: String, route: String },

    #[error("#[admits_oauth_client] on '{anchor}' declares an empty list of redirect_routes")]
    EmptyRedirectRoutes { anchor: String },

    #[error("#[admits_oauth_client] on '{anchor}' declares an empty list of redirect_uris")]
    EmptyRedirectUris { anchor: String },

    #[error("#[admits_oauth_client] on '{anchor}' declares an empty list of resources")]
    EmptyResources { anchor: String },

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(
        "#[admits_oauth_client] on '{anchor}' redirects to '{redirect_uri}', which is neither https nor http to a loopback ip address"
    )]
    InsecureRedirectUri {
        anchor: String,
        redirect_uri: String,
    },

    #[error(
        "#[admits_oauth_client] on '{anchor}' names the client '{client_id}', which is the issuer of the provider itself"
    )]
    IssuerClientId { anchor: String, client_id: String },

    #[error(
        "a client_credentials scope #[admits_oauth_client] declares on '{anchor}' is malformed: {rejection}"
    )]
    MalformedClientCredentialsScope {
        anchor: String,
        rejection: ResourceScopeRejection,
    },

    #[error(
        "the client_id #[admits_oauth_client] declares on '{anchor}' is malformed: {rejection}"
    )]
    MalformedClientId {
        anchor: String,
        rejection: ClientIdRejection,
    },

    #[error("the jwks_uri #[admits_oauth_client] declares on '{anchor}' is malformed: {rejection}")]
    MalformedJwksUri {
        anchor: String,
        rejection: HttpsUrlRejection,
    },

    #[error(
        "the redirect uri '{redirect_uri}' #[admits_oauth_client] declares on '{anchor}' is malformed: {source}"
    )]
    MalformedRedirectUri {
        anchor: String,
        redirect_uri: String,
        #[source]
        source: url::ParseError,
    },

    #[error(
        "#[admits_oauth_client] on '{anchor}' names a resource that is not a single plain tag name"
    )]
    MalformedResourceTag { anchor: String },

    #[error("#[admits_oauth_client] on '{anchor}' does not declare how the client authenticates")]
    MissingAuthentication { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' grants client_credentials without declaring their scopes"
    )]
    MissingClientCredentialsScopes { anchor: String },

    #[error("#[admits_oauth_client] on '{anchor}' does not declare its client_id")]
    MissingClientId { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' grants authorization_code without declaring its scopes"
    )]
    MissingCodeScopes { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' grants authorization_code without declaring its consent"
    )]
    MissingConsent { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' grants authorization_code without declaring its id_token_signing"
    )]
    MissingIdTokenSigning { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' trusts keys the client publishes without declaring their jwks_uri"
    )]
    MissingJwksUri { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' grants authorization_code without declaring its redirect_uris or redirect_routes"
    )]
    MissingRedirectUris { anchor: String },

    #[error("#[admits_oauth_client] on '{anchor}' does not declare its resources")]
    MissingResources { anchor: String },

    #[error("#[admits_oauth_client] on '{anchor}' redirects to '{written}', which names no item")]
    UnknownRedirectRoute { anchor: String, written: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' declares the redirect uri '{redirect_uri}' with a fragment"
    )]
    RedirectUriHasFragment {
        anchor: String,
        redirect_uri: String,
    },

    #[error(
        "#[admits_oauth_client] on '{anchor}' declares the redirect uri '{redirect_uri}', which must be written as '{canonical}'"
    )]
    RedirectUriNotCanonical {
        anchor: String,
        canonical: String,
        redirect_uri: String,
    },

    #[error(
        "the clients accepted by '{first}' and '{second}' share the jwks_uri '{jwks_uri}', so each could impersonate the other"
    )]
    SharedClientJwksUri {
        first: String,
        jwks_uri: String,
        second: String,
    },

    #[error(
        "#[admits_oauth_client] on '{anchor}' authenticates with '{method}', but an admitted client authenticates with ClientAuthenticationMethod::PrivateKeyJwt or ClientAuthenticationMethod::None"
    )]
    UnsupportedAuthentication {
        anchor: String,
        method: &'static str,
    },

    #[error(
        "#[admits_oauth_client] on '{anchor}' names the client '{client_id}', which is a uuid that could be mistaken for the subject of a user"
    )]
    UuidClientId { anchor: String, client_id: String },

    #[error("#[admits_oauth_client] on '{anchor}' names a tag that is not a single plain name")]
    MalformedTag { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' authenticates with ClientAuthenticationMethod::PrivateKeyJwt without declaring the keys that verify its assertions as a variant of margaret::framework::accepted_clients::client_keys::ClientKeys"
    )]
    MissingClientKeys { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' trusts keys the client publishes without pinning the algorithm of its assertions as a variant of margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm"
    )]
    MissingSigningAlgorithm { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' pins its assertions to JwsAlgorithm::EdDsa, which does not name its curve (RFC 9864), so pin JwsAlgorithm::Ed25519 instead"
    )]
    PolymorphicSigningAlgorithm { anchor: String },

    #[error("#[admits_oauth_client] on '{anchor}' does not name a tag")]
    MissingTag { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' grants token_exchange without declaring the scopes an exchange can grant"
    )]
    MissingTokenExchangeScopes { anchor: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' authenticates with '{written}', which is not a variant of margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod"
    )]
    UnknownAuthentication { anchor: String, written: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' verifies its assertions with the keys '{written}', which is not a variant of margaret::framework::accepted_clients::client_keys::ClientKeys"
    )]
    UnknownClientKeys { anchor: String, written: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' declares the consent '{written}', which is not a variant of margaret::framework::accepted_clients::consent_policy::ConsentPolicy"
    )]
    UnknownConsent { anchor: String, written: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' signs id tokens with '{written}', which is not a variant of margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning"
    )]
    UnknownIdTokenSigning { anchor: String, written: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' names the resource '{tag}', which no #[issues_resource_tokens] declares"
    )]
    UnknownResource { anchor: String, tag: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' names the scope '{written}', which no #[oauth_scope] declares and which is not margaret::framework::oauth_vocabulary::openid_scope::OpenidScope"
    )]
    UnknownScope { anchor: String, written: String },

    #[error(
        "#[admits_oauth_client] on '{anchor}' pins its assertions to '{written}', which is not a variant of margaret::framework::jose_parameters::jws_algorithm::JwsAlgorithm"
    )]
    UnknownSigningAlgorithm { anchor: String, written: String },
}
