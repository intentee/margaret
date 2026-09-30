use std::sync::Arc;

use url::Url;

use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::localhost_trust::localhost_trust;

/// # Panics
///
/// Panics when the fixture key set url is malformed.
#[must_use]
pub fn localhost_jwks_endpoint() -> Arc<TrustedIssuer> {
    Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(StaticEndpoint::new(
            Url::parse("https://localhost/jwks").expect("the fixture key set url parses"),
        )),
        Arc::new(localhost_trust()),
    ))
}
