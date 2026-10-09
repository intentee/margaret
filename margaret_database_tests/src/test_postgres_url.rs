use std::env;

use url::Url;

const TEST_POSTGRES_URL_VARIABLE: &str = "MARGARET_TEST_POSTGRES_URL";

/// # Panics
///
/// Panics when the url of the shared test cluster is not set or is not a url.
#[must_use]
pub fn test_postgres_url() -> Url {
    env::var(TEST_POSTGRES_URL_VARIABLE)
        .expect("MARGARET_TEST_POSTGRES_URL is not set; run the tests via `make coverage`")
        .parse()
        .expect("MARGARET_TEST_POSTGRES_URL is a url")
}
