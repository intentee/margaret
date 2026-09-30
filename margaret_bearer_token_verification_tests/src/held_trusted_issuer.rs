use std::sync::Arc;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[must_use]
pub fn held_trusted_issuer(trust: TokenTrust, key_set: VerificationKeySet) -> TrustedIssuer {
    let trusted_issuer =
        TrustedIssuer::for_oidc_issuer(Arc::new(IssuerMetadata::awaiting()), Arc::new(trust));

    trusted_issuer.key_set.start_fetch();
    trusted_issuer.key_set.hold(key_set);

    trusted_issuer
}
