use std::future::Future;

use spiffe::spiffe_id::SpiffeId;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::request::Request;
use crate::response::Response;

pub async fn with_verified_peer<'request, TOutput, TContinuation, TFuture>(
    request: &'request Request,
    refused: fn(Response) -> TOutput,
    continuation: TContinuation,
) -> TOutput
where
    TContinuation: FnOnce(&'request SpiffeId) -> TFuture,
    TFuture: Future<Output = TOutput>,
{
    match request.peer_identity() {
        PeerIdentity::Verified { spiffe_id } => continuation(spiffe_id).await,
        PeerIdentity::Anonymous | PeerIdentity::Unidentified => refused(Response::forbidden()),
    }
}
