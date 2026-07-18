use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::redirect::Policy;

use crate::jwks_client_error::JwksClientError;

pub(crate) fn build_jwks_http_client(
    client_builder: ClientBuilder,
) -> Result<Client, JwksClientError> {
    client_builder
        .https_only(true)
        .redirect(Policy::none())
        .build()
        .map_err(JwksClientError::HttpClientBuild)
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::build_jwks_http_client;

    #[test]
    fn surfaces_builder_errors() {
        let builder_with_invalid_tls_backend = Client::builder().use_preconfigured_tls(0u8);

        let error = build_jwks_http_client(builder_with_invalid_tls_backend)
            .expect_err("an unsupported tls backend cannot build a client");

        assert_eq!(
            error.to_string(),
            "the jwks http client could not be built: builder error"
        );
    }
}
