use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_declaration_anchor::declaration_anchor_error::DeclarationAnchorError;
use margaret_https_url::https_url_error::HttpsUrlError;
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

    #[error("the audience #[{attribute}] declares on '{anchor}' is malformed: {source}")]
    MalformedAudience {
        anchor: String,
        attribute: &'static str,
        #[source]
        source: RegisteredClaimsError,
    },

    #[error("the issuer #[{attribute}] declares on '{anchor}' is malformed: {source}")]
    MalformedIssuer {
        anchor: String,
        attribute: &'static str,
        #[source]
        source: RegisteredClaimsError,
    },

    #[error("the jwks_uri #[provides_jwks_endpoint] declares on '{anchor}' is rejected: {source}")]
    MalformedJwksUri {
        anchor: String,
        #[source]
        source: HttpsUrlError,
    },

    #[error("#[{attribute}] on '{anchor}' names a tag that is not a single plain name")]
    MalformedTag {
        anchor: String,
        attribute: &'static str,
    },

    #[error("#[{attribute}] on '{anchor}' does not declare the audience of the trusted tokens")]
    MissingAudience {
        anchor: String,
        attribute: &'static str,
    },

    #[error("#[{attribute}] on '{anchor}' does not declare the issuer of the trusted tokens")]
    MissingIssuer {
        anchor: String,
        attribute: &'static str,
    },

    #[error("#[provides_jwks_endpoint] on '{anchor}' does not declare the jwks_uri of the issuer")]
    MissingJwksUri { anchor: String },

    #[error("#[{attribute}] on '{anchor}' does not name a tag")]
    MissingTag {
        anchor: String,
        attribute: &'static str,
    },

    #[error(
        "the issuer '{issuer}' is trusted for the audience '{audience}' by both '{first}' and '{second}', so its tokens have no single addressee"
    )]
    TokenTrustDeclaredTwice {
        audience: String,
        first: String,
        issuer: String,
        second: String,
    },
}
