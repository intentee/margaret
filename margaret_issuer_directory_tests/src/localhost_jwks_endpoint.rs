use std::sync::Arc;

use url::Url;

use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

/// # Panics
///
/// Panics when the fixture key set url is malformed.
#[must_use]
pub fn localhost_jwks_endpoint(trust: TokenTrust) -> Arc<TrustedIssuer> {
    Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(StaticEndpoint::new(
            Url::parse("https://localhost/jwks").expect("the fixture key set url parses"),
        )),
        Arc::new(TokenTrustDeclaration { trust }),
    ))
}
