use url::Url;

/// # Panics
///
/// Panics when the path does not join onto the provider origin.
#[must_use]
pub fn provider_url(path: &str) -> Url {
    Url::parse("https://localhost")
        .and_then(|origin| origin.join(path))
        .expect("the provider path is a url")
}
