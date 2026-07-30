use std::sync::Arc;

use url::Url;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn unreachable_endpoint() -> Arc<dyn ProvidesEndpoint> {
    let issuer_url = Url::parse("https://127.0.0.1:1").expect("the unreachable issuer url parses");

    Arc::new(StaticEndpoint::new(issuer_url))
}
