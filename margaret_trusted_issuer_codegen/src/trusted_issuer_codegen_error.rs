use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_https_url::https_url_rejection::HttpsUrlRejection;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[derive(Debug, Error)]
pub enum TrustedIssuerCodegenError {
    #[error(transparent)]
    Anchor(#[from] DeclarationAnchorError),

    #[error(transparent)]
    AttributeArguments(#[from] AttributeArgumentsError),

    #[error(
        "the issuers '{first}' and '{second}' publish their metadata at the same discovery location '{discovery_url}', so at most one of them can match it"
    )]
    DistinctIssuersShareDiscovery {
        discovery_url: String,
        first: String,
        second: String,
    },

    #[error(transparent)]
    Index(#[from] AttributeError),

    #[error(
        "the issuer '{issuer}' is trusted through a jwks endpoint by '{first}' and by '{second}' as well, so its key set has no single source"
    )]
    JwksEndpointIssuerTrustedTwice {
        first: String,
        issuer: String,
        second: String,
    },

    #[error("#[verifies_tokens_from_issuer] on '{anchor}' declares an empty audience")]
    EmptyAudience { anchor: String },

    #[error(
        "the issuer #[verifies_tokens_from_issuer] declares on '{anchor}' is malformed: {source}"
    )]
    MalformedIssuer {
        anchor: String,
        #[source]
        source: RegisteredClaimsError,
    },

    #[error(
        "the jwks_uri #[verifies_tokens_from_issuer] declares on '{anchor}' is rejected: {rejection}"
    )]
    MalformedJwksUri {
        anchor: String,
        rejection: HttpsUrlRejection,
    },

    #[error(
        "#[verifies_tokens_from_issuer] on '{anchor}' names a tag that is not a single plain name"
    )]
    MalformedTag { anchor: String },

    #[error(
        "#[verifies_tokens_from_issuer] on '{anchor}' does not declare the audience of the verified tokens"
    )]
    MissingAudience { anchor: String },

    #[error(
        "#[verifies_tokens_from_issuer] on '{anchor}' does not declare the issuer of the verified tokens"
    )]
    MissingIssuer { anchor: String },

    #[error(
        "#[verifies_tokens_from_issuer] on '{anchor}' does not declare where the keys of the issuer come from as a variant of margaret::framework::trusted_issuer::issuer_keys::IssuerKeys"
    )]
    MissingIssuerKeys { anchor: String },

    #[error(
        "#[verifies_tokens_from_issuer] on '{anchor}' publishes its keys without declaring their jwks_uri"
    )]
    MissingJwksUri { anchor: String },

    #[error("#[verifies_tokens_from_issuer] on '{anchor}' does not name a tag")]
    MissingTag { anchor: String },

    #[error(
        "the issuer '{issuer}' is trusted for the audience '{audience}' by both '{first}' and '{second}', so its tokens have no single addressee"
    )]
    TokenTrustDeclaredTwice {
        audience: String,
        first: String,
        issuer: String,
        second: String,
    },

    #[error(
        "#[verifies_tokens_from_issuer] on '{anchor}' declares the keys '{written}', which is not a variant of margaret::framework::trusted_issuer::issuer_keys::IssuerKeys"
    )]
    UnknownIssuerKeys { anchor: String, written: String },
}
