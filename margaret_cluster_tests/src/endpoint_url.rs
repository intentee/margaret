use url::Url;

use crate::declared_path::declared_path;

/// # Panics
///
/// Panics when the declared endpoint does not join the base URL.
#[must_use]
pub fn endpoint_url(base: &Url, declared_endpoint: &str) -> Url {
    base.join(&declared_path(declared_endpoint))
        .expect("the endpoint joins the base URL")
}
