use std::sync::Arc;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;

use crate::localhost_discovered_issuer::LOCALHOST_DISCOVERED_ISSUER;
use crate::polled_fixture::PolledFixture;

#[must_use]
pub fn localhost_oidc_issuer() -> PolledFixture {
    PolledFixture::discovered(
        LOCALHOST_DISCOVERED_ISSUER,
        Arc::new(IssuerMetadata::awaiting()),
    )
}
