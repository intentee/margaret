use std::str::FromStr as _;

use spiffe::spiffe_id::SpiffeId;

#[must_use]
pub fn peer() -> SpiffeId {
    SpiffeId::from_str("spiffe://example.org/identity-client")
        .expect("the fixture spiffe id is valid")
}
