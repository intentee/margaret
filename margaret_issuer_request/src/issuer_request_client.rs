use http::Response;
use reqwest::Client;
use reqwest::ClientBuilder;
use reqwest::IntoUrl;
use reqwest::Request;
use reqwest::RequestBuilder;
use reqwest::redirect::Policy;
use tokio_util::sync::CancellationToken;

use crate::issuer_answer::IssuerAnswer;
use crate::issuer_document::IssuerDocument;
use crate::issuer_exchange_error::IssuerExchangeError;
use crate::issuer_request_timeout::ISSUER_REQUEST_TIMEOUT;
use crate::read_response_body::read_response_body;
use crate::response_body::ResponseBody;

async fn received_document(request: RequestBuilder) -> IssuerDocument {
    let response = match request.send().await {
        Ok(response) => response,
        Err(source) => return IssuerDocument::Failed(IssuerExchangeError::Transport(source)),
    };
    let status = response.status();

    if !status.is_success() {
        return IssuerDocument::UnexpectedStatus(status);
    }

    match read_response_body(response).await {
        ResponseBody::Failed(source) => {
            IssuerDocument::Failed(IssuerExchangeError::Transport(source))
        }
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
    /// Returns `reqwest::Error` when the builder cannot produce a client.
    pub fn build(client_builder: ClientBuilder) -> Result<Self, reqwest::Error> {
        client_builder
            .use_native_tls()
            .https_only(true)
            .redirect(Policy::none())
            .build()
            .map(|http_client| Self { http_client })
    }

    /// # Errors
    ///
    /// Returns `reqwest::Error` when the system TLS configuration cannot produce a client.
    pub fn create() -> Result<Self, reqwest::Error> {
        Self::build(Client::builder())
    }

    #[must_use]
    pub fn preconfigured(http_client: Client) -> Self {
        Self { http_client }
    }

    /// # Errors
    ///
    /// Returns `IssuerExchangeError::Transport` when the request cannot be sent or its answer
    /// cannot be read.
    pub async fn exchange(
        &self,
        mut request: Request,
    ) -> Result<IssuerAnswer, IssuerExchangeError> {
        *request.timeout_mut() = Some(ISSUER_REQUEST_TIMEOUT);

        let response = match self.http_client.execute(request).await {
            Ok(response) => response,
            Err(source) => return Err(IssuerExchangeError::Transport(source)),
        };
        let status = response.status();
        let headers = response.headers().clone();

        match read_response_body(response).await {
            ResponseBody::Failed(source) => Err(IssuerExchangeError::Transport(source)),
            ResponseBody::Oversized { max_bytes } => Ok(IssuerAnswer::Oversized { max_bytes }),
            ResponseBody::Read(body) => {
                let mut answer = Response::new(body.to_vec());

                *answer.status_mut() = status;
                *answer.headers_mut() = headers;

                Ok(IssuerAnswer::Received(answer))
            }
        }
    }

    pub async fn fetch_document(
        &self,
        url: impl IntoUrl,
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
