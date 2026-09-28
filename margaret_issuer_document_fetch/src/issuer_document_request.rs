use std::time::Duration;

use tokio_util::sync::CancellationToken;
use url::Url;

pub struct IssuerDocumentRequest<'request> {
    pub cancellation_token: &'request CancellationToken,
    pub timeout: Duration,
    pub url: Url,
}
