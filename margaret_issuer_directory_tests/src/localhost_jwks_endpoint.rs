use crate::constants::LOCALHOST_JWKS_ENDPOINT_ISSUER;
use crate::polled_fixture::PolledFixture;

#[must_use]
pub fn localhost_jwks_endpoint() -> PolledFixture {
    PolledFixture::published(LOCALHOST_JWKS_ENDPOINT_ISSUER)
}
