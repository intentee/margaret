use serde_json::Value;

use margaret_jws_verification::compact_jws::CompactJws;

/// # Panics
///
/// Panics when the payload of the jws is not json.
#[must_use]
pub fn jws_claims(jws: &CompactJws<'_>) -> Value {
    serde_json::from_slice(jws.payload()).expect("the payload is json")
}
