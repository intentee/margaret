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
    use super::build_reqwest_client;

    #[test]
    fn surfaces_builder_errors() {
        let builder_with_invalid_tls_backend =
            reqwest::Client::builder().use_preconfigured_tls(0u8);

        assert!(build_reqwest_client(builder_with_invalid_tls_backend).is_err());
    }
}
