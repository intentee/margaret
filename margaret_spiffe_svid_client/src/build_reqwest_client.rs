use reqwest::Client;
use reqwest::ClientBuilder;

use crate::svid_error::SvidError;

pub(crate) fn build_reqwest_client(client_builder: ClientBuilder) -> Result<Client, SvidError> {
    client_builder
        .build()
        .map_err(|source| SvidError::ReqwestClient { source })
}

#[cfg(test)]
mod tests {
    use reqwest::Client;

    use super::build_reqwest_client;

    #[test]
    fn surfaces_a_client_build_failure_as_an_svid_error_instead_of_panicking() {
        let builder_with_invalid_tls_backend = Client::builder().use_preconfigured_tls(0u8);

        assert!(build_reqwest_client(builder_with_invalid_tls_backend).is_err());
    }
}
