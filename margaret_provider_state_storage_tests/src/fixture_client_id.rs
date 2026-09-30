use margaret_oauth_vocabulary::client_id::ClientId;

/// # Panics
///
/// Panics when the fixture client identifier is rejected.
#[must_use]
pub fn fixture_client_id() -> ClientId {
    "client".parse().expect("the client identifier is visible")
}
