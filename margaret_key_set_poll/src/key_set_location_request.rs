use std::time::Duration;

use tokio_util::sync::CancellationToken;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;

pub struct KeySetLocationRequest<'request> {
    pub cancellation_token: &'request CancellationToken,
    pub issuer_document_client: &'request IssuerDocumentClient,
    pub timeout: Duration,
}
