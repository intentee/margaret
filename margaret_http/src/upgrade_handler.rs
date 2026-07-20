use async_trait::async_trait;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio_util::sync::CancellationToken;

use crate::response::Response;

#[async_trait]
pub trait UpgradeHandler: Send + Sync {
    fn switching_response(&self) -> Response;

    async fn serve(
        self: Box<Self>,
        upgraded: TokioIo<Upgraded>,
        connection_token: CancellationToken,
    );
}
