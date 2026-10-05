use std::sync::Arc;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::localhost_trust::localhost_trust;

#[must_use]
pub fn localhost_oidc_issuer() -> Arc<TrustedIssuer> {
    Arc::new(TrustedIssuer::for_oidc_issuer(
        Arc::new(IssuerMetadata::awaiting()),
        Arc::new(TokenTrustDeclaration {
            trust: localhost_trust(),
        }),
    ))
}
