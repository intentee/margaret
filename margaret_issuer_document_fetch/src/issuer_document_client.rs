use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::RequestBuilder;
use reqwest::redirect::Policy;

use crate::issuer_document_fetch::IssuerDocumentFetch;
use crate::issuer_document_fetch_error::IssuerDocumentFetchError;
use crate::issuer_document_request::IssuerDocumentRequest;

async fn fetch_document(request: RequestBuilder) -> IssuerDocumentFetch {
    let response = match request.send().await {
        Ok(response) => response,
        Err(error) => return IssuerDocumentFetch::TransportFailed(error),
    };
    let status = response.status();

    if !status.is_success() {
        return IssuerDocumentFetch::UnexpectedStatus(status);
    }

    match response.bytes().await {
        Ok(document) => IssuerDocumentFetch::Fetched(document),
        Err(error) => IssuerDocumentFetch::TransportFailed(error),
    }
}

pub struct IssuerDocumentClient {
    http_client: Client,
}

impl IssuerDocumentClient {
    /// # Errors
    ///
    /// Returns `IssuerDocumentFetchError::ClientBuild` when the builder cannot produce a client.
    pub fn build(client_builder: ClientBuilder) -> Result<Self, IssuerDocumentFetchError> {
        let http_client = client_builder
            .use_native_tls()
            .https_only(true)
            .redirect(Policy::none())
            .build()
            .map_err(|source| IssuerDocumentFetchError::ClientBuild { source })?;

        Ok(Self { http_client })
    }

    pub async fn fetch(
        &self,
        IssuerDocumentRequest {
            cancellation_token,
            timeout,
            url,
        }: IssuerDocumentRequest<'_>,
    ) -> IssuerDocumentFetch {
        let request = self.http_client.get(url).timeout(timeout);

        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => IssuerDocumentFetch::Cancelled,
            fetched = fetch_document(request) => fetched,
        }
    }
}
