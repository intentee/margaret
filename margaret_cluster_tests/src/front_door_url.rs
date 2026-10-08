use url::Url;

/// # Panics
///
/// Panics when the port does not form a URL.
#[must_use]
pub fn front_door_url(port: u16) -> Url {
    Url::parse(&format!("https://localhost:{port}")).expect("the front door forms a URL")
}
