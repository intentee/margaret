use margaret_oauth_vocabulary::client_secret::ClientSecret;

/// # Panics
///
/// Panics when the fixture client secret is rejected.
#[must_use]
pub fn fixture_client_secret() -> ClientSecret {
    "s3cret/+="
        .parse()
        .expect("the fixture client secret is visible")
}
