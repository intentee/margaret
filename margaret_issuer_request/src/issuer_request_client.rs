use std::future::Future;
use std::pin::Pin;

use oauth2::AsyncHttpClient;
use oauth2::HttpRequest;
use oauth2::HttpResponse;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::Request;
use reqwest::RequestBuilder;
use reqwest::redirect::Policy;
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::issuer_document::IssuerDocument;
use crate::issuer_exchange_error::IssuerExchangeError;
use crate::issuer_request_error::IssuerRequestError;
use crate::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use crate::read_response_body::read_response_body;
use crate::response_body::ResponseBody;

async fn received_document(request: RequestBuilder) -> IssuerDocument {
    let response = match request.send().await {
        Ok(response) => response,
        Err(error) => return IssuerDocument::TransportFailed(error),
    };
    let status = response.status();

    if !status.is_success() {
        return IssuerDocument::UnexpectedStatus(status);
    }

    match read_response_body(response).await {
        ResponseBody::Failed(error) => IssuerDocument::TransportFailed(error),
        ResponseBody::Oversized { max_bytes } => IssuerDocument::Oversized { max_bytes },
        ResponseBody::Read(document) => IssuerDocument::Fetched(document),
    }
}

pub struct IssuerRequestClient {
    http_client: Client,
}

impl IssuerRequestClient {
    /// # Errors
    ///
    /// Returns `IssuerRequestError::ClientBuild` when the builder cannot produce a client.
    pub fn build(client_builder: ClientBuilder) -> Result<Self, IssuerRequestError> {
        let http_client = client_builder
            .use_native_tls()
            .https_only(true)
            .redirect(Policy::none())
            .build()
            .map_err(|source| IssuerRequestError::ClientBuild { source })?;

        Ok(Self { http_client })
    }

    /// # Errors
    ///
    /// Returns `IssuerRequestError::ClientBuild` when the system TLS configuration cannot produce a
    /// client.
    pub fn create() -> Result<Self, IssuerRequestError> {
        Self::build(Client::builder())
    }

    /// # Errors
    ///
    /// Returns `IssuerExchangeError::Transport` when the request cannot be sent or its answer
    /// cannot be read, and `IssuerExchangeError::Oversized` when the answer exceeds the size cap.
    pub async fn exchange(
        &self,
        mut request: Request,
    ) -> Result<HttpResponse, IssuerExchangeError> {
        *request.timeout_mut() = Some(ISSUER_REQUEST_TIMEOUT);

        let response = match self.http_client.execute(request).await {
            Ok(response) => response,
            Err(source) => return Err(IssuerExchangeError::Transport(source)),
        };
        let status = response.status();
        let headers = response.headers().clone();

        match read_response_body(response).await {
            ResponseBody::Failed(source) => Err(IssuerExchangeError::Transport(source)),
            ResponseBody::Oversized { max_bytes } => {
                Err(IssuerExchangeError::Oversized { max_bytes })
            }
            ResponseBody::Read(body) => {
                let mut answer = HttpResponse::new(body.to_vec());

                *answer.status_mut() = status;
                *answer.headers_mut() = headers;

                Ok(answer)
            }
        }
    }

    pub async fn fetch_document(
        &self,
        url: Url,
        cancellation_token: &CancellationToken,
    ) -> IssuerDocument {
        let request = self.http_client.get(url).timeout(ISSUER_REQUEST_TIMEOUT);

        tokio::select! {
            biased;
            () = cancellation_token.cancelled() => IssuerDocument::Cancelled,
            received = received_document(request) => received,
        }
    }
}

impl<'client> AsyncHttpClient<'client> for IssuerRequestClient {
    type Error = IssuerExchangeError;
    type Future =
        Pin<Box<dyn Future<Output = Result<HttpResponse, IssuerExchangeError>> + Send + 'client>>;

    fn call(&'client self, request: HttpRequest) -> Self::Future {
        Box::pin(async move {
            match Request::try_from(request) {
                Ok(request) => self.exchange(request).await,
                Err(source) => Err(IssuerExchangeError::Transport(source)),
            }
        })
    }
}
