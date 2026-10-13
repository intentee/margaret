use url::Url;

/// # Panics
///
/// Panics when the declared URL does not parse.
#[must_use]
pub fn declared_path(declared_url: &str) -> String {
    Url::parse(declared_url)
        .expect("the declared URL parses")
        .path()
        .to_string()
}
